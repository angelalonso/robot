# Robot

A step by step Guide on how I build, configure and run my own robot using a specific choice of Hardware and Software.

## Can I run it myself? How?

Yeah. You just need to be familiar with troubleshooting on linux and searching the web for resources :) 
Everything software-related I use is included here, but neither properly sorted nor documented.
Some thing you may want to check are:
- [How I put together the current Chassis](https://github.com/angelalonso/robot/blob/master/docs/000_Base_Chassis.md)
- [Where I originally got my HW from](https://github.com/angelalonso/robot/blob/master/docs/000_Base_ShoppingList.md)
- [How to configure the Raspberry](https://github.com/angelalonso/robot/blob/master/docs/000_Base_Raspberry_raspbian.md)
 

## Notes on why I do this

- I was looking for a project to learn that would have me entertained for longer than a month.
- As a kid I loved the idea of having a Robot at home one day.
- That robot had to be configurable with only two limitations: pocket and knowledge 

## Notes on the status

Right now, It can manage two motors through an API. 
- The code to run on the Raspberry is on /circuits
  - You can use curl to send calls to the API
  - There is a very primitive Android app on /AndroidApp that helps with that, but you probably need to open ports on the RAspberry (not yet tested).
  - Still, running off a battery is not yet tested.
  - I need to also work on a cross-compiling system

## Notes on my choice of Software

- Full Rust:
  - There used to be a working thing written completely on Rust but it didn't scale well.
- ROS2 - Python
  - The project was redone from scratch, to use ROS2 and python instead, but interfaces on python were too CPU-intensive.
  - I redid the interfaces' functionality on Rust and added that as a python library. Then I realized having Python code would be too slow
- ROS2 - C++:
  - After realizing Python would be too slow, I started redoing everything on ROS's C++
  - When I had something working, I also realized compiling it on the Raspberry would be too heavy, so I started introducing cross-compilation.
  - Then I tried to introduce libraries on Rust that would be called from C++. I never got it to work using ROS's cmakelists

So my next approach is a redesign, again based off a full-Rust version that tries to mimc ROS2's architecture principles (e.g.: nodes, actions, messages...), with the hope that one day there will be a full-on ROS2-Rust library that I can easily migrate to.

I need to make the following work to consider this approach "usable":
- DONE - Create a 'Node' that lives until a CTRL-C is received
- Make that Node work like an action server
- DONE - Create a second Node that works like an action client
- DONE - Have the action server turn an LED on and off
- Make this run on the Raspberry and check load
- Cross compile to save time -> not working, neither with --target nor with docker, we'll try qemu

Once that is working, I will adapt the code and auxiliar scripts to keep using what worked well in the past:

- Autosetup of the Raspberry 
- Build pipeline
- DONE .env configuration

Then I will try to extend to the rest of functionality:
- Engines
- Control through a secured API
- Status advertised on LED
- Automatic mode
- Autoboot

## Contribute and Credits

Just PM me on Github, I haven't figured this out yet.

## License

GNU General Public License v3.0

## NEXT UP:
### Must have
- Cross compiling/building
### Should have
- Modules mimic ROS2 (E.g.: instead of comms, something like actions that wraps comms)
- "Autonomous" run with List of actions to do
### Would be good to have
- Completely autonomous run mode


# TBD
- Document:
  - Raspbian
    - Auto install with copy over of .env
