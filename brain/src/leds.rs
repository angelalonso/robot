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
    //led_r: LEDObj,
    //led_g: LEDObj,
    //led_b: LEDObj,
}

impl LEDs {
    pub fn new(mode: String) -> Result<Self, &'static str> {
        //TODO define LEDs based on mode, (if on laptop, set the to None)
        let mut l_y_o = None;
        if mode != "dryrun" {
            l_y_o = Some(Arc::new(Mutex::new(LED::new(21))));
            //l_y = Some(LED::new(21));
        }
        let l_y = LEDObj {
            name: "led_y".to_string(),
            object: l_y_o,
            on: false,
        };
        Ok(Self {
            led_y: l_y,
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
            None => debug!("This would have set -{:?}- to LED Y, if we had one", new_state)
        }
    }

}
