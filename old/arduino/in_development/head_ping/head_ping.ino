#include <Servo.h>
Servo myservo;
int pos = 0; 

const int pin_in=3;
const int pin_out=4;

void setup(){
myservo.attach(9);

Serial.begin(9600);
pinMode(pin_in, INPUT);
pinMode(pin_out, OUTPUT);
}

void loop()
{
long dur;
long dis;
long tocm;
digitalWrite(pin_out,LOW);
delayMicroseconds(2);
digitalWrite(pin_out,HIGH);
delayMicroseconds(10);
digitalWrite(pin_out,LOW);
dur=pulseIn(pin_in,HIGH);
tocm=microsecondsToCentimeters(dur);
Serial.println(String(tocm));
delay(100);
for (pos = 0; pos <= 180; pos += 1) { // goes from 0 degrees to 180 degrees
  // in steps of 1 degree
  myservo.write(pos);              // tell servo to go to position in variable 'pos'
  delay(15);                       // waits 15ms for the servo to reach the position
}
for (pos = 180; pos >= 0; pos -= 1) { // goes from 180 degrees to 0 degrees
  myservo.write(pos);              // tell servo to go to position in variable 'pos'
  delay(15);                       // waits 15ms for the servo to reach the position
}
}

long microsecondsToCentimeters(long microseconds)
{
return microseconds / 29 / 2;
}
