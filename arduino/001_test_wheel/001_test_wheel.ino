#define enA 9
#define in1 6
#define in2 7
#define button 4
int rotDirection = 0;
void setup() {
  pinMode(enA, OUTPUT);
  pinMode(in1, OUTPUT);
  pinMode(in2, OUTPUT);
  pinMode(button, INPUT);
  // Set initial rotation direction
  digitalWrite(in1, HIGH);
  digitalWrite(in2, LOW);
  Serial.begin(9600);
  int j;
  for ( j = 0; j < 10; j++) {
    analogWrite(enA, j);
    delay(50);                                            
    Serial.print("LOG: j is ");  //messages starting with LOG: will be read but not taken as result
    Serial.println(j);
  }
}
void loop() {
 
}
