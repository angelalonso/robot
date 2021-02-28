/*
Basic_I2C.ino
Brian R Taylor
brian.taylor@bolderflight.com
CodYright (c) 2017 Bolder Flight Systems
Permission is hereby granted, free of charge, to any person obtaining a codY of this software 
and associated documentation files (the "Software"), to deal in the Software without restriction, 
including without limitation the rights to use, codY, modify, merge, publish, distribute, 
sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is 
furnished to do so, subject to the following conditions:
The above codYright notice and this permission notice shall be included in all copies or 
substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING 
BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND 
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COdYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, 
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, 
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

#include "MPU9250.h"

const int SerialFreq = 115200;
const int samples = 70;
const int beat = 100;

// an MPU9250 object with the MPU-9250 sensor on I2C bus 0 with address 0x68
MPU9250 IMU(Wire,0x68);
int is_ok;
int current_millis,start_millis;
float AXbase,GXbase,MXbase = 0;
float AYbase,GYbase,MYbase = 0;
float AZbase,GZbase,MZbase = 0;
float AX,GX,MX;
float AXprev;
int AXdelta;
float AY,GY,MY;
float AZ,GZ,MZ;
float vX,vY,vZ = 0;
float dX,dY,dZ = 0;
float AXsum = 0;
float GXsum = 0;
float MXsum = 0;
float AYsum = 0;
float GYsum = 0;
float MYsum = 0;
float AZsum = 0;
float GZsum = 0;
float MZsum = 0;

void get_base_data(){
  float AXaux[samples],GXaux[samples],MXaux[samples]; 
  float AYaux[samples],GYaux[samples],MYaux[samples]; 
  float AZaux[samples],GZaux[samples],MZaux[samples]; 
  for (int i = 0; i < samples; i++) {
    IMU.readSensor();
    AXaux[i] = IMU.getAccelX_mss();
    GXaux[i] = IMU.getGyroX_rads();
    MXaux[i] = IMU.getMagX_uT();
    AYaux[i] = IMU.getAccelY_mss();
    GYaux[i] = IMU.getGyroY_rads();
    MYaux[i] = IMU.getMagY_uT();
    AZaux[i] = IMU.getAccelZ_mss();
    GZaux[i] = IMU.getGyroZ_rads();
    MZaux[i] = IMU.getMagZ_uT();
    AXsum = AXsum + AXaux[i];
    GXsum = GXsum + GXaux[i];
    MXsum = MXsum + MXaux[i];
    AYsum = AYsum + AYaux[i];
    GYsum = GYsum + GYaux[i];
    MYsum = MYsum + MYaux[i];
    AZsum = AZsum + AZaux[i];
    GZsum = GZsum + GZaux[i];
    MZsum = MZsum + MZaux[i];

  }
  AXbase = AXsum / samples;
  GXbase = GXsum / samples;
  MXbase = MXsum / samples;
  AYbase = AYsum / samples;
  GYbase = GYsum / samples;
  MYbase = MYsum / samples;
  AZbase = AZsum / samples;
  GZbase = GZsum / samples;
  MZbase = MZsum / samples;
}

void get_data(){
  float AXaux[samples],GXaux[samples],MXaux[samples]; 
  float AYaux[samples],GYaux[samples],MYaux[samples]; 
  float AZaux[samples],GZaux[samples],MZaux[samples]; 
  float AXsum = 0;
  float GXsum = 0;
  float MXsum = 0;
  float AYsum = 0;
  float GYsum = 0;
  float MYsum = 0;
  float AZsum = 0;
  float GZsum = 0;
  float MZsum = 0;
  for (int i = 0; i < samples; i++) {
    IMU.readSensor();
    AXaux[i] = IMU.getAccelX_mss();
    GXaux[i] = IMU.getGyroX_rads();
    MXaux[i] = IMU.getMagX_uT();
    AYaux[i] = IMU.getAccelY_mss();
    GYaux[i] = IMU.getGyroY_rads();
    MYaux[i] = IMU.getMagY_uT();
    AZaux[i] = IMU.getAccelZ_mss();
    GZaux[i] = IMU.getGyroZ_rads();
    MZaux[i] = IMU.getMagZ_uT();
    AXsum = AXsum + AXaux[i];
    GXsum = GXsum + GXaux[i];
    MXsum = MXsum + MXaux[i];
    AYsum = AYsum + AYaux[i];
    GYsum = GYsum + GYaux[i];
    MYsum = MYsum + MYaux[i];
    AZsum = AZsum + AZaux[i];
    GZsum = GZsum + GZaux[i];
    MZsum = MZsum + MZaux[i];
  }
  AXprev = AX;
  AX = (AXsum / samples) - AXbase;
  GX = (GXsum / samples) - GXbase;
  MX = (MXsum / samples) - MXbase;
  AY = (AYsum / samples) - AYbase;
  GY = (GYsum / samples) - GYbase;
  MY = (MYsum / samples) - MYbase;
  AZ = (AZsum / samples) - AZbase;
  GZ = (GZsum / samples) - GZbase;
  MZ = (MZsum / samples) - MZbase;
}

void get_pos() {
  AX = AX - (MX / 6);
  vX = vX + (AX * beat / 1000);
  dX = dX + (vX * beat / 1000);
}

void get_delta() {
  float test,AXdelta;
  if (AX > 0) {
    test = (AX - AXprev) / AXprev;
  } else {
    test = -(AX - AXprev) / AXprev;
  }
  if (test > 1 or test < -1) {
    AXdelta = test;
  } else {
    AXdelta = 0.0;
  }
  Serial.print(AXdelta);
  Serial.print("\t");
}

void setup() {
  // serial to display data
  Serial.begin(115200);
  while(!Serial) {}

  // start communication with IMU 
  is_ok = IMU.begin();
  if (is_ok < 0) {
    Serial.println("IMU init ERROR. Check IMU connection");
    Serial.print("Status: ");
    Serial.println(is_ok);
    while(1) {}
  }
  start_millis = millis();
  get_base_data();
}

void loop() {
  // read the sensor
  get_data();
  get_pos();
  // display the data
  current_millis = millis();
  while ((current_millis - start_millis) < beat) {
    delay(1);
    current_millis = millis();
  }
  //Serial.print(dX);
  Serial.print(AX);
  Serial.print("\t");
  //get_delta();
  //Serial.print(GX);
  //Serial.print("\t");
  //Serial.print(MX);
  //Serial.print("\t");
  //Serial.print(AY);
  //Serial.print("\t");
  //Serial.print(GY);
  //Serial.print("\t");
  //Serial.print(MY);
  //Serial.print("\t");
  //Serial.print(AZ);
  //Serial.print("\t");
  //Serial.print(GZ);
  //Serial.print("\t");
  //Serial.print(MZ);
  //Serial.print("\t");
  Serial.println("");
  start_millis = current_millis;
}
