// rf95_server.ino
// -*- mode: C++ -*-
// 

#include <SPI.h>
#include <RH_RF95.h>
#include <AutomatoMsg.h>
#include <Automato.h>

RH_RF95 rf95(PIN_LORA_CS, PIN_LORA_IRQ); // Slave select, interrupt pin for Automato Sensor Module.

#define REGISTER_COUNT 100
char registers[REGISTER_COUNT];
#define REGISTER_LED 0

#define REGISTER_LAYOUT_VERSION 1

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

  //while (!Serial); // Wait for serial port to be available

  Serial.println("Initializing LoRa"); 
  if (!rf95.init())
    Serial.println("init failed");  
  // Defaults after init are 434.0MHz, 13dBm, Bw = 125 kHz, Cr = 4/5, Sf = 128chips/symbol, CRC on
  // specify by country?
  rf95.setFrequency(915.0);

  // The default transmitter power is 13dBm, using PA_BOOST.
  // You can set transmitter powers from 5 to 23 dBm:
  //  driver.setTxPower(23);

  // print my mac id.
  Serial.println("rf95 server");

  Serial.print("my mac address:");
  Serial.println(Automato::macAddress());

  // init register array to zero.
  for (int i; i < REGISTER_COUNT; ++i)
    registers[i] = 0;
}

void loop()
{
  msgbuf mb;

  // get my id.
  
  if (rf95.available())
  {
    // Should be a message for us now   
    uint8_t len = sizeof(mb.buf);
    float temp_data;
    if (rf95.recv(mb.buf, &len))
    {
      uint64_t tomac = 0;
      memcpy(((char*)&tomac), &mb.msg.tomac, 6); 
      
      if (tomac == Automato::macAddress()) 
      {
        switch (mb.msg.type) {
          case mt_write:
            if (0 <= mb.msg.address && mb.msg.address < 40) {
              if (mb.msg.payload == 0) {
                digitalWrite(mb.msg.address, LOW);
                // indicate success
                mb.msg.type = mt_ack;
                mb.msg.payload = ac_success;
                rf95.send((const uint8_t*)&mb.msg, sizeof(message));
                rf95.waitPacketSent();
              } else if (mb.msg.payload == 1) {
                digitalWrite(mb.msg.address, HIGH);
                // indicate success
                mb.msg.type = mt_ack;
                mb.msg.payload = ac_success;
                rf95.send((const uint8_t*)&mb.msg, sizeof(message));
                rf95.waitPacketSent();
              }
            } else {
              // failed, invalid address.
              mb.msg.type = mt_ack;
              mb.msg.payload = ac_invalid_address;
              rf95.send((const uint8_t*)&mb.msg, sizeof(message));
              rf95.waitPacketSent();
            };
            break;
          default:
            // failed, invalid message type.
            mb.msg.type = mt_ack;
            mb.msg.payload = ac_invalid_message_type;
            rf95.send((const uint8_t*)&mb.msg, sizeof(message));
            rf95.waitPacketSent();
            break;
        };
      }
      else {
        Serial.print("ignoring message for ");
        Serial.println(tomac);
        Serial.print("because my mac is ");
        Serial.println(Automato::macAddress());
      }
      // print message AFTER processing.
      // printMessage(mb.msg);
    }
    else
    {
      Serial.println("recv failed");
    }

  }
}
