// Button Sensor
const int ButtonPin = 9;
bool SwitchValue = false;
int buttonValue;


void setup() {
  pinMode (ButtonPin, INPUT) ;
  Serial.begin (9600);
}

void loop() {
  buttonValue = digitalRead(ButtonPin); // read the value of the button
  if(buttonValue == HIGH)
  {
    if(SwitchValue == false){
      SwitchValue = true;
    } else {
      SwitchValue = false;
    }
  }
  if(SwitchValue == true)
  {
    delay(50);    
    Serial.println("SENSOR: data_button_1");
  }
  else
  {
    delay(50);    
    Serial.println("SENSOR: data_button_0");
  }
  
  delay(100);
}
