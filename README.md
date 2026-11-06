# ACC-TEST #

## What is this? ##

grip-logic is a simple program written in Rust which allows you to automate changing TC and BB settings throughout the lap.

## How does it work? ##

The program utilizes ACC's API (shared memory region) to get the current track position as well as current brake-balance and uses a virtual keyboard that comes with Logitech G-Hub to automate keybind presses for specific TC levels and BB adjustments. **IMPORTANT**: YOU MUST HAVE LOGITECH G-HUB INSTALLED ON YOU PC, IT DOESNT HAVE TO BE RUNNING AND YOU DO NOT NEED ANY LOGITECH GEAR, JUST THE PROGRAM INSTALLED

## How to use? ##

To build from source use: `cargo build --release`.  

## 3rd party ##

The InputSimulator.dll is courtesy of @Chaoses-Ib <https://github.com/Chaoses-Ib/IbInputSimulator>

## Known issues ##

- The program crashes when `Crew Chief` is ran before it.  
- You have to have brake balance inc/dec bound to something for traction control to work (dont ask why, im just retarded)
