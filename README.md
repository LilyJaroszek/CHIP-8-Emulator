# Chip-8-Emulator
A simple CHIP-8 emulator written in Rust that uses the command line for display output.
## Setup
Build the program source code. Add a folder named "roms" where the built executable is located. Add the CHIP-8 ROMs you want to run in the folder. You can now run the executable in the command line.
## Command Line Usage
chip-8_emulator \<ROM Name\> \<Flags (optional)\>
### Command Line Flags
-step: Start with instruction step mode on<br/>
-debug: Start with debug information on
## Controls
### Emulator Control Keys
Esc: Exit the emulator<br/>
Enter: Toggle instruction step mode<br/>
Down: Step to the next instruction<br/>
Tab: Toggle debug information<br/>
End: Dump the memory onto a file<br/>
### CHIP-8 Keys
1:'1' 2:'2' 3:'3' C:'4'<br/>
4:'q' 5:'w' 6:'e' D:'r'<br/>
7:'a' 8:'s' 9:'d' E:'f'<br/>
A:'z' 0:'x' B:'c' F:'v'<br/>
