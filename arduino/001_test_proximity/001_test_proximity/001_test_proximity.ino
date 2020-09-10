const int motorInA1Pin = 4;
const int motorInA2Pin = 5;
const int motorEnAPin = 6;
const int motorInB1Pin = 7;
const int motorInB2Pin = 8;
const int motorEnBPin = 9;
const int motorSpeed = 255;
const int proxTriggerPin = 10;
const int proxEchoPin = 11;
// vars
long duration;
int distance;

void setup() {
  pinMode(motorEnAPin, OUTPUT);
  pinMode(motorInA1Pin, OUTPUT);
  pinMode(motorInA2Pin, OUTPUT);
  pinMode(motorEnBPin, OUTPUT);
  pinMode(motorInB1Pin, OUTPUT);
  pinMode(motorInB2Pin, OUTPUT);
  // Set initial rotation direction
  digitalWrite(motorInA1Pin, LOW);
  digitalWrite(motorInA2Pin, HIGH);
  digitalWrite(motorInB1Pin, LOW);
  digitalWrite(motorInB2Pin, HIGH);
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
  Serial.print("LOG: Distance -> ");
  Serial.println(distance);
  if (distance < 10) {
    Serial.print("LOG: TOO CLOSE!");
      analogWrite(motorEnAPin, motorSpeed);
      analogWrite(motorEnBPin, motorSpeed);
  } else {
      analogWrite(motorEnAPin, 0);
      analogWrite(motorEnBPin, 0);
  }

}
