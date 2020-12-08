int distanceVal;
int distancePrevVal;
String msg;
// PINS
const int ProximityTriggerPin = 7;
const int ProximityEchoPin = 6;

void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
  pinMode(ProximityTriggerPin, OUTPUT);
  pinMode(ProximityEchoPin, INPUT);
  Serial.begin (9600);
}

void loop() {

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
  if (distanceVal != distancePrevVal) {
    news = true;
    distancePrevVal = distanceVal;
    msg.concat("distance=");
    msg.concat(distanceVal);
    msg.concat("|");
  };
  if (news == true) {
    Serial.println(msg);
    digitalWrite(LED_BUILTIN, HIGH);
    delay(40);                   
    digitalWrite(LED_BUILTIN, LOW);  
    delay(60);            
  } else {
    Serial.println("SENSOR: -");
    digitalWrite(LED_BUILTIN, HIGH);
    delay(20);                   
    digitalWrite(LED_BUILTIN, LOW); 
    delay(50);    
    digitalWrite(LED_BUILTIN, HIGH);
    delay(20);                   
    digitalWrite(LED_BUILTIN, LOW); 
    delay(10);
  }
  //delay(100);
}
