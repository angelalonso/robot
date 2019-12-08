const int pin_in=3;
const int pin_out=4;

void setup(){
Serial.begin(9600);
pinMode(in, INPUT);
pinMode(out, OUTPUT);
}

void loop()
{
long dur;
long dis;
long tocm;
digitalWrite(out,LOW);
delayMicroseconds(2);
digitalWrite(out,HIGH);
delayMicroseconds(10);
digitalWrite(out,LOW);
dur=pulseIn(in,HIGH);
tocm=microsecondsToCentimeters(dur);
Serial.println(String(tocm));
delay(100);
}

long microsecondsToCentimeters(long microseconds)
{
return microseconds / 29 / 2;
}
