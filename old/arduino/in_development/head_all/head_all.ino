#include <Servo.h>
Servo myservo;

int min_dist = 20;

int servo_pos = 0; 
char servo_rotate = 'l';

const int pin_in=3;
const int pin_out=4;
const int enAPin=12;//   Left  motor PWM speed control
const int in1Pin=11;//   Left  motor  Direction  1
const int in2Pin=10;//   Left  motor  Direction 
const int delay_servo=5;   // waits 15ms for the servo to reach the servo_position

const int delay_eyes=100;

void setup(){
  myservo.attach(13);

  Serial.begin(9600);

  pinMode(pin_in, INPUT);
  pinMode(pin_out, OUTPUT);
  pinMode(enAPin, OUTPUT);
  pinMode(in1Pin, OUTPUT);
  pinMode(in2Pin, OUTPUT);
  digitalWrite(in1Pin, LOW);
  digitalWrite(in2Pin, LOW);
}

void loop()
{
  //directionControl();
  //delay(1000);
  if (isObstacle()){
    Serial.println("SERVO IS AT " + String(servo_pos));
    rotate();
  }
}

void directionControl() {
  // Set motors to maximum speed
  // For PWM maximum possible values are 0 to 255
  analogWrite(enAPin, 255);
  digitalWrite(in1Pin, HIGH);
  digitalWrite(in2Pin, LOW);
}

bool isObstacle()
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
  if (tocm <= min_dist) {
    return true;
  } else {
    return false;
  }
}

void rotate(){
  if (servo_pos <= 0) {
    servo_rotate = 'r';
  }
  if (servo_pos >= 180) {
    servo_rotate = 'l';
  }
  if (servo_rotate == 'r') {
    servo_pos += 3;
  }
  if (servo_rotate == 'l') {
    servo_pos -= 3;
  }
  myservo.write(servo_pos);
}

long microsecondsToCentimeters(long microseconds)
{
  return microseconds / 29 / 2;
}
