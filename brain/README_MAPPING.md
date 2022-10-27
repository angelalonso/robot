[#](#) How it works

- Move until something is found in radar   <------
- Get action(s) (e.g.: save value and move)       |
- Take action(s)                                  |
- Get result (e.g: something is found in radar) --

# The Limitations

- Each node is independent
  - We cannot create an instance of Status and call it from several nodes to get the same info
 
- Services would help on that but they clog CPU
  - We have to use messages/topics instead and desing the node asking for info separately from the node that handles that info
  - E.g.: 
    - Node A publishes to Topic 1 asking for Status
    - Our program sees message on Topic 1 and publishes Status on Topic 2
    - Node B is listening on Topic 2 and prints out the Status received

# Phases
## Create a base module on Rust managing State
- Basic State and associated functions - DONE
- Read a list of Actions coming from a yaml
- Have a loop to Apply them, based on State
  - First State to check is time, and first action to take is print out
