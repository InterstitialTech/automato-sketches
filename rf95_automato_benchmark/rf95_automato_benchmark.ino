// rf95_automato_benchmark.ino
// -*- mode: C++ -*-
// Example sketch showing how to create a simple messageing client
// with the RH_RF95 class. RH_RF95 class does not provide for addressing or
// reliability, so you should only use RH_RF95 if you do not need the higher
// level messaging abilities.
// It is designed to work with the other example rf95_server
// Tested with Automato Sensor Module.

#include <SPI.h>
#include <RH_RF95.h>

RH_RF95 rf95(PIN_LORA_CS, PIN_LORA_IRQ); // Slave select, interrupt pin for Automato Sensor Module.

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

  // The default transmitter power is 13dBm, using PA_BOOST.
  // You can set transmitter powers from 5 to 23 dBm:
  //  driver.setTxPower(23);
}

void loop()
{
  Serial.println("Testing rf95 packet sizes");

  uint8_t sendbuf[RH_RF95_MAX_MESSAGE_LEN];
  for (int i = 0; i < RH_RF95_MAX_MESSAGE_LEN; ++i) {
    sendbuf[i] = i;
  }

  int packets_per_size = 100;

  for (int len = 0; len < RH_RF95_MAX_MESSAGE_LEN; ++len)
  {
    Serial.print("testing packet length: ");
    Serial.println(len);

    for (int p = 0; p < packets_per_size; ++p) {
      sendmsg(&sendbuf, len);
    }
  }
}

void sendmsg(uint8_t *sendbuf, size_t size)
{
  long start = millis();
  // Send a message to rf95_server
  rf95.send(sendbuf, size);
  
  rf95.waitPacketSent();
  // Now wait for a reply
  uint8_t buf[RH_RF95_MAX_MESSAGE_LEN];
  uint8_t len = sizeof(buf);

  if (rf95.waitAvailableTimeout(3000))
  { 
    // Should be a reply message for us now   
    if (rf95.recv(buf, &len))
    {
      long end = millis();
      Serial.print("message length, ");
      Serial.print(size);
      Serial.print(", millis, ");
      Serial.println(end-start);
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
