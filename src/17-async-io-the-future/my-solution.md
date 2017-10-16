# My solution

``` rust
pub fn main() -> ! {
    let mut timer = Timer::new().unwrap();
    let Serial { mut tx, mut rx } = Serial::new().unwrap();

    let mut periodic = timer.periodic(100);
    let mut bytes = rx.bytes();
    let mut leds = LEDS.iter()
        .zip(LEDS.iter().skip(1))
        .chain(iter::once((&LEDS[7], &LEDS[0])))
        .cycle();

    loop {
        if let Async::Ready(()) = periodic.poll() {
            if let Some((current, next)) = leds.next() {
                current.off();
                next.on();
            }
        }

        if let Async::Ready(byte) = bytes.poll() {
            tx.write(byte).wait();
        }
    }
}
```
