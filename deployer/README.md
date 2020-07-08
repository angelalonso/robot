# Deployer tool 
Deployment tool for the following setup:

a) We develop on a PC
b) We have Raspberry Pi, accessible through SSH, where all code is deployed to.
b.1) This includes arduino code, which the Raspberry will push to the Arduino itself as it needs to

## Requirements
On your PC:
- arduino-cli https://arduino.github.io/arduino-cli/installation/

On the Raspberry:
- The Controller application (TO BE DONE)
- A user with access to sudo and without a password for that (see https://phpraxis.wordpress.com/2016/09/27/enable-sudo-without-password-in-ubuntudebian/)


## WISHLIST
- Avoid the need to have a user with sudo access that needs no password
