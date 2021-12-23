sudo apt-get update  
sudo apt-get install -y vim git pkg-config libudev1 libudev-dev bison flex 
wget http://download.savannah.gnu.org/releases/avrdude/avrdude-6.2.tar.gz  
tar xfv avrdude-6.2.tar.gz  
cd avrdude-6.2/  
./configure --enable-linuxgpio  
make  
sudo make install  

export RUSTUP_UNPACK_RAM=200000000  
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  


echo REMEMBER TO DO THIS NOW:
echo "sudo vim /usr/local/etc/avrdude.conf"  
echo   
echo 'programmer'  
echo '  id    = "linuxgpio";'  
echo '  desc  = "Use the Linux sysfs interface to bitbang GPIO lines";'  
echo '  type  = "linuxgpio";'  
echo '  reset = 4;'  
echo '  sck   = 11;'  
echo '  mosi  = 10;'  
echo '  miso  = 9;'  
echo ';'  


