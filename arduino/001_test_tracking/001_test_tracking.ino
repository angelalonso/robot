int Sensor = 3; // sensor input pin
 
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
    Serial.println("LOG: RESULT ON");
  }
  else
  {
    delay(50);    
    Serial.println("LOG: RESULT off");
  }
  delay(500); // 
}
