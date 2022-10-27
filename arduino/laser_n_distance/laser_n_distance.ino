#include <Wire.h>
#include "Adafruit_VL53L0X.h"

// Distance sensor config
#define ProximityTriggerPin 2
#define ProximityEchoPin 3
int distanceVal;
int distancePrevVal;

// Laser sensor config
Adafruit_VL53L0X laser = Adafruit_VL53L0X();
VL53L0X_RangingMeasurementData_t measure;
#define LaserXSHUT 12
#define LOX1_ADDRESS 0x30
int laserVal;
int laserPrevVal;

// Sync and communication variables

int incomingByte = 0;
bool sync = false;
String msg;

void setLaserID() {
  // This function prepares the Laser
  // reset
  digitalWrite(LaserXSHUT, LOW);
  delay(10);
  // unreset
  digitalWrite(LaserXSHUT, HIGH);
  delay(10);

  // activating Laser
  digitalWrite(LaserXSHUT, HIGH);

  // initing Laser
  if(!laser.begin(LOX1_ADDRESS)) {
    Serial.println(F("Failed to boot first VL53L0X"));
    while(1);
  }
}

// Main setup function

void setup() {
  
  Serial.begin(9600);
  Wire.begin();

  // wait until serial port opens for native USB devices
  while (! Serial) { delay(1); }

  pinMode(ProximityTriggerPin, OUTPUT);
  pinMode(ProximityEchoPin, INPUT);
  pinMode(LaserXSHUT, OUTPUT);

  Serial.println(F("Laser Shutdown pin inited..."));

  digitalWrite(LaserXSHUT, LOW);

  Serial.println(F("Laser in reset mode...(pin is low)"));
  
  Serial.println(F("Starting Laser..."));
  setLaserID();
}

// Functions to get values

String getDistance(String msg) {
  digitalWrite(ProximityTriggerPin, LOW); // Reset triggerPin
  delayMicroseconds(2);
  digitalWrite(ProximityTriggerPin, HIGH); // Sets triggerPin on HIGH state for 10 microsecs
  delayMicroseconds(10);
  digitalWrite(ProximityTriggerPin, LOW);
  long duration = pulseIn(ProximityEchoPin, HIGH);
  distanceVal = duration*0.034/2;
  // This one is here because sensor throws 1000+ value when too close
  //if ((distanceVal - distancePrevVal) > 500 ) {
  //  distanceVal = 0;
  //}

  distancePrevVal = distanceVal;
  msg.concat("distance=");
  msg.concat(distanceVal);
  msg.concat("|");

  return msg;
}

String getLaser(String msg) {
  laser.rangingTest(&measure, false); // pass in 'true' to get debug data printout!

  if (measure.RangeStatus != 4) {  // phase failures have incorrect data
    laserVal = measure.RangeMilliMeter;
    
    laserPrevVal = laserVal;
    msg.concat("laser=");
    msg.concat(laserVal);
    msg.concat("|");
  }

  return msg;
}

// Main Loop

void loop() {
  
  msg = "SENSOR: ";

  if (Serial.available() > 0) {
    // read the incoming byte to sync:
    incomingByte = Serial.read();
    sync = true;
  }
  
  delay(30);
  if (sync == true) {
    // LASER SENSOR
    msg = getLaser(msg);  // not working, why?
    // DISTANCE SENSOR
    msg = getDistance(msg);
  }
  Serial.println(msg);
  delay(100);
}
