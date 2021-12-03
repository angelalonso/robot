// PINS
const int ProximityTriggerPin = 3;
const int ProximityEchoPin = 2;
// OTHER VARS
int incomingByte = 0;
String msg;
bool sync = false;
int distanceVal;
int distancePrevVal;

void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
  pinMode(ProximityTriggerPin, OUTPUT);
  pinMode(ProximityEchoPin, INPUT);
  Serial.begin(9600);
  
}

void blink1() {
    digitalWrite(LED_BUILTIN, HIGH);
    delay(40);                   
    digitalWrite(LED_BUILTIN, LOW);  
    delay(60);
}

void blink2() {
    digitalWrite(LED_BUILTIN, HIGH);
    delay(20);                   
    digitalWrite(LED_BUILTIN, LOW); 
    delay(50);    
    digitalWrite(LED_BUILTIN, HIGH);
    delay(20);                   
    digitalWrite(LED_BUILTIN, LOW); 
    delay(10);
}

String getDistance(String msg) {
  // Needed "protocol" for the proximity sensor
  digitalWrite(ProximityTriggerPin, LOW); // Reset triggerPin
  delayMicroseconds(2);
  digitalWrite(ProximityTriggerPin, HIGH); // Sets triggerPin on HIGH state for 10 microsecs
  delayMicroseconds(10);
  digitalWrite(ProximityTriggerPin, LOW);
  long duration = pulseIn(ProximityEchoPin, HIGH);
  distanceVal = duration*0.034/2;
  // This one is here because sensor throws 1000+ value when too close
  if ((distanceVal - distancePrevVal) > 500 ) {
    distanceVal = 0;
  }

  distancePrevVal = distanceVal;
  msg.concat("distance=");
  msg.concat(distanceVal);
  msg.concat("|");

  return msg;
}


void loop() {
  msg = "SENSOR: ";
  delay(30);
  if (Serial.available() > 0) {
    // read the incoming byte:
    incomingByte = Serial.read();
    
    //Serial.println("LOG: Synced");
    
    sync = true;
  }

  delay(30);
  if (sync == true) {
    // DISTANCE SENSOR
    msg = getDistance(msg);

    Serial.println(msg);
  }
  //this is only here to improve sync between arduino and raspberry, avoid unnecessary reads per call, reads that are not complete...and so on.
  //  Adapt this delay and the one at ARDUINO_READ_DELAY on .env if you need to improve synchronisation.
  //    (my Hardware may be different than yours and these values may not work so well for you)
  delay(100);
}
