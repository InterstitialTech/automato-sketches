// rf95_client.ino
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

RH_RF95 rf95(PIN_LORA_CS, PIN_LORA_IRQ); // Slave select, interrupt pin for Automato Sensor Module.

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
  //while (!Serial) ; // Wait for serial port to be available
  Serial.println("Initializing LoRa"); 
  if (!rf95.init())
    Serial.println("init failed");
  // Defaults after init are 434.0MHz, 13dBm, Bw = 125 kHz, Cr = 4/5, Sf = 128chips/symbol, CRC on
  rf95.setFrequency(915.0);

  // print my mac id.
  Serial.print("my mac address:");
  Serial.println(Automato::macAddress());

  // The default transmitter power is 13dBm, using PA_BOOST.
  // You can set transmitter powers from 5 to 23 dBm:
  //  driver.setTxPower(23);

  on = true;
}

void loop()
{
  // build a test message.
  msgbuf test;

  uint64_t targetmac = 30960684215092;

  uint64_t mymac = Automato::macAddress(); 

  setupMessage(test.msg, mymac, targetmac, mt_write, PIN_LED, 1, (on ? 1 : 0));

  rf95.send((uint8_t*)&test.msg, sizeof(message));
  rf95.waitPacketSent();

  // Now wait for a reply
  uint8_t len = sizeof(test.buf);

  if (rf95.waitAvailableTimeout(3000))
  { 
    // Should be a reply message for us now   
    if (rf95.recv(test.buf, &len))
    {
      // what happened?  should be an ack.
      switch (test.msg.type) 
      {
        case mt_ack:
          switch (test.msg.payload) 
          {
            case ac_success:
              Serial.println("ac_success");
              break;
            case ac_invalid_address:
              Serial.println("ac_invalid_address");
              break;
            case ac_invalid_message_type:
              Serial.println("ac_invalid_message_type");
              break;
            default:
              Serial.println("unknown ack code");
              break;
          }
          break;
        default:
          Serial.print("unexpected message type; expected ack, got ");
          Serial.println(test.msg.type);
          break;
      }
      on = !on;
    }
    else
    {
      Serial.println("recv failed");
    }
  }
  else
  {
    Serial.println("No reply, is rf95_server running?");
  }
}
