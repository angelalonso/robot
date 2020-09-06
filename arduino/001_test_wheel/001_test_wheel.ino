
void setup() {
  #define enA 5
  #define in1 6
  #define in2 7
  
  pinMode(enA, OUTPUT);
  pinMode(in1, OUTPUT);
  pinMode(in2, OUTPUT);
  // Set initial rotation direction
  digitalWrite(in1, HIGH);
  digitalWrite(in2, LOW);
  Serial.begin(9600);
  int j;
  for ( j = 0; j < 256; j++) {
    analogWrite(enA, j);
    delay(50);                                            
    Serial.print("LOG: j is ");  //messages starting with LOG: will be read but not taken as result
    Serial.println(j);
  }
  delay(50); // always in front of a serial println
  Serial.println("Wheel Finished");
}
void loop() {
 
}
