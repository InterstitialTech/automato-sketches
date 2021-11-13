// server.ino
// -*- mode: C++ -*-

#include <SPI.h>
#include <RH_RF95.h>
#include <AutomatoMsg.h>
#include <Automato.h>

Automato automato;

void setup()
{
  pinMode(PIN_LORA_RST, INPUT); // Let the pin float.
  pinMode(PIN_LED, OUTPUT);

  // Disable SPI devices until needed.
  pinMode(PIN_LCD_CS, OUTPUT);
  digitalWrite(PIN_LCD_CS, HIGH);
  pinMode(PIN_TCH_CS, OUTPUT);
  digitalWrite(PIN_TCH_CS, HIGH);
  pinMode(PIN_SD_CS, OUTPUT);
  digitalWrite(PIN_SD_CS, HIGH);
  
  Serial.begin(115200);

  automato.init();

  Serial.println("automato remote control server");

  Serial.print("my mac address:");
  Serial.println(Automato::macAddress());

}

void loop()
{
  automato.doRemoteControl();
}
