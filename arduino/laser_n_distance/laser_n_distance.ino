#include <Wire.h>
#include <VL53L0X.h>

VL53L0X sensor;

// PINS
const int ProximityTriggerPin = 3;
const int ProximityEchoPin = 2;

int distanceVal;
int distancePrevVal;
int laserVal;
int laserPrevVal;

int incomingByte = 0;
bool sync = false;
String msg;

void setup() {
  
  Serial.begin(9600);
  Wire.begin();

  sensor.setTimeout(500);
  if (!sensor.init())
  {
    Serial.println("Failed to detect and initialize sensor!");
    while (1) {}
  }

  sensor.startContinuous();
  
  pinMode(LED_BUILTIN, OUTPUT);
  pinMode(ProximityTriggerPin, OUTPUT);
  pinMode(ProximityEchoPin, INPUT);
}

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

  laserVal = sensor.readRangeContinuousMillimeters();

  laserPrevVal = laserVal;
  msg.concat("laser=");
  msg.concat(laserVal);
  msg.concat("|");

  return msg;
}

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
    msg = getLaser(msg);  
    // DISTANCE SENSOR
    msg = getDistance(msg);
  }
  Serial.println(msg);
  delay(100);
}
