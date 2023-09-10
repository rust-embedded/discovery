# 一般故障排除

## `cargo-embed`问题
大多数`cargo-embed`问题要么与未正确安装`udev`规则（在Linux上）有关，要么与在嵌入中选择了错误的芯片配置有关。
`Embed.toml`确保你两个都是对的。

如果上述方法不适用于您，您可以在[`discovery` issue tracker]打开问题。
或者，您也可以访问[Rust Embedded matrix channel]或者[probe-rs matrix channel]并在那里寻求帮助。

[`discovery` issue tracker]: https://github.com/rust-embedded/discovery/issues
[Rust Embedded matrix channel]: https://matrix.to/#/#rust-embedded:matrix.org
[probe-rs matrix channel]: https://matrix.to/#/#probe-rs:matrix.org

## Cargo 问题

### "找不到`core`crate"

#### 症状

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

#### 原因

您忘记为微控制器安装正确的目标(对于v2，`thumbv7em-none-eabihf`，对于v1`thumbv6m-none-eabi`)。

#### 修复

安装正确的目标。

``` console
# micro:bit v2
$ rustup target add thumbv7em-none-eabihf

# micro:bit v1
$ rustup target add thumbv6m-none-eabi
```
