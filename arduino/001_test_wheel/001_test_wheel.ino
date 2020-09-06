
void setup() {
  #define inA1 4
  #define inA2 5
  #define enA 6
  #define inB1 7
  #define inB2 8
  #define enB 9
  
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
  int j;
  for ( j = 0; j < 256; j++) {
    analogWrite(enA, j);
    analogWrite(enB, j);
    delay(50);                                            
    Serial.print("LOG: j is ");  //messages starting with LOG: will be read but not taken as result
    Serial.println(j);
  }
  delay(50); // always in front of a serial println
  Serial.println("Wheel Finished");
}
void loop() {
 
}
