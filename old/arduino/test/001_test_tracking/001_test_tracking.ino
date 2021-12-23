const int TrackerPin = 2;
void setup() {
  pinMode (TrackerPin, INPUT);
  Serial.begin (9600);
}

void loop() {
  boolean sensorValue = digitalRead(TrackerPin); // read the value of tracking module
  if(sensorValue == HIGH) //if it is HiGH
  { 
    delay(50);    
    Serial.print("SENSOR: data_tracker_");
    Serial.println (sensorValue, DEC);
  }
  else
  {
    delay(50);    
    Serial.print("SENSOR: data_tracker_");
    Serial.println (sensorValue, DEC);
  }
  delay(100);
}
