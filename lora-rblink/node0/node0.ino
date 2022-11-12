/*
 * Automato "Hello World"
 *  - Display temperature, humidity, and chip ID on the screen
 *  
 *  Interstitial Technology, 20210721
 *  info@interstitial.coop
 *  
 */

#include <Automato.h>

Automato automato(0, false);

void setup() {
    pinMode(PIN_LORA_RST, INPUT); // Let the pin float.

    // Disable other SPI devices.
    // pinMode(PIN_LCD_CS, OUTPUT);
    // digitalWrite(PIN_LCD_CS, HIGH);
    // pinMode(PIN_TCH_CS, OUTPUT);
    // digitalWrite(PIN_TCH_CS, HIGH);
    // pinMode(PIN_SD_CS, OUTPUT);
    // digitalWrite(PIN_SD_CS, HIGH);

    Serial.begin(115200);

    Serial.println("automato remote control client");
    automato.init();

    delay(1000);
}

bool on = false;

void loop() {
    Serial.print("digitalwrite "); 
    auto res = automato.remoteDigitalWrite(1, PIN_LED, on); 
    Serial.print("result:"); 
    Serial.println(res); 
    on = !on;
}

