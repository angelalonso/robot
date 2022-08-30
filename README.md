# Robot

A step by step Guide to building and configuring your own robot using partially my choice of Hardware and Software.

## Motivation

- I was looking for a project to learn that would have me entertained for longer than a month.
- As a kid I loved the idea of having a Robot at home one day.
- That robot had to be configurable with only two limitations: pocket and knowledge 

## Status

Right now, It does not work 

There used to be a working thing written completely on Rust but it didn't scale well.
It was redone from scratch, to use ROS2 instead, but somehow interfaces on python were too CPU-intensive.
A minimal example of a ROS library was created as a PoC that Rust Libraries would work ok

Now the next goal is to make everything come together as follows:
- :wrench: Auto boot
  - :+1: python3.9
  - :bulb: env/bin/activate: No such file or directory
  - :bulb: maturin 
```
⚠️  Warning: the following metadata fields in `package.metadata.maturin` section of Cargo.toml are deprecated and will be removed in future versions: classifiers, please set them in pyproject.toml as PEP 621 specifies.
💥 maturin failed
  Caused by: Cargo metadata failed. Does your crate compile with `cargo build`?
  Caused by: `cargo metadata` exited with an error:     Updating crates.io index
 Downloading crates ...
error: failed to download from `https://crates.io/api/v1/crates/autocfg/1.1.0/download`
```
  - :bulb: configure raspberry automatically
    - :bulb: Install raspbian, then document what file to copy and where
    - :bulb: LED shows Step the run is in, and another pattern when ready
  - :bulb: Automate latest from master
    - :wrench: Use Rust library found at standard path - find generated file and link to it?
    - :wrench: Create buildmin mode that does not build Rust parts
  - :wrench: Install an LED
    - :+1: Make LED work on test computer
    - :+1: Install an LED physically
    - :+1: Make LED work on Robot
  - :bulb: Show status through this LED
  - :bulb: Get a working API 
    - :bulb: Make sure this uses SSL and a Device ID to only allow calls from a set of given devices.
    - :bulb: The list of devices must go in a protected file
    - :bulb: Automate as much as possible this process
  - :bulb: Show status through a call to this API
- :bulb: Mode
  - :bulb: Available modes:
    - :bulb: RC - actions can be controlled from an Android App that uses the API
    - :bulb: Callibrating - goes through a set of tests to "learn from itself" (e.g.: how much power and time it needs on each wheel to turn 30 degrees clockwise)
    - :bulb: Mapping - Moves around a space to build a map of the surroundings
    - :bulb: Auto - uses mapping to move around among other actions
      - :bulb: This mode can also override actions if API says so
  - :bulb: The default mode is set on a file
  - :bulb: The mode itself can be overridden on the go via API


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


