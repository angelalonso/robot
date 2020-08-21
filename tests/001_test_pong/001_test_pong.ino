void setup() {
  // initialize digital pin LED_BUILTIN as an output.
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(9600);

}

// the loop function runs over and over again forever
void loop() {
  for (int j = 0; j < 10; j++) {
    digitalWrite(LED_BUILTIN, HIGH);   // turn the LED on 
    delay(50);                       // wait for half a second
    digitalWrite(LED_BUILTIN, LOW);    // turn the LED off 
    delay(950);                       // wait for half a second
    Serial.println(j, DEC);
  }
  Serial.println("pong");
}
