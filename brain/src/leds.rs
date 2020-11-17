use rust_gpiozero::LED;
use std::sync::Arc;
use std::sync::Mutex;
use log::{debug, info};

#[derive(Clone, Debug)]
pub struct LEDObj {
    name: String,
    object: Option<Arc<Mutex<LED>>>,
    on: bool,
}
#[derive(Clone)]
pub struct LEDs {
    // TODO: turn this into vector and dynamically create LEDS based on setup
    objects: Vec<LEDObj>,
    led_y: LEDObj,
    led_r: LEDObj,
    led_g: LEDObj,
    led_b: LEDObj,
}

impl LEDs {
    pub fn new(mode: String, led_vector: Vec<String>) -> Result<Self, &'static str> {
        let mut objs = [].to_vec();
        for o in led_vector {
            println!("LED   - {}", o);
            let config = o.split("__").collect::<Vec<_>>();
            let keyval = config[1].split("=").collect::<Vec<_>>();
            let mut l_o = None;
            if mode != "dryrun" {
                l_o = Some(Arc::new(Mutex::new(LED::new(keyval[1].parse::<u8>().unwrap()))));
            }
            let l = LEDObj {
                name: config[0].to_string(),
                object: l_o,
                on: false,
            };
            objs.push(l);
        }
        let mut l_y_o = None;
        if mode != "dryrun" {
            l_y_o = Some(Arc::new(Mutex::new(LED::new(21))));
        }
        let l_y = LEDObj {
            name: "led_y".to_string(),
            object: l_y_o,
            on: false,
        };
        let mut l_r_o = None;
        if mode != "dryrun" {
            l_r_o = Some(Arc::new(Mutex::new(LED::new(20))));
        }
        let l_r = LEDObj {
            name: "led_r".to_string(),
            object: l_r_o,
            on: false,
        };
        let mut l_g_o = None;
        if mode != "dryrun" {
            l_g_o = Some(Arc::new(Mutex::new(LED::new(16))));
        }
        let l_g = LEDObj {
            name: "led_g".to_string(),
            object: l_g_o,
            on: false,
        };
        let mut l_b_o = None;
        if mode != "dryrun" {
            l_b_o = Some(Arc::new(Mutex::new(LED::new(12))));
        }
        let l_b = LEDObj {
            name: "led_b".to_string(),
            object: l_b_o,
            on: false,
        };
        Ok(Self {
            objects: objs,
            led_y: l_y,
            led_r: l_r,
            led_g: l_g,
            led_b: l_b,
        })
    }

    pub fn set_led(&mut self, led_id: String, new_state: bool) {
       // match led_id.as_str() {
       //     "led_y" => self.set_led_y(new_state),
       //     "led_r" => self.set_led_r(new_state),
       //     "led_g" => self.set_led_g(new_state),
       //     "led_b" => self.set_led_b(new_state),
       //     _ => (),
       // }
        match self.objects.iter_mut().find(|x| *x.name == *led_id) {
            Some(l) => {
                match &l.object {
                    Some(o) => {
                        if new_state {
                            if ! l.on {
                                l.on = true;
                                o.lock().unwrap().on();
                            }
                        } else {
                            if l.on {
                                l.on = false;
                                o.lock().unwrap().off();
                            }
                        }
                    }            
                    None => info!("- Mocked - Setting -{:?}- to LED Y", new_state)
                }
            },
            None => (),
        }
    }

    pub fn set_led_y(&mut self, new_state: bool) {
        match &self.led_y.object {
            Some(o) => {
                if new_state {
                    if ! self.led_y.on {
                        self.led_y.on = true;
                        o.lock().unwrap().on();
                    }
                } else {
                    if self.led_y.on {
                        self.led_y.on = false;
                        o.lock().unwrap().off();
                    }
                }
            }            
            None => debug!("- Mocked - Setting -{:?}- to LED Y", new_state)
        }
    }

    pub fn set_led_r(&mut self, new_state: bool) {
        match &self.led_r.object {
            Some(o) => {
                if new_state {
                    if ! self.led_r.on {
                        self.led_r.on = true;
                        o.lock().unwrap().on();
                    }
                } else {
                    if self.led_r.on {
                        self.led_r.on = false;
                        o.lock().unwrap().off();
                    }
                }
            }            
            None => debug!("- Mocked - Setting -{:?}- to LED Y", new_state)
        }
    }

    pub fn set_led_g(&mut self, new_state: bool) {
        match &self.led_g.object {
            Some(o) => {
                if new_state {
                    if ! self.led_g.on {
                        self.led_g.on = true;
                        o.lock().unwrap().on();
                    }
                } else {
                    if self.led_g.on {
                        self.led_g.on = false;
                        o.lock().unwrap().off();
                    }
                }
            }            
            None => debug!("- Mocked - Setting -{:?}- to LED Y", new_state)
        }
    }

    pub fn set_led_b(&mut self, new_state: bool) {
        match &self.led_b.object {
            Some(o) => {
                if new_state {
                    if ! self.led_b.on {
                        self.led_b.on = true;
                        o.lock().unwrap().on();
                    }
                } else {
                    if self.led_b.on {
                        self.led_b.on = false;
                        o.lock().unwrap().off();
                    }
                }
            }            
            None => debug!("- Mocked - Setting -{:?}- to LED Y", new_state)
        }
    }

}
