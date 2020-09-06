void setup() {
  // initialize digital pin LED_BUILTIN as an output.
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(9600);
}

// the loop function runs over and over again forever
void loop() {
// put your main code here, to run repeatedly
  digitalWrite(LED_BUILTIN, HIGH);   // turn the LED on 
  delay(50);                       
  digitalWrite(LED_BUILTIN, LOW);    // turn the LED off 
  delay(100);     
  digitalWrite(LED_BUILTIN, HIGH);   // turn the LED on 
  delay(50);                       
  digitalWrite(LED_BUILTIN, LOW);    // turn the LED off 
  delay(800);                      
  Serial.println("LOG: Ready");  //messages starting with LOG: will be read but not taken as result
}
