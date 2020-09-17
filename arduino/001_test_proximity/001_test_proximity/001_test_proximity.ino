const int proxTriggerPin = 7;
const int proxEchoPin = 6;
// vars
long duration;
int distance;

void setup() {
  pinMode(proxTriggerPin, OUTPUT); // Sets the trigPin as an Output
  pinMode(proxEchoPin, INPUT); // Sets the echoPin as an Input
  Serial.begin(9600); // Starts the serial communication

}

void loop() {

  // Clears the proxTriggerPin
  digitalWrite(proxTriggerPin, LOW);
  delayMicroseconds(2);
  // Sets the proxTriggerPin on HIGH state for 10 micro seconds
  digitalWrite(proxTriggerPin, HIGH);
  delayMicroseconds(10);
  digitalWrite(proxTriggerPin, LOW);
  // Reads the proxEchoPin, returns the sound wave travel time in microseconds
  duration = pulseIn(proxEchoPin, HIGH);
  distance = duration*0.034/2;
  delay(50);    
  Serial.print("LOG: How far -> ");
  Serial.println(distance);
  delay(50);  
  Serial.print("LOG: How long -> ");
  Serial.println(duration);
  delay(50);
  if (distance < 20) {
    Serial.println("ACTION: move backwards");
  } else {
    if (distance = 0) {
    Serial.println("ACTION: stop");
    } else {
      Serial.println("ACTION: move forwards");
    }
  }
  delay(2000);
}
