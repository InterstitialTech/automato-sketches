# automato 0.1.2 'brain board' installation, and compiling an example.
#
# Not a script, just the commands you'll probably need starting from
# a fresh linux system.

# Step 0: install arduino-cli.

# init a config file:
arduino-cli config init

# add additional urls:
[nix-shell:~/code/arduino]$ cat ~/.arduino15/arduino-cli.yaml
board_manager:
    additional_urls:
        - https://raw.githubusercontent.com/InterstitialTech/automato-arduino/refs/heads/main/package_automato_index.json
        - https://espressif.github.io/arduino-esp32/package_esp32_index.json

# update arduino libs/board list etc.
arduino-cli core update-index

# list attached boards.
arduino-cli board list

# what packages installed.
arduino-cli core list

# search for available packages.
arduino-cli core search auto

# install esp32 3.0.3
arduino-cli core install esp32:esp32

# install automato esp32
arduino-cli core install automato:esp32

# clone automato library into ~/Arduino/libraries
git clone git@github.com:InterstitialTech/automato-library.git
mkdir ~/Arduino/libraries -p
mv automato-library/ ~/Arduino/libraries/

# link to arduino work dir for convenience. 
ln -s ~/Arduino/libraries/automato-library/ .

# install depenedencies
arduino-cli lib install "Adafruit GFX Library"
arduino-cli lib install "Adafruit ILI9341"
arduino-cli lib install "SparkFun SHTC3 Humidity and Temperature Sensor Library"
arduino-cli lib install "SparkFun SHTC3 Humidity and Temperature Sensor Library"
arduino-cli compile -b automato:esp32:automato_brain automato-library/examples/EspNowRecv/
wget http://www.airspayce.com/mikem/arduino/RadioHead/RadioHead-1.120.zip
mv RadioHead-1.120.zip ~/Arduino/libraries/
unzip ~/Arduino/libraries/RadioHead-1.120.zip -d ~/Arduino/libraries/

# IMPORTANT for 
# in nixos add your user to the "dialup" group, log out and log back in to access /dev/ttyACM0.
# don't use sudo because then arduino_cli will look for /home/root/Arduino and /home/root/.arduino15

# upload sketch to automato.
arduino-cli upload automato-library/examples/EspNowRecv/ -p /dev/ttyACM0 -b automato:esp32:automato_brain
