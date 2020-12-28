int incomingByte = 0;
String msg;
int distanceVal;
int distancePrevVal;
int buttonVal;
int buttonPrevVal;
// PINS
const int ProximityTriggerPin = 3;
const int ProximityEchoPin = 2;
const int ButtonPin = 7;


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
void loop() {
  if (Serial.available() > 0) {
    // read the incoming byte:
    incomingByte = Serial.read();
    
    msg = "SENSOR: ";
    bool news = false;
    
    // Needed "protocol" for the proximity sensor
    digitalWrite(ProximityTriggerPin, LOW); // Reset triggerPin
    delayMicroseconds(2);
    digitalWrite(ProximityTriggerPin, HIGH); // Sets triggerPin on HIGH state for 10 microsecs
    delayMicroseconds(10);
    digitalWrite(ProximityTriggerPin, LOW);
    long duration = pulseIn(ProximityEchoPin, HIGH);
    distanceVal = duration*0.034/2;
    //delay(30);

    if (distanceVal != distancePrevVal) {
      news = true;
      distancePrevVal = distanceVal;
      msg.concat("distance=");
      msg.concat(distanceVal);
      msg.concat("|");
    
    };
    
    buttonVal=digitalRead(ButtonPin); //read the value of the Button
    if (buttonVal != buttonPrevVal) {
      news = true;
      buttonPrevVal = buttonVal;
      msg.concat("button=");
      msg.concat(buttonVal);
      msg.concat("|");
    };
    
    if (news == true) {
      Serial.println(msg);
    }
    //delay(100);
  }
  
}
