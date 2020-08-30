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
    Serial.print("LOG: j is ");  //messages starting with LOG: will be read but not taken as result
    Serial.println(j);
  }
  digitalWrite(LED_BUILTIN, LOW); 
  delay(50); // we need some delay between two consecutive serial.println to be read by the brain
  Serial.println("ping");
}

// the loop function runs over and over again forever
void loop() {
// put your main code here, to run repeatedly
}
