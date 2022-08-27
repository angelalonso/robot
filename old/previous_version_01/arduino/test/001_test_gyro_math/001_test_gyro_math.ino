#include<Wire.h>
#include<math.h>
const int MPU=0x68;
int16_t AcX,AcY,AcZ,GyX,GyY,GyZ;
//int16_t Tmp;
//
double pitch,roll;

void setup() {
  Wire.begin();
  Wire.beginTransmission(MPU);
  Wire.write(0x6B);
  Wire.write(0);
  Wire.endTransmission(true);
  Serial.begin(9600);
}

void loop() {
  Wire.beginTransmission(MPU);
  Wire.write(0x3B);
  Wire.endTransmission(false);
  Wire.requestFrom(MPU,12,true);
  //
  int AcXoff,AcYoff,AcZoff,GyXoff,GyYoff,GyZoff;
  int temp,toff;
  double t,tx,tf;
  //
  AcXoff = -950;
  AcYoff = -300;
  AcZoff = 0;
  //
  toff = -1600;
  //
  GyXoff = 480;
  GyYoff = 170;
  GyZoff = 210;
  //
  AcX=Wire.read()<<8|Wire.read() + AcXoff;
  AcY=Wire.read()<<8|Wire.read() + AcYoff;
  AcZ=Wire.read()<<8|Wire.read() + AcZoff;
  //
  temp=Wire.read()<<8|Wire.read() + toff;
  tx=temp;
  t = tx/340 + 36.53;
  // 
  GyX=Wire.read()<<8|Wire.read() + GyXoff;
  GyY=Wire.read()<<8|Wire.read() + GyYoff;
  GyZ=Wire.read()<<8|Wire.read() + GyZoff;
  //
  getAngle(AcX,AcY,AcZ);
  Serial.print("Angle: ");
  Serial.print("Pitch = ");Serial.print(pitch);
  Serial.print(" | Roll = ");Serial.println(roll);
  
  //AcX=Wire.read()<<8|Wire.read();
  //AcY=Wire.read()<<8|Wire.read();
  //AcZ=Wire.read()<<8|Wire.read();
  //Tmp=Wire.read()<<8|Wire.read();
  //
  //GyX=Wire.read()<<8|Wire.read();
  //GyY=Wire.read()<<8|Wire.read();
  //GyZ=Wire.read()<<8|Wire.read();

  //Serial.print("Accelerometer: ");
  //Serial.print("X = ");Serial.print(AcX);
  //Serial.print(" | Y = ");Serial.print(AcY);
  //Serial.print(" | Z = ");Serial.println(AcZ);

  //Serial.print("Temp: ");Serial.println(Tmp);
  //
  Serial.print("Temp: ");Serial.println(t);
  
  //Serial.print("Gyroscope: ");
  //Serial.print("X = ");Serial.print(GyX);
  //Serial.print(" | Y = ");Serial.print(GyY);
  //Serial.print(" | Z = ");Serial.println(GyZ);
  //Serial.println(" ");
  delay(4000);
}

void getAngle(int Vx, int Vy, int Vz) {
  double x = Vx;
  double y = Vy;
  double z = Vz;
  pitch = atan(x/sqrt((y*y) + (z*z)));
  roll = atan(y/sqrt((x*x) + (z*z)));
  //radians to degrees
  pitch = pitch * (180.0/3.14);
  roll = roll * (180.0/3.14);
}
