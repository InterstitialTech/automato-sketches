// rf95_client.ino
// -*- mode: C++ -*-
// Example sketch showing how to create a simple messageing client
// with the RH_RF95 class. RH_RF95 class does not provide for addressing or
// reliability, so you should only use RH_RF95 if you do not need the higher
// level messaging abilities.
// It is designed to work with the other example rf95_server
// Tested with Automato Sensor Module.

#include <SPI.h>
#include <RH_RF95.h>
#include <registermsg.hh>
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
  message test;

  uint64_t targetmac = 30960684215092;

  uint64_t mymac = Automato::macAddress(); 

  /*
  char macbuf[6];

  uint64_t testmac;

  Serial.print("mymac");
  Serial.println(mymac);
  memcpy(macbuf, (const char*)&mymac, 6);
  const char *cmac = (const char*)&mymac;
  Serial.print("mymac0: ");
  Serial.println((int)macbuf[0]);
  Serial.print("mymac1: ");
  Serial.println((int)macbuf[1]);
  Serial.print("mymac2: ");
  Serial.println((int)macbuf[2]);
  Serial.print("mymac3: ");
  Serial.println((int)macbuf[3]);
  Serial.print("mymac4: ");
  Serial.println((int)macbuf[4]);
  Serial.print("mymac5: ");
  Serial.println((int)macbuf[5]);

  memcpy((char*)&testmac, macbuf, 6);
  Serial.print("testmac");
  Serial.println(testmac);

  cmac = (const char*)&testmac;
  Serial.print("testmac0: ");
  Serial.println((int)macbuf[0]);
  Serial.print("testmac1: ");
  Serial.println((int)macbuf[1]);
  Serial.print("testmac2: ");
  Serial.println((int)macbuf[2]);
  Serial.print("testmac3: ");
  Serial.println((int)macbuf[3]);
  Serial.print("testmac4: ");
  Serial.println((int)macbuf[4]);
  Serial.print("testmac5: ");
  Serial.println((int)macbuf[5]);

  Serial.print("targetmac");
  Serial.println(targetmac);
  */

  setupMessage(test, mymac, targetmac, mt_write, PIN_LED, 1, (on ? 1 : 0));

  // memcpy(test.frommac, ((const char*)&mymac), 4); 
  // memcpy(test.tomac, ((const char*)&targetmac), 4); 
  // test.type = mt_write;
  // test.address = 0;
  // test.length = 1;
  // test.payload = 1;

  // Serial.println("Sending message:");
  // printMessage(test);

  // Serial.println("Sending to rf95_server");
  // Send a message to rf95_server
  // float sample_data = 1.23;
  // rf95.send((uint8_t*)&sample_data, sizeof(sample_data));
  rf95.send((uint8_t*)&test, sizeof(message));
  
  rf95.waitPacketSent();
  // Now wait for a reply
  uint8_t buf[RH_RF95_MAX_MESSAGE_LEN];
  uint8_t len = sizeof(buf);

  if (rf95.waitAvailableTimeout(3000))
  { 
    // Should be a reply message for us now   
    if (rf95.recv(buf, &len))
    {
      // Serial.print("got reply: ");
      // Serial.println((char*)buf);
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
  // delay(100);

    // String s = Serial.readString();
    // if (s != "") {
    //   // interpret string command.
    // }
}
