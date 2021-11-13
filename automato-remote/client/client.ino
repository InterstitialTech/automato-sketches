// client.ino
// -*- mode: C++ -*-
// Example sketch showing how to create a simple messageing client
// with the RH_RF95 class. RH_RF95 class does not provide for addressing or
// reliability, so you should only use RH_RF95 if you do not need the higher
// level messaging abilities.
// It is designed to work with the other example rf95_server
// Tested with Automato Sensor Module.

#include <SPI.h>
#include <AutomatoMsg.h>
#include <pins_arduino.h>
#include <Automato.h>

Automato automato;

bool on;

void setup() 
{
  pinMode(PIN_LORA_RST, INPUT); // Let the pin float.

  // Disable other SPI devices.
  pinMode(PIN_LCD_CS, OUTPUT);
  digitalWrite(PIN_LCD_CS, HIGH);
  pinMode(PIN_TCH_CS, OUTPUT);
  digitalWrite(PIN_TCH_CS, HIGH);
  pinMode(PIN_SD_CS, OUTPUT);
  digitalWrite(PIN_SD_CS, HIGH);

  Serial.begin(115200);

  Serial.println("automato remote control client");

  automato.init();

  // print my mac id.
  Serial.print("my mac address:");
  Serial.println(Automato::macAddress());

  on = true;
}

void loop()
{
  // the automato we're going to control remotely.
  uint64_t targetmac = 30960684215092;

  if (automato.remoteDigitalWrite(targetmac, PIN_LED, (on ? 1 : 0))) 
  {
    Serial.print("successful write: ");
    Serial.println(on);
    on = !on;
  }
  else 
  {
    Serial.print("write failed!");
  }
}
