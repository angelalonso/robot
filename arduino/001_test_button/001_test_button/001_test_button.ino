// Button Sensor
const int ButtonPin = 9;
int buttonState = 0;


void setup() {
  pinMode (ButtonPin, INPUT) ;
  Serial.begin (9600);
}

void loop() {
  buttonState = digitalRead(ButtonPin); // read the value of the button
  
  delay(50);    
  Serial.print("SENSOR: data_button_");
  Serial.println (buttonState);
  
  
  delay(100);
}
