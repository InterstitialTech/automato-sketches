/*
 * Automato "Hello World"
 *  - Display temperature, humidity, and chip ID on the screen
 *  
 *  Interstitial Technology, 20210721
 *  info@interstitial.coop
 *  
 */

#include <Automato.h>

Automato automato(1, true);

void setup() {
    pinMode(PIN_LORA_RST, INPUT); // Let the pin float.

    Serial.begin(115200);

    automato.init();

}

void loop() {

    automato.doRemoteControl();

}

