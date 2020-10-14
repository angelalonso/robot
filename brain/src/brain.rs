// This is the actual central block of our robot
// It is called Brain for obvious reasons, but is divided on two:
//  - Cerebellum part: Manages any movement actions
//  - Brain part: Manages any other actions, such as:
//    - installing a new .hex into arduino
use crate::arduino::Arduino;
use crate::config::Config;
use crate::cerebellum::Cerebellum;
use crate::log;
use crate::mover::Mover;
use std::fs::File;
use std::process;
use std::str;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrainDeadError {
    /// This is just the most basic I dont care Error
    #[error("Source contains no data")]
    EmptyError,

    #[error("Config contains no related entries")]
    NoConfigFound,

    #[error("Something went wrong while working with timestamps")]
    SystemTimeError,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MetricEntry {
    pub time: f64,
    pub motor_l: i16,
    pub motor_r: i16,
    pub tracker: bool,
    pub distance: u16,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MoveAction {
    motor_l: i16,
    motor_r: i16,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RuleInput {
    time: String,
    motor_l: String,
    motor_r: String,
    tracker: String,
    distance: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RuleEntry {
    input: Vec<RuleInput>,
    action: MoveAction,
}

#[derive(Clone)]
pub struct Brain<'a> {
    pub name: &'a str,
    pub starttime: u128,
    pub config: Config,
    pub cerebellum: Cerebellum,
    pub serialport: &'a str,
    pub timeout: u64,
    pub metrics: Vec<MetricEntry>,
    pub arduino: Arduino<'a>,
    pub mover: Mover<'a>,
}

/// Initialize all the things
impl Brain<'static> {
    pub fn new(brain_name: &'static str, config_file: String, cerebellum_config_file: String, raw_serial_port: Option<&'static str>) -> Result<Self, &'static str> {
        // This will be used to calculate our timestamps
        let st = SystemTime::now();
        let start_time = match st.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => 0,
        };
        // This loads the rules dictating what actions the brain takes
        // TODO: we should definitely do the same for the Cerebellum rules
        let cfg = Config::new(config_file);
        let crb = Cerebellum::new(cerebellum_config_file);
        // Serial Port to communicate with Arduino
        let sp = match raw_serial_port {
            Some(port) => port,
            None => "/dev/ttyUSB0",
        };
        // Vector of latest metrics received from the Arduino
        let mtr: Vec<MetricEntry> = [].to_vec();
        // Instance of the Arduino Module
        let a = Arduino::new("arduino", None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Arduino: {}", err);
            process::exit(1);
        });
        // Instance of the Mover module
        let m = Mover::new().unwrap_or_else(|err| {
            eprintln!("Problem Initializing Mover: {}", err);
            process::exit(1);
        });
        Ok(Self {
            name: brain_name,
            starttime: start_time,
            config: cfg,
            cerebellum: crb,
            serialport: sp,
            timeout: 4,
            metrics: mtr,
            arduino: a,
            mover: m,
        })
    }

    pub fn get_input(mut self) {
        // TODO: reorder and simplify:
        // - Get input from Arduino
        // - Call "do_brain_stuff"
        //   - dissect_msg
        //   - build_actions_from_msg
        //   - do_crbllum_actions
        // - Call "do_crbllum_stuff"
        //   - build_metrics
        //   - build_actions_from_metrics
        //   - do_crbllum_actions
        loop {
            let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
            let msgs = s.clone();
            let mut arduino = self.arduino.clone();
            let mut brain_clone = self.clone();
            let brain_name = self.name.clone();
            let _handle = thread::spawn(move || {
                let _received = match arduino.read_channel(msgs){
                    Ok(rcv) => {
                        match brain_clone.get_brain_actions(&rcv){
                            Ok(acts) => println!("Taking action {:?}", acts.join(", ")),
                            Err(_) => log(Some(&brain_name), "D", "No actions were found for trigger"),
                        };
                    },
                    Err(_) => log(Some(&brain_name), "D", "Nothing read from Channel"),
                };
            });
            // TODO: move this to cerebellum
            let mut _current_metric = MetricEntry {
                time: 0.0,
                motor_l: 0,
                motor_r: 0,
                tracker: false,
                distance: 0,
            };
            loop {
                let msg = r.recv();
                let mut msg_actions = Vec::new();
                let mut msg_sensors = String::new();
                let actionmsg = msg.clone();
                let sensormsg = msg.clone();
                if actionmsg.unwrap().split(": ").collect::<Vec<_>>()[0] == "ACTION".to_string() {
                    msg_actions.push(msg.unwrap().replace("ACTION: ", ""));
                } else if sensormsg.unwrap().split(": ").collect::<Vec<_>>()[0] == "SENSOR".to_string() {
                    msg_sensors = msg.unwrap().replace("SENSOR: ", "");
                }
                self.do_brain_actions(msg_actions).unwrap();
                // TODO: move this to cerebellum
                let prev_metric = self.cerebellum.current_metric.clone();
                // TODO: move this call itself to cerebellum
                self.cerebellum.current_metric = self.build_crbllum_input(msg_sensors, prev_metric).unwrap();
                // TODO: remove this maybe?
                //self.get_crbllum_input(&current_metric);
                // TODO: move this call itself to cerebellum
                self.cerebellum.get_input();
                // TODO: move this call itself to cerebellum
                //let crbllum_action = self.cerebellum.do_actions(&current_metric).unwrap();
                let crbllum_action = self.cerebellum.do_actions().unwrap();
                // TODO: move this call itself to cerebellum
                if crbllum_action.len() > 0 {
                    self.mover.set_move(format!("{:?}_{:?}", crbllum_action[0].action.motor_l, crbllum_action[0].action.motor_r));
                }
            }
        }
    }
    ///------------------------------------------------------///
    ///  Brain
    ///------------------------------------------------------///
    /// Get the action that relates to the trigger received and call to apply it
    /// Hm...maybe this one and do_brain_actions should go together?
    pub fn get_brain_actions(&mut self, trigger: &str) -> Result<Vec<String>, BrainDeadError> {
        log(Some(&self.name), "D", &format!("Received {}", trigger));
        let actions = self.config.get_actions(&trigger);
        match actions {
            Ok(acts) => {
                match acts {
                    Some(a) => {
                        self.do_brain_actions(a.clone()).unwrap();
                        Ok(a)
                    },
                    None => {
                        log(Some(&self.name), "D", "Nothing to do");
                        Err(BrainDeadError::NoConfigFound)
                    },
                }
            },
            Err(_e) => Err(BrainDeadError::NoConfigFound),
        }
    }

    /// Call the action needed
    /// , which, right now, is just installing a new hex into the arduino
    pub fn do_brain_actions(&mut self, actions: Vec<String>) -> Result<(), BrainDeadError> {
        for action in &actions {
            let action_vec: Vec<&str> = action.split('_').collect();
            match action_vec[0] {
                "install" => self.arduino.install(&action_vec[1..].to_vec().join("_")).unwrap(),
                "move" => self.mover.set_move(action_vec[1..].to_vec().join("_")),
                _ => log(Some(&self.name), "D", "Relaxing here..."),
            };
        }
        Ok(())
    }


    ///------------------------------------------------------///
    ///  Cerebellum
    ///------------------------------------------------------///
    pub fn get_crbllum_config(&mut self) -> Result<Vec<RuleEntry>, BrainDeadError> {
        let filepointer = File::open("move_cfg.yaml").unwrap();
        let rules: Vec<RuleEntry> = serde_yaml::from_reader(filepointer).unwrap();
        Ok(rules)
    }

    pub fn build_crbllum_input(&mut self, sensors: String, prev_metric: MetricEntry) -> Result<MetricEntry, BrainDeadError> {
        log(Some(&self.name), "I", &format!("Moving : {}", self.mover.movement));
        let m_l: i16;
        let m_r: i16;
        if self.mover.movement == "forwards" {
            m_l = 100;
            m_r = 100;
        } else if self.mover.movement == "forwards_slow" {
            m_l = 55;
            m_r = 55;
        } else if self.mover.movement == "backwards" {
            m_l = -100;
            m_r = -100;
        } else if self.mover.movement == "rotate_left" {
            m_l = -70;
            m_r = 70;
        } else if self.mover.movement == "rotate_right" {
            m_l = 70;
            m_r = -70;
        } else {
            let motor_values: Vec<i16> = match self.mover.movement.split("_")
                .map(|s| s.parse())
                .collect() {
                    Ok(v) => v,
                    Err(_e) => [0,0].to_vec(),
                };
            m_l = motor_values[0];
            m_r = motor_values[1];
        };
        let ct = SystemTime::now();
        let current_time = match ct.duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(_e) => return Err(BrainDeadError::SystemTimeError),
        };
        let diff_time: f64 = (current_time as f64 - self.starttime as f64) as f64 / 100 as f64;
        let (trckr_msg, dist_msg) = self.get_values_from_sensor_msg(sensors, prev_metric);
        let m = MetricEntry {
            time: diff_time,
            motor_l: m_l,
            motor_r: m_r,
            tracker: trckr_msg,
            distance: dist_msg,
        };
        Ok(m)
    }

    pub fn get_values_from_sensor_msg(&mut self, sensor_msg: String, prev_metric: MetricEntry) -> (bool, u16) {
        let split_msg = sensor_msg.split("_").collect::<Vec<_>>();
        let mut trck: bool = prev_metric.tracker;
        let mut dist: u16 = prev_metric.distance;
        if split_msg[1] == "tracker" {
            let trck_int: u8 = split_msg[2].parse().unwrap();
            trck = trck_int != 0;
        } else if split_msg[1] == "distance" {
            dist = split_msg[2].parse().unwrap();
            println!("MESSAGE IS ->{}<-", sensor_msg);
        }
        (trck, dist)
    }

    pub fn get_crbllum_input<'a>(&mut self, metric: &'a MetricEntry) {
        if self.metrics.len() == 0 {
            self.metrics.push(metric.clone());
            
        } else {
            if metric.motor_l == self.metrics[0].motor_l &&
               metric.motor_r == self.metrics[0].motor_r &&
               metric.tracker == self.metrics[0].tracker &&
               metric.distance == self.metrics[0].distance
            {
                self.metrics[0].time += 0.1;
            } else {
                self.metrics.push(metric.clone());
                self.metrics.rotate_right(1);
                self.metrics[0].time = 0.1;
            }
            if self.metrics.len() > 5{
                self.metrics.pop();
            }
        }
    }
}
