#include <Arduino.h>
// #include <WiFi.h>
// #include <WiFiUdp.h>

#include <esp_wifi.h>

#include <Ethernet.h>

#include <Automato.h>

// #include "peripherals.h"
// #include "credentials.h"


// Led redLed( GPIO_NUM_19 );
// Led greenLed( GPIO_NUM_21 );

Automato automato(0, false);

WiFiUDP udp;
const char *ssid = "meh";
const char *password = "meh";

const char *toStr( wl_status_t status ) {
    switch( status ) {
    case WL_NO_SHIELD: return "No shield";
    case WL_IDLE_STATUS: return "Idle status";
    case WL_NO_SSID_AVAIL: return "No SSID avail";
    case WL_SCAN_COMPLETED: return "Scan compleded";
    case WL_CONNECTED: return "Connected";
    case WL_CONNECT_FAILED: return "Failed";
    case WL_CONNECTION_LOST: return "Connection lost";
    case WL_DISCONNECTED: return "Disconnected";
    }
    return "Unknown";
}

void setupAp() {
    // Ethernet.begin();
    // greenLed.set();
    WiFi.begin("wat", 12, "qt");
    delay( 500 ); // If not used, somethimes following command fails
    ESP_ERROR_CHECK( esp_wifi_set_protocol( WIFI_IF_AP, WIFI_PROTOCOL_LR ) );
    WiFi.mode( WIFI_AP );


    Serial.println( WiFi.softAP( ssid, password ) );
    Serial.println( WiFi.softAPIP() );
    delay( 1000 );
    // greenLed.reset();
}

void setup() {
    Serial.begin( 115200 );
    Serial.println( "Master" );
    setupAp();
    udp.begin( 8888 );
}

void loop() {
    udp.beginPacket( { 192, 168, 4, 255 }, 8888 );
    udp.write( 'b' );
    // redLed.invert();
    if ( !udp.endPacket() )
        ESP.restart(); // When the connection is bad, the TCP stack refuses to work
    delay( 100 );
}
