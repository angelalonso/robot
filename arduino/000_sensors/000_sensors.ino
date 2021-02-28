// PINS
const int ProximityTriggerPin = 3;
const int ProximityEchoPin = 2;
const int ButtonPin = 7;
// OTHER VARS
int incomingByte = 0;
String msg;
bool news = false;
int distanceVal;
int distancePrevVal;
int buttonVal;
int buttonPrevVal;

void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
  pinMode(ProximityTriggerPin, OUTPUT);
  pinMode(ProximityEchoPin, INPUT);
  pinMode(ButtonPin, INPUT);
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
  if (distanceVal != distancePrevVal) {
    news = true;
    distancePrevVal = distanceVal;
    msg.concat("distance=");
    msg.concat(distanceVal);
    msg.concat("|");
  };
  return msg;
}

String getButton(String msg) {
  buttonVal=digitalRead(ButtonPin); //read the value of the Button
  if (buttonVal != buttonPrevVal) {
    news = true;
    buttonPrevVal = buttonVal;
    msg.concat("button=");
    msg.concat(buttonVal);
    msg.concat("|");
  };
  return msg;
}

void loop() {
  if (Serial.available() > 0) {
    // read the incoming byte:
    incomingByte = Serial.read();
    Serial.println("received something");
    
    msg = "SENSOR: ";
    news = false;

    // DISTANCE SENSOR
    msg = getDistance(msg);

    // BUTTON SENSOR
    msg = getButton(msg);


    // SEND MESSAGE OR NOT
    if (news == true) {
      Serial.println(msg);
    }
  }
}
