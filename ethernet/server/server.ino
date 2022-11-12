#include <Arduino.h>
#include <WiFi.h>
#include <WiFiUdp.h>
#include <esp_wifi.h>

// #include "peripherals.h"
// #include "credentials.h"

// Led redLed( GPIO_NUM_19 );
// Led greenLed( GPIO_NUM_21 );

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

void setup() {
    Serial.begin( 115200 );
    Serial.println( "Slave" );

    WiFi.begin();
    delay( 500 );
    ESP_ERROR_CHECK( esp_wifi_set_protocol( WIFI_IF_STA, WIFI_PROTOCOL_LR ) );
    WiFi.mode( WIFI_STA );

    udp.begin( 8888 );
}

void loop() {
    if ( WiFi.status() != WL_CONNECTED ) {
        Serial.println( "|" );
        int tries = 0;
        WiFi.begin( ssid, password );
        while( WiFi.status() != WL_CONNECTED ) {
            tries++;
            if ( tries == 5 )
                return;
            Serial.println( toStr( WiFi.status() ) );
//             greenLed.set();
            delay( 10 );
//             greenLed.reset();
            delay( 500 );
        }
        Serial.print( "Connected " );
        Serial.println( WiFi.localIP() );
    }
    int size = udp.parsePacket();
    if ( size == 0 )
        return;
    char c = udp.read();
    // if ( c == 'b' )
//         greenLed.invert();
    udp.flush();
}
