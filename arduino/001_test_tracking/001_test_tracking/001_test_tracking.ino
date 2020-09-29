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
    delay(50);
    Serial.println("ACTION: move_forwards_slow");
  }
  else
  {
    delay(50);    
    Serial.print("SENSOR: data_tracker_");
    Serial.println (sensorValue, DEC);
    delay(50);
    Serial.println("ACTION: move_stop");
  }
  delay(100);
}
