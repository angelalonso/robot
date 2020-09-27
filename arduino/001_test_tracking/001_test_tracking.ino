int Sensor = 2; // sensor input pin
 
void setup ()
{
  Serial.begin(9600); // Initialize serial output
  pinMode (Sensor, INPUT) ;
}
 
 
void loop ()
{
  bool val = digitalRead (Sensor) ; // The current signal of the sensor will be read
 
  if (val == HIGH)
  {
    delay(50);    
    Serial.println("LOG: RESULT B");
  }
  else
  {
    delay(50);    
    Serial.println("LOG: RESULT W");
  }
  delay(250); // 
}
