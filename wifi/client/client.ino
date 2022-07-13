#include <Arduino.h>
#include <WiFi.h>
#include <WiFiUdp.h>

#include <esp_wifi.h>

const char* ssid     = "test6";
const char* password = "123456789";

WiFiUDP udp;

int status = WL_IDLE_STATUS;                     // the Wifi radio's status

const char *showWlStatus( wl_status_t status ) {
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
    // WiFi.begin(ssid, password);
    // udp.begin( 8888 );
  // attempt to connect to Wifi network:
  while ( status != WL_CONNECTED) {
    Serial.print("Attempting to connect to WEP network, SSID: ");
    Serial.println(ssid);
    status = WiFi.begin(ssid, password);
  ESP_ERROR_CHECK( esp_wifi_set_protocol( ESP_IF_WIFI_STA,
                                          WIFI_PROTOCOL_11B | 
                                          WIFI_PROTOCOL_11G |
                                          WIFI_PROTOCOL_11N 
                                          ) );    


    Serial.print("status: ");
    Serial.print(status);
    Serial.print("status string: ");

    Serial.println(showWlStatus(status));

    // wait 10 seconds for connection:
    delay(3000);
  }

  // once you are connected :
  Serial.print("You're connected to the network");

}

void loop() {
    Serial.print("wifi status:");
    Serial.println(WiFi.status());
    delay( 1000 );
    Serial.println("sending packet:");
    udp.beginPacket( { 192, 168, 4, 1 }, 8888 );
    udp.write( 'b' );
    udp.endPacket();
}
