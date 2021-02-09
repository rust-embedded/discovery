# Connect to gdb remote server
target remote :3333

# Load will flash the code
load

# Eanble demangling asm names on disassembly
set print asm-demangle on

# Enable pretty printing
set print pretty on

# Disable style sources as the default colors can be hard to read
set style sources off

# Have the tpiu send the data to a file tim.txt
monitor tpiu config internal itm.txt uart off 8000000

# Turn on the itm port
monitor itm port 0 on

# Set a breakpoint at main
break main

# Continue running and we'll hit the main breakpoint
continue
