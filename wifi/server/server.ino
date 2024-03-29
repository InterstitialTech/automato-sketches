// Load Wi-Fi library
#include <WiFi.h>
#include <WiFiUdp.h>
#include <esp_wifi.h>
// Replace with your network credentials
const char* ssid     = "test6";
const char* password = "123456789";

WiFiUDP udp;

// Variable to store the HTTP request
String header;

void setup() {
  Serial.begin(115200);

  // Connect to Wi-Fi network with SSID and password
  Serial.print("Setting AP (Access Point)…");
  // Remove the password parameter, if you want the AP (Access Point) to be open
  WiFi.softAP(ssid, password);

  // AP means access point, which includes a dhcp server.
  // in LR mode, normal wifi devices will not be able to see the access point!
  ESP_ERROR_CHECK( esp_wifi_set_protocol( WIFI_IF_AP, WIFI_PROTOCOL_LR ) );

  // ESP_ERROR_CHECK( esp_wifi_set_protocol( ESP_IF_WIFI_AP,
  //                                         WIFI_PROTOCOL_11B |
  //                                         WIFI_PROTOCOL_11G |
  //                                         WIFI_PROTOCOL_11N
  //                                         ) );


  IPAddress IP = WiFi.softAPIP();
  Serial.print("AP IP address: ");
  Serial.println(IP);

  udp.begin(8888);
}

int count = 0;

void loop(){
    // delay(1000);
    IPAddress IP = WiFi.softAPIP();
    Serial.print("AP IP address: ");
    Serial.println(IP);
    int size = udp.parsePacket();
    if ( size == 0 )
        return;
    char c = udp.read();
    count++;
    Serial.print("received: ");
    Serial.print(count);
    Serial.print(" packets");
    udp.flush();
}

