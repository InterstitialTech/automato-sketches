#include <Automato.h>

RH_RF95 rf95(PIN_LORA_CS, PIN_LORA_IRQ); // Slave select, interrupt pin for Automato Sensor Module.
Adafruit_ILI9341 tft = Adafruit_ILI9341(PIN_LCD_CS, PIN_LCD_DC, PIN_LCD_RST);
SHTC3 shtc3;
float temperature, humidity;
String lora_result = "NO REPLY";

#define UPDATE_MILLIS 3000  // 3 second LCD update cycle.
unsigned long update_timer;

void setup() {
  // user LED
  pinMode(PIN_LED, OUTPUT);

  // Disable SPI devices until needed.
  pinMode(PIN_TCH_CS, OUTPUT);
  digitalWrite(PIN_TCH_CS, HIGH);
  pinMode(PIN_SD_CS, OUTPUT);
  digitalWrite(PIN_SD_CS, HIGH);
  pinMode(PIN_LORA_CS, OUTPUT);
  digitalWrite(PIN_LORA_CS, HIGH);
  pinMode(PIN_LCD_CS, OUTPUT);
  digitalWrite(PIN_LCD_CS, HIGH);

  // USB serial
  Serial.begin(115200);
  while(!Serial);

  // LoRa
  pinMode(PIN_LORA_RST, INPUT); // Let LoRa reset pin float (no reset).
  if (!rf95.init())
    Serial.println("LoRa init failed");
  // Defaults are 434.0MHz, 13dBm, Bw = 125 kHz, Cr = 4/5, Sf = 128chips/symbol, CRC on.
  rf95.setFrequency(915.0);

  // SHTC3
  Wire.begin();
  shtc3.begin();

  // LCD
  pinMode(PIN_LED_LCD, OUTPUT);
  digitalWrite(PIN_LED_LCD, HIGH);
  tft.begin();
  tft.setRotation(1);

  update_timer = millis();
}

void updateLCD() {

  tft.fillScreen(ILI9341_BLACK);

  tft.setCursor(0,0);
  tft.setTextColor(ILI9341_GREEN);
  tft.setTextSize(4);
  tft.println("Hello World!");
  tft.println();

  tft.setTextSize(2);

  tft.setTextColor(ILI9341_RED);
  tft.print("Temperature = ");
  tft.print(temperature);
  tft.println(" F");
  tft.println();

  tft.setTextColor(ILI9341_BLUE);
  tft.print("Humidity = ");
  tft.print(humidity);
  tft.println("%");
  tft.println();

  tft.setTextColor(ILI9341_GREEN);
  tft.print("LoRa ");
  tft.println(lora_result);
  tft.println();

}


void loop() {
  // Handle sensor and LCD update every UPDATE_MILLIS milliseconds.
  unsigned long now = millis();
  if (now - update_timer > UPDATE_MILLIS) {
    // Reset timer.
    update_timer = now;
    
    // get a reading from the temp/humidity sensor
    shtc3.update();
    temperature = shtc3.toDegF();
    humidity = shtc3.toPercent();

    // output to LCD
    updateLCD();

    // output to serial
    Serial.print("RH = ");
    Serial.print(shtc3.toPercent());
    Serial.print("%, T = ");
    Serial.print(shtc3.toDegF());
    Serial.println(" deg F");

    // Broadcast LoRa message.
    Serial.println("Sending to rf95_server.");
    // Send a message to rf95_server
    rf95.send((uint8_t*)&temperature, sizeof(temperature));
    rf95.waitPacketSent();

    // Now wait for a reply
    uint8_t buf[RH_RF95_MAX_MESSAGE_LEN];
    uint8_t len = sizeof(buf);
    
    if (rf95.waitAvailableTimeout(3000)) { 
      // Should be a reply message for us now   
      if (rf95.recv(buf, &len)) {
        Serial.print("got reply: ");
        Serial.println((char*)buf);
        lora_result = "SEND SUCCESS";
      }
      else {
        Serial.println("recv failed");
        lora_result = "RECV FAIL";
      }
    }
    else {
      Serial.println("No reply, is rf95_server running?");
      lora_result = "NO REPLY";
    }

    tft.println();
    
  }
  

}
