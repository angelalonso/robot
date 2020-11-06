use rust_gpiozero::LED;
use std::sync::Arc;
use std::sync::Mutex;
use log::debug;

#[derive(Clone)]
pub struct LEDObj {
    name: String,
    object: Option<Arc<Mutex<LED>>>,
    on: bool,
}
#[derive(Clone)]
pub struct LEDs {
    led_y: LEDObj,
    led_r: LEDObj,
    led_g: LEDObj,
    led_b: LEDObj,
}

impl LEDs {
    pub fn new(mode: String) -> Result<Self, &'static str> {
        let mut l_y_o = None;
        if mode != "dryrun" {
            l_y_o = Some(Arc::new(Mutex::new(LED::new(21))));
            println!("IN");
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
        let l_g_o = None;
        //let mut l_g_o = None;
        //if mode != "dryrun" {
        //    l_g_o = Some(Arc::new(Mutex::new(LED::new(20))));
        //}
        let l_g = LEDObj {
            name: "led_g".to_string(),
            object: l_g_o,
            on: false,
        };
        let l_b_o = None;
        //let mut l_b_o = None;
        //if mode != "dryrun" {
        //    l_b_o = Some(Arc::new(Mutex::new(LED::new(16))));
        //}
        let l_b = LEDObj {
            name: "led_b".to_string(),
            object: l_b_o,
            on: false,
        };
        Ok(Self {
            led_y: l_y,
            led_r: l_r,
            led_g: l_g,
            led_b: l_b,
        })
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
