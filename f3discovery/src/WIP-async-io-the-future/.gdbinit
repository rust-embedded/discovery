target remote :3333
set print asm-demangle on
load
monitor tpiu config internal itm.txt uart off 8000000
break main
continue
