// Button Sensor
const int ButtonPin = 9;
bool SwitchValue = false;
int buttonValue;
// Tracker Sensor
const int TrackerPin = 2;
// Proximity Sensor
const int ProximityTriggerPin = 7;
const int ProximityEchoPin = 6;

void setup() {
  pinMode (ButtonPin, INPUT) ;
  pinMode (TrackerPin, INPUT);
  pinMode(ProximityTriggerPin, OUTPUT);
  pinMode(ProximityEchoPin, INPUT);
  Serial.begin (9600);
}

void loop() {
  buttonValue = digitalRead(ButtonPin); // read the value of the button
  if(buttonValue == HIGH)
  {
    if(SwitchValue == false){
      SwitchValue = true;
    } else {
      SwitchValue = false;
    }
  }
  if(SwitchValue == true)
  {
    delay(50);    
    Serial.println("SENSOR: data_button_1");
  }
  else
  {
    delay(50);    
    Serial.println("SENSOR: data_button_0");
  }
  boolean trackerValue = digitalRead(TrackerPin); // read the value of tracking module
  if(trackerValue == HIGH) //if it is HiGH
  { 
    delay(50);    
    Serial.print("SENSOR: data_tracker_");
    Serial.println (trackerValue, DEC);
  }
  else
  {
    delay(50);    
    Serial.print("SENSOR: data_tracker_");
    Serial.println (trackerValue, DEC);
  }

  // Needed "protocol" for the proximity sensor
  digitalWrite(ProximityTriggerPin, LOW); // Reset triggerPin
  delayMicroseconds(2);
  digitalWrite(ProximityTriggerPin, HIGH); // Sets triggerPin on HIGH state for 10 microsecs
  delayMicroseconds(10);
  digitalWrite(ProximityTriggerPin, LOW);
  long duration = pulseIn(ProximityEchoPin, HIGH);
  int distanceVALUE = duration*0.034/2;
  delay(50);
  Serial.print("SENSOR: data_distance_");
  Serial.println (distanceVALUE, DEC);
  
  delay(100);
}
