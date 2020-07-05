# Robot

Resources to build a home robot.


# WHY this guide?
- I have very specific Hardware, most notable an <old> Raspberry pi B.  
- Most of the docs online refer to the newer boards.  
- Everything I document here was either not-so-easy to find or directly a result of my trial and error.  

# Status

Under development

# PHASE 1: Connect Raspberry to Arduino
## Raspberry Pi Model B GPIO map

    5v|3.3v  
    5v|2 SDA  
   GND|3 SCL  
14 TXD|4  
15 RXD|GND  
    18|17  
   GND|27  
    23|22  
    24|3.3v  
   GND|10 MOSI  
    25|9 MISO  
     8|11 SCKL  
     7|GND  

## Arduino clone "ISCP" Map - yeah I know, freaking clones' typos

5|3|1  
6|4|2  
  
1 MISO  
2 Vcc  
3 ICSP SCK  
4 ICSP MOSI  
5 ICSP RESET  
6 ICSP GND  

## Connection
1 -> 21  
2 -> 2  
3 -> 23  
4 -> 19  
5 -> 7  
6 -> 6  

# PHASE 2: install and configure avrdude on Raspberry
sudo apt-get update  
sudo apt-get install bison flex -y  
wget http://download.savannah.gnu.org/releases/avrdude/avrdude-6.2.tar.gz  
tar xfv avrdude-6.2.tar.gz  
cd avrdude-6.2/  
./configure --enable-linuxgpio  
make  
sudo make install  
sudo vim /usr/local/etc/avrdude.conf  
  
```programmer  
  id    = "linuxgpio";  
  desc  = "Use the Linux sysfs interface to bitbang GPIO lines";  
  type  = "linuxgpio";  
  reset = 4;  
  sck   = 11;  
  mosi  = 10;  
  miso  = 9;  
;  
```
  
sudo avrdude -c linuxgpio -p atmega328p -v  

# PHASE 3: prepare Raspberry to run rust
export RUSTUP_UNPACK_RAM=200000000  
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  

