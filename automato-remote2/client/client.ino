// client.ino
// -*- mode: C++ -*-

#include <SPI.h>
#include <AutomatoMsg.h>
#include <pins_arduino.h>
#include <Automato.h>

Automato automato(1);

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
  uint8_t targetaddr = 2;

  if (automato.remoteDigitalWrite(targetaddr, PIN_LED, (on ? 1 : 0))) 
  {
    Serial.print("successful write: ");
    Serial.println(on);
    on = !on;
  }
  else 
  {
    Serial.println("write failed!");
  }
}
