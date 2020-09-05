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
}
void loop() {
  analogWrite(enA, 255); 
}
