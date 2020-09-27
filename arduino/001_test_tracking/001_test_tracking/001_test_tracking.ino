const int TrackerPin = 8;
int sensorValue = 0; 
void setup() {
  // put your setup code here, to run once
  pinMode (TrackerPin, INPUT);
  Serial.begin (9600);
}

void loop() {
  // put your main code here, to run repeatedly:
  sensorValue = analogRead (TrackerPin);
  if (sensorValue < 50 && sensorValue < 500) 
  {
    delay(50);    
    Serial.print("LOG: RESULT ON -> ");
    Serial.println (sensorValue, DEC);
  }
  else (sensorValue > 500&& sensorValue > 1023);
    {
    delay(50);    
    Serial.print("LOG: RESULT off -> ");
    Serial.println (sensorValue, DEC);
  }
}
