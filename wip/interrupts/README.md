# Interrupts

Challenge: LED roulette (you can drop *the trace*) + echo sever running
concurrently.

There are a few ways to do this. I'm going to show you how to do it using
*interrupts*.

An interrupt is when, given a certain *condition*, the processor stops doing
what's currently doing to execute a handler (a function). Once the processor
finishes executing the handler it resumes the routine it was executing.

Interrupts are very similar to exceptions in how they are executed but
interrupts are not triggered by fatal conditions, like invalid memory address.
Instead, interrupts are triggered by events or signals that are part of the
normal execution of a program.

Among the events that trigger interrupts we have: the update event of a timer;
new data is ready to be read from UART/I2C/SPI/etc.; "transmission complete",
the data that

Interrupts have their own "context of execution"; each interrupt has its own
stack frame (set of local variables) and can't access the stack frame of the function , they
are *like* threads.

To
We are going to put the LED roulette in a second "thread". These are not OS
level threads but an (each) interrupt has its own context of execution.

``` rust
#[export_name = "main"]
#[inline(never)]
pub fn main() -> ! {
    let usart1 = unsafe { peripheral::usart1_mut() };

    loop {
        while !usart1.isr.read().rxne() {}

        let byte = usart1.rdr.read().rdr() as u8;
        usart1.tdr.write(|w| w.tdr(u16::from(byte)));
    }
}

#[export_name = "_tim7"]
pub extern "C" fn interrupt_handler() {
    unsafe { bkpt!() }
}
```

```
(gdb) backtrace
#0  interrupts::interrupt_handler () at $PWD/src/main.rs:62
#1  <signal handler called>
#2  core::ptr::write_volatile<u32> (dst=0xe000e104, src=8388608)
    at $SYSROOT/lib/rustlib/src/rust/src/libcore/ptr.rs:260
#3  0x080005dc in volatile_register::RW<u32>::write<u32> (self=<optimized out>, value=<optimized out>)
    at $VOLATILE_REGISTER/src/lib.rs:78
#4  interrupts::init () at $PWD/src/main.rs:57
#5  0x080004ec in interrupts::main () at $PWD/src/main.rs:16
```
