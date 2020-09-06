
void setup() {
  #define inA1 4
  #define inA2 5
  #define enA 6
  #define inB1 7
  #define inB2 8
  #define enB 9
  #define speed 255
  
  pinMode(enA, OUTPUT);
  pinMode(inA1, OUTPUT);
  pinMode(inA2, OUTPUT);
  pinMode(enB, OUTPUT);
  pinMode(inB1, OUTPUT);
  pinMode(inB2, OUTPUT);
  // Set initial rotation direction
  digitalWrite(inA1, HIGH);
  digitalWrite(inA2, LOW);
  digitalWrite(inB1, HIGH);
  digitalWrite(inB2, LOW);
  Serial.begin(9600);
}
void loop() {
  int i;
  for ( i = 0; i < 2; i++) {
    delay(1000);     
    Serial.println("LOG: Un,");
    delay(1000);
    Serial.println("LOG: dos,");
    delay(1000);
    Serial.println("LOG: tres!");
    delay(1000);
    Serial.println("LOG: Un pasito palante, Maria!");
    digitalWrite(inA1, HIGH);
    digitalWrite(inA2, LOW);
    digitalWrite(inB1, HIGH);
    digitalWrite(inB2, LOW);
    analogWrite(enA, speed);
    analogWrite(enB, speed);
    delay(1000);
    analogWrite(enA, 0);
    analogWrite(enB, 0);
    Serial.println("LOG: Un,");
    delay(1000);
    Serial.println("LOG: dos,");
    delay(1000);
    Serial.println("LOG: tres!");
    delay(1000);
    Serial.println("LOG: Un pasito patras...");
    digitalWrite(inA1, LOW);
    digitalWrite(inA2, HIGH);
    digitalWrite(inB1, LOW);
    digitalWrite(inB2, HIGH);
    analogWrite(enA, speed);
    analogWrite(enB, speed);
    delay(1000);
    analogWrite(enA, 0);
    analogWrite(enB, 0);
  }
  Serial.println("LOG: Aunque me muera ahora!");
  digitalWrite(inA1, HIGH);
  digitalWrite(inA2, LOW);
  digitalWrite(inB1, LOW);
  digitalWrite(inB2, HIGH);
  analogWrite(enB, speed);
  delay(2000);
  analogWrite(enA, 0);
  analogWrite(enB, 0);
  delay(50); // always in front of a serial println
  Serial.println("Wheel Finished");
}
