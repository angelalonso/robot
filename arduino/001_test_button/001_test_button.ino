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
  
  delay(50);    
  Serial.print("SENSOR: data_button_");
  Serial.println (buttonValue, DEC);
  
  
  delay(100);
}
