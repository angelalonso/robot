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
    objects: Vec<LEDObj>,
}

impl LEDs {
    pub fn new(mode: String, led_vector: Vec<String>) -> Result<Self, &'static str> {
        let mut objs = [].to_vec();
        for o in led_vector {
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
        Ok(Self {
            objects: objs,
        })
    }

    pub fn set_led(&mut self, led_id: String, new_state: bool) {
        match self.objects.iter_mut().find(|x| *x.name == *led_id) {
            Some(l) => {
                match &l.object {
                    Some(o) => {
                        println!("SWITCHING");
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
}
