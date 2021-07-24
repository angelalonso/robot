
#include "MPU9250.h"

MPU9250 *IMU;
int status;

const  double xBias = 1624.766973;
const  double yBias = -977.863214;
const  double zBias = -674.986618;

const  double xScaleX = 0.012480;
const  double xScaleY = 0.000097;
const  double xScaleZ = 0.000032;

const  double yScaleX = 0.000097;
const  double yScaleY = 0.012680;
const  double yScaleZ = -0.000110;


const unsigned ACLineFrequency = 50;
const unsigned samplesPerACCycle = 2;
const unsigned sampleCount = ACLineFrequency * samplesPerACCycle;
const unsigned  sampleIntervalMicroseconds = 1000000UL / sampleCount;

 double *samplesX;
 double *samplesY;

 double sampleTotalX = 0;
 double sampleTotalY = 0;

 double sampleAverageX = 0;
 double sampleAverageY = 0;

unsigned sampleIndexX = 0;
unsigned sampleIndexY = 0;

unsigned  LastsampleTime;

 double getNewValue(char dimension)
{
  IMU->readSensor();

   double x, y, z;

  x = IMU->getMagX_uT() * (100000.0 / 1100.0) + xBias;
  y = IMU->getMagY_uT() * (100000.0 / 1100.0) + yBias;
  z = IMU->getMagZ_uT() * (100000.0 / 1100.0) + zBias;

   double calX, calY, calZ;

  calX = x * xScaleX + y * xScaleY + z * xScaleZ;
  calY = x * yScaleX + y * yScaleY + z * yScaleZ;

  if (dimension == 'x')
    return calX; 
 else if (dimension == 'y')
    return calY;
  else
    return calZ;
}


void Addsample()
{
  sampleTotalX -= samplesX[sampleIndexX];
  samplesX[sampleIndexX] = getNewValue('x');
  sampleTotalX += samplesX[sampleIndexX];

  sampleTotalY -= samplesY[sampleIndexY];
  samplesY[sampleIndexY] = getNewValue('y');
  sampleTotalY += samplesY[sampleIndexY];

  sampleIndexX = (sampleIndexX + 1) % sampleCount;
  sampleIndexY = (sampleIndexY + 1) % sampleCount;
}



void setup() {
  IMU = new MPU9250(Wire, 0x68);
  Serial.begin(115200);

  while (!Serial) {}
  status = IMU->begin();
  if (status < 0) {
     //...
  }

  samplesX = new  double[sampleCount]();
  samplesY = new  double[sampleCount]();

  for (unsigned i = 0; i < sampleCount; i++)
  {
    Addsample();
    delayMicroseconds(sampleIntervalMicroseconds);
  }
  LastsampleTime = micros();

  sampleAverageX = sampleTotalX / sampleCount;
}

void loop() {

  unsigned  currentMicros = micros();
  if (currentMicros - LastsampleTime >= sampleIntervalMicroseconds)
  {
    LastsampleTime += sampleIntervalMicroseconds;
    Addsample();

    sampleAverageX = sampleTotalX / sampleCount;
    sampleAverageY = sampleTotalY / sampleCount;


  }

   double heading = atan2(sampleAverageY, sampleAverageX) * 180.0 / PI;
  if (heading < 0.0) heading = heading + 360.0;


  Serial.println(heading, 6);


}
