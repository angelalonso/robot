// Button Sensor
//const int ButtonPin = 13;
//int buttonVal;
//int buttonPrevVal;
int distanceVal;
int distancePrevVal;
String msg;
//// Tracker Sensor disabled for now
//const int TrackerPin = 2;
// Proximity Sensor
const int ProximityTriggerPin = 7;
const int ProximityEchoPin = 6;

void setup() {
  //pinMode (ButtonPin, INPUT) ;
  //pinMode (TrackerPin, INPUT);
  pinMode(ProximityTriggerPin, OUTPUT);
  pinMode(ProximityEchoPin, INPUT);
  Serial.begin (9600);
}

void loop() {
  //buttonVal = digitalRead(ButtonPin); // read the value of the button
  //if(buttonValue == HIGH)
  //{
  //  delay(20);    
  //  Serial.println("SENSOR: button=1");
  //}
  //else
  //{
  //  delay(20);    
  //  Serial.println("SENSOR: button=0");
  //}
  //boolean trackerValue = digitalRead(TrackerPin); // read the value of tracking module
  //if(trackerValue == HIGH) //if it is HiGH
  //{ 
  //  delay(50);    
  //  Serial.print("SENSOR: tracker=");
  //  Serial.println (trackerValue, DEC);
  //}
  //else
  //{
  //  delay(50);    
  //  Serial.print("SENSOR: tracker=");
  //  Serial.println (trackerValue, DEC);
  //}

  // Needed "protocol" for the proximity sensor
  digitalWrite(ProximityTriggerPin, LOW); // Reset triggerPin
  delayMicroseconds(2);
  digitalWrite(ProximityTriggerPin, HIGH); // Sets triggerPin on HIGH state for 10 microsecs
  delayMicroseconds(10);
  digitalWrite(ProximityTriggerPin, LOW);
  long duration = pulseIn(ProximityEchoPin, HIGH);
  distanceVal = duration*0.034/2;
  delay(30);
  msg = "SENSOR: ";
  bool news = false;
//  if (buttonVal != buttonPrevVal) {
//    news = true;
//    buttonPrevVal = buttonVal;
//    if(buttonVal == HIGH) {
//      msg.concat("button=1|");
//    } else {
//      msg.concat("button=1|");
//    };
//  };
  if (distanceVal != distancePrevVal) {
    news = true;
    distancePrevVal = distanceVal;
    msg.concat("distance=");
    msg.concat(distanceVal);
    msg.concat("|");
  };
  if (news == true) {
    Serial.println(msg);
  } else {
    Serial.println("SENSOR: -");
  }
  
  //Serial.print("SENSOR: distance=");
  //Serial.println (distanceVal, DEC);
  
  delay(100);
}
