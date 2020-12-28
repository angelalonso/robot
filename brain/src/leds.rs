use rust_gpiozero::LED;
use std::sync::Arc;
use std::sync::Mutex;
use log::debug;
use std::collections::HashMap;

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
    pub fn new(mode: String, led_map: HashMap<String, HashMap<String, String>>) -> Result<Self, &'static str> {
        let mut objs = [].to_vec();
        for o in led_map {
            let mut l_o = None;
            if mode != "dryrun" {
                let gpio = o.1["gpio"].parse::<u8>().unwrap();
                l_o = Some(Arc::new(Mutex::new(LED::new(gpio))));
            }
            let l = LEDObj {
                name: o.0.to_string(),
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
        if let Some(l) = self.objects.iter_mut().find(|x| *x.name == *led_id) {
            match &l.object {
                Some(o) => {
                    if new_state {
                        if ! l.on {
                            l.on = true;
                            o.lock().unwrap().on();
                        }
                    } else if l.on {
                        l.on = false;
                        o.lock().unwrap().off();
                    }
                },            
                None => debug!("- Mocked - Setting -{:?}- to LED Y", new_state),
            }
        }
    }
}
