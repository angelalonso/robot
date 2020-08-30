void setup() {
  // initialize digital pin LED_BUILTIN as an output.
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(9600);
  int j;
  for ( j = 0; j < 10; j++) {
    digitalWrite(LED_BUILTIN, LOW);   // turn the LED on 
    delay(50);                       
    digitalWrite(LED_BUILTIN, HIGH);    // turn the LED off 
    delay(950);                      
    Serial.print("LOG: j is ");
    Serial.println(j);
  }
  delay(500); 
  Serial.println("ping");
}

// the loop function runs over and over again forever
void loop() {
// put your main code here, to run repeatedly
}
