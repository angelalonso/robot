// Button Sensor
const int ButtonPin = 8;
int buttonState = 1;


void setup() {
  pinMode (ButtonPin, INPUT) ;
  Serial.begin (9600);
}

void loop() {
  //buttonState = digitalRead(ButtonPin); // read the value of the button
  
  delay(50);    
  Serial.print("LOG: data_button_");
  Serial.println (buttonState);
  
  
  delay(100);
}
