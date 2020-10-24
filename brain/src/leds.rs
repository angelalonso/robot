use rust_gpiozero::LED;

pub struct LEDs {
    led_y: Option<LED>,
    led_r: Option<LED>,
    led_g: Option<LED>,
    led_b: Option<LED>,
    led_y_on: bool,
    led_r_on: bool,
    led_g_on: bool,
    led_b_on: bool,
}

impl LEDs {
    pub fn new(mode: String) -> Result<Self, &'static str> {
        //TODO define LEDs based on mode, (if on laptop, set the to None)
        let l_y: std::option::Option<LED>;
        if mode == "classic" {
            l_y = Some(LED::new(21));
        } else {
            l_y = None;
        }
        Ok(Self {
            led_y: l_y,
            led_r: None,
            led_g: None,
            led_b: None,
            led_y_on: false,
            led_r_on: false,
            led_g_on: false,
            led_b_on: false,
        })
    }

    pub fn set_led_y(&mut self, new_state: bool) {
        match &self.led_y {
            Some(led_y) => {
                if new_state {
                    if ! self.led_y_on {
                        self.led_y_on = true;
                        led_y.on();
                    }
                } else {
                    if self.led_y_on {
                        self.led_y_on = false;
                        led_y.off();
                    }
                }
            }            
            None => println!("This would have done somthing with LED Y, if we had one")
        }
    }

    pub fn set_led_r(&mut self, new_state: bool) {
        match &self.led_r {
            Some(led_r) => {
                if new_state {
                    if ! self.led_r_on {
                        self.led_r_on = true;
                        led_r.on();
                    }
                } else {
                    if self.led_r_on {
                        self.led_r_on = false;
                        led_r.off();
                    }
                }
            }            
            None => println!("This would have done somthing with LED R, if we had one")
        }
    }

    pub fn set_led_g(&mut self, new_state: bool) {
        match &self.led_g {
            Some(led_g) => {
                if new_state {
                    if ! self.led_g_on {
                        self.led_g_on = true;
                        led_g.on();
                    }
                } else {
                    if self.led_g_on {
                        self.led_g_on = false;
                        led_g.off();
                    }
                }
            }            
            None => println!("This would have done somthing with LED G, if we had one")
        }
    }

    pub fn set_led_b(&mut self, new_state: bool) {
        match &self.led_b {
            Some(led_b) => {
                if new_state {
                    if ! self.led_b_on {
                        self.led_b_on = true;
                        led_b.on();
                    }
                } else {
                    if self.led_b_on {
                        self.led_b_on = false;
                        led_b.off();
                    }
                }
            }            
            None => println!("This would have done somthing with LED B, if we had one")
        }
    }
}
