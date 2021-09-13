# General troubleshooting

## `cargo-embed` problems
Most `cargo-embed` problems are either related to not having installed the `udev`
rules properly (on Linux) or having selected the wrong chip configuration in `Embed.toml` so
make sure you got both of those right.

If the above does not work out for you, you can open an issue in the [`discovery` issue tracker].
Alternatively you can also visit the [Rust Embedded matrix channel] or the [probe-rs matrix channel]
and ask for help there.

[`discovery` issue tracker]: https://github.com/rust-embedded/discovery/issues
[Rust Embedded matrix channel]: https://matrix.to/#/#rust-embedded:matrix.org
[probe-rs matrix channel]: https://matrix.to/#/#probe-rs:matrix.org

## Cargo problems

### "can't find crate for `core`"

#### Symptoms

```
   Compiling volatile-register v0.1.2
   Compiling rlibc v1.0.0
   Compiling r0 v0.1.0
error[E0463]: can't find crate for `core`

error: aborting due to previous error

error[E0463]: can't find crate for `core`

error: aborting due to previous error

error[E0463]: can't find crate for `core`

error: aborting due to previous error

Build failed, waiting for other jobs to finish...
Build failed, waiting for other jobs to finish...
error: Could not compile `r0`.

To learn more, run the command again with --verbose.
```

#### Cause

You forgot to install the proper target for your microcontroller (`thumbv7em-none-eabihf` for v2
and `thumbv6m-none-eabi` for v1).

#### Fix

Install the proper target.

``` console
# micro:bit v2
$ rustup target add thumbv7em-none-eabihf

# micro:bit v1
$ rustup target add thumbv6m-none-eabi
```
