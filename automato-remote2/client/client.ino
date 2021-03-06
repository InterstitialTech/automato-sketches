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

Automato automato(1, NULL, 0, true);

// the automato we're going to control remotely.
uint8_t serveraddr(2);

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

  automato.init(915.0, 20);

  // print my mac id.
  Serial.print("my mac address:");
  Serial.println(Automato::macAddress());

  on = true;

  pinMode(A0, INPUT);
  pinMode(A1, INPUT);
  pinMode(A6, INPUT);
  pinMode(A7, INPUT);
}

void loop()
{
  Serial.println("----------------------- test start ---------------------");

  AutomatoResult ar;

  // write to a pin on the remote automato.
  if (ar = automato.remoteDigitalWrite(serveraddr, PIN_LED, (on ? 1 : 0))) 
  {
    Serial.print("successful remoteDigitalWrite: ");
    Serial.println(on);
    on = !on;
  }
  else 
  {
    Serial.println("-------- failure ---------");
    Serial.println("remoteDigitalWrite failed!");
    Serial.println(ar.as_string());
  }

  // read a char field from the remote.
  char remotename[sizeof(ServerData::name)];
  if (ar = automato.remote_memread(serveraddr,
                             ServerData,
                             name,
                             remotename))
  {
    Serial.print("retrieved remote name: ");
    Serial.println(remotename);
  }
  else 
  {
    Serial.println("-------- failure ---------");
    Serial.println("failed to retrieve remote name!");
    Serial.println(ar.as_string());
  }

  // write a value to the ServerData memory map on the report.
  float temp = 75;
  if (ar = automato.remote_memwrite(serveraddr,
                               ServerData,
                               targettemp,
                               &temp))
  {
    Serial.print("wrote remote temp: ");
    Serial.println(temp);
  }
  else 
  {
    Serial.println("-------- failure ---------");
    Serial.println("failed to write remote temp!");
    Serial.println(ar.as_string());
  }

  // read digital status of a pin on the remote.
  uint8_t a7;
  automato.remotePinMode(serveraddr, A7, INPUT);
  if (ar = automato.remoteDigitalRead(serveraddr, A7, &a7)) 
  {
    Serial.print("read remote pin: ");
    Serial.println(a7);

    // write the value from the remote board to our LED.
    digitalWrite(PIN_LED, a7);
  }
  else 
  {
    Serial.println("-------- failure ---------");
    Serial.println("read remote pin failed!");
    Serial.println(ar.as_string());
  }

  // read remote temperature and humidity sensors.
  float temperature;
  float humidity;
  if (ar = automato.remoteTemperature(serveraddr, temperature))
  {
    Serial.print("remote temperature: ");
    Serial.println(temperature);
  }
  else
  {
    Serial.println("-------- failure ---------");
    Serial.println("error retrieving temperature");
    Serial.println(ar.as_string());
  }

  if (ar = automato.remoteHumidity(serveraddr, humidity))
  {
    Serial.print("remote humidity: ");
    Serial.println(humidity);
  }
  else
  {
    Serial.println("-------- failure ---------");
    Serial.println("error retrieving humidity");
    Serial.println(ar.as_string());
  }

  // remote general info.
  RemoteInfo serverinfo;
  if (ar = automato.remoteAutomatoInfo(serveraddr, serverinfo)) 
  {
    Serial.println("remote server info:");
    Serial.print("protoversion: ");
    Serial.println(serverinfo.protoversion);
    Serial.print("macAddress: ");
    Serial.println(serverinfo.macAddress);
    Serial.print("datalen: ");
    Serial.println(serverinfo.datalen);
  }
  else
  {
    Serial.println("-------- failure ---------");
    Serial.println("failed to retrieve remote info!");
    Serial.println(ar.as_string());
  }

  // read analog input value from the remote.
  uint16_t a6;
  if (ar = automato.remoteAnalogRead(serveraddr, A6, &a6)) 
  {
    Serial.print("remoteAnalogRead: ");
    Serial.println(a6);
  }
  else
  {
    Serial.println("-------- failure ---------");
    Serial.println("error reading remote analog pin");
    Serial.println(ar.as_string());
  }

  delay(500);
}
