const int TrackerPin = 2;
int sensorValue = 0; 
void setup() {
  pinMode (TrackerPin, INPUT);
  Serial.begin (9600);
}

void loop() {
  boolean val = digitalRead(TrackerPin); // read the value of tracking module
  if(val == HIGH) //if it is HiGH
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
