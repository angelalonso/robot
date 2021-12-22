# Robot

A step by step Guide to building and configuring your own robot using partially my choice of Hardware and Software.

## Motivation

- I was looking for a project to learn that would have me entertained for longer than a month.
- As a kid I loved the idea of having a Robot at home one day.
- That robot had to be configurable with only two limitations: pocket and knowledge 

## Status

Redone from scratch, to change two philosophical points:
- This time around I am using ROS2 instead of my own Rust-based program to make it more scalable and standardized
- I am building the robot and its documentation as a Project in several phases:
  - The base project, which just moves two motors based on a predefined set of actions
  - Additional "plugins" that anyone could add to the base project. E.g.: Inputs such as distance sensor, outputs such as an LED...

## HOW TO

- [x] Base Project: 
  - [[Buy your Hardware]](docs/000_Base_ShoppingList.md)
  - [[Connect everything to the Chassis]](docs/000_Base_Chassis.md) (To be reviewed)
  - [[Set up the Raspberry Pi]](docs/000_Base_Raspberry.md) (To be reviewed)
- [x] Plugin: Input distance sensor. TBD

## Screenshots

To be done

## Code Style

I am using ROS2 with Python mostly (the base project is declared as a C++ one, because some ROS2 stuff would not work for python projects). 
Before this project, I had never used ROS in my life. and I consider myself a Python middle-range user (I cannot explain why stuf works like they do, but with enough access to Stackoverflow and time I can do anything on Python)

I am also looking for a way to have Rust added to the formula.

## Built with

- [ROS2](https://docs.ros.org/en/foxy/index.html)
- [Bash](https://tiswww.case.edu/php/chet/bash/bashtop.html)
  - [Bash autocompletion](https://www.gnu.org/software/bash/manual/html_node/Programmable-Completion.html)
- [Raspberry Pi](https://www.raspberrypi.org/)
- [Arduino](https://www.arduino.cc/)

Consider the above also a list of minimum requirements.

## Contribute and Credits

Just PM me on Github, I haven't figured this out yet.

## License

GNU General Public License v3.0


