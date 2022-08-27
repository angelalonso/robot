# Robot

A step by step Guide to building and configuring your own robot using partially my choice of Hardware and Software.

## Motivation

- I was looking for a project to learn that would have me entertained for longer than a month.
- As a kid I loved the idea of having a Robot at home one day.
- That robot had to be configurable with only two limitations: pocket and knowledge 

## Status

It does not work "as is"

There used to be a working thing written completely on Rust but it didn't scale well.
It was redone from scratch, to use ROS2 instead, but somehow interfaces on python were too CPU-intensive.
A minimal example of a ROS library was created as a PoC that Rust Libraries would work ok

Now the next goal is to make everything come together. 

| :zap:        Current WIP is making averything come together, see thirdphase branch |
|------------------------------------------------------------------------------------|

| :exclamation:  Consider everything from here on as a Work in Progress |
|-----------------------------------------------------------------------|

## HOW TO

- [x] Base Project: 
  - [[Buy your Hardware]](docs/000_Base_ShoppingList.md)
  - [[Connect everything to the Chassis]](docs/000_Base_Chassis.md) (To be reviewed)
  - [[Set up the Raspberry Pi]](docs/000_Base_Raspberry.md) (To be reviewed)
  - [[Configure and start it up]](docs/000_Base_Software.md) (To be reviewed)
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
- [Rust](https://www.rust-lang.org/)

Consider the above also a list of minimum requirements.

## Contribute and Credits

Just PM me on Github, I haven't figured this out yet.

## License

GNU General Public License v3.0


