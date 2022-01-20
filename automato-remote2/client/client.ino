// client.ino
// -*- mode: C++ -*-

#include <SPI.h>
#include <AutomatoMsg.h>
#include <pins_arduino.h>
#include <Automato.h>

// ideally this would go in a shared header file,
struct ServerData {
  char name[25];
  float targettemp;
  uint64_t macAddress;
  float temperature;
  float humidity;
};

Automato automato(1, NULL, 0);

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
  Serial.println("automato remote control client loop");

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

  char name[sizeof(ServerData::name)];
  if (automato.remoteMemRead(targetaddr,
                             (uint16_t)offsetof(ServerData, name),
                             (uint8_t)sizeof(ServerData::name),
                             (void*)name))
  {
    Serial.print("retrieved remote name: ");
    Serial.println(name);
  }
  else 
  {
    Serial.println("failed to retreive remote name!");
  }

}
