const int TrackerPin = A5;
int sensorValue = 0; 
void setup() {
  // put your setup code here, to run once
  pinMode (TrackerPin, INPUT);
  Serial.begin (9600);
}

void loop() {
  // put your main code here, to run repeatedly:
  Serial.print("LOG: test");
  sensorValue = analogRead (TrackerPin);
  if (sensorValue < 50 && sensorValue < 500) 
  {
    delay(50);    
    Serial.print("LOG: RESULT ON -> ");
    Serial.println (sensorValue, DEC);
    delay(50);
    Serial.println("ACTION: move_forwards_slow");
  }
  else (sensorValue > 500&& sensorValue > 1023);
    {
    delay(50);    
    Serial.print("LOG: RESULT off -> ");
    Serial.println (sensorValue, DEC);
    delay(50);
    Serial.println("ACTION: move_stop");
  }
  delay(500);
}
