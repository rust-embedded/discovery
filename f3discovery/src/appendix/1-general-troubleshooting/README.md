# 一般故障排除

## OpenOCD 问题

### 无法连接到OpenOCD - "Error: open failed"

#### 症状

尝试与设备建立*新连接*时，会出现如下错误：

```
$ openocd -f (..)
(..)
Error: open failed
in procedure 'init'
in procedure 'ocd_bouncer'
```

#### 原因

设备未（正确）连接或未使用正确的ST-LINK接口配置。

#### 修复

Linux:

- 使用`lsusb`检查USB连接。
- 您可能没有足够的权限打开设备。请使用`sudo`重试。如果这有效，您可以使用[这些指令]使OpenOCD在没有root权限的情况下工作。
- 您可能使用了错误的ST-LINK接口配置。尝试使用`interface/stlink-v2.cfg`而不是`interface/stlink-v2-1.cfg`。

[这些指令]: ../../03-setup/linux.md#udev-rules

Windows:

- 您可能缺少ST-LINK USB驱动程序。[此处]的安装说明。

[此处]: ../../03-setup/windows.md#st-link-usb-driver

### 无法连接到OpenOCD - "Polling again in X00ms"

#### 症状

尝试与设备建立*新连接*时，会出现如下错误：

```
$ openocd -f (..)
(..)
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 100ms
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 300ms
Info : Previous state query failed, trying to reconnect
```

#### 原因

微控制器可能卡在某个紧密的无限循环中，或者持续引发异常，例如异常处理程序正在引发异常。

#### 修复

- 关闭OpenOCD（如果正在运行）
- 按住重置（黑色）按钮
- 启动OpenOCD命令
- 现在，松开复位按钮


### OpenOCD连接丢失 - "Polling again in X00ms"

#### 症状

*正在运行*OpenOCD会话突然出现错误：

```
# openocd -f (..)
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 100ms
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f3x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 300ms
Info : Previous state query failed, trying to reconnect
```

#### 原因

USB连接丢失。

#### 修复

- 关闭OpenOCD
- 断开并重新连接USB电缆。
- 重新启动OpenOCD

### 无法刷新设备 - "Ignoring packet error, continuing..."

#### 症状

在刷新设备时，您可以：

```
$ arm-none-eabi-gdb $file
Start address 0x8000194, load size 31588
Transfer rate: 22 KB/sec, 5264 bytes/write.
Ignoring packet error, continuing...
Ignoring packet error, continuing...
```

#### 原因

在运行"打印"到ITM的程序时关闭`itmdump`。当前GDB会话看起来正常工作，只是没有ITM输出，
但下一个GDB会话将出现上一节中显示的消息错误。

或者，在发出`monitor tpiu`之后调用`itmdump`，从而使`itmdump`删除OpenOCD正在写入的文件/命名管道。

#### 修复

- 关闭/杀死GDB，OpenOCD和`itmdump`。
- 删除`itmdump`正在使用的文件/命名管道 (例如，`itm.txt`)。
- 启动OpenOCD。
- 然后，启动`itmdump`。
- 然后，启动执行`monitor tpiu`命令的GDB会话。


### 无法连接到OpenOCD - "Error: couldn't bind [telnet] to socket: Address already in use"

#### 症状

尝试与设备建立*新连接*时，会出现如下错误：

```
$ openocd -f (..)
(..)
Error: couldn't bind telnet to socket: Address already in use
```

#### 原因

OpenOCD需要访问的一个或多个端口3333、4444或6666正在被另一个进程使用。
每个端口用于另一个方面：3333用于gdb，4444用于telnet，6666用于TCL的远程过程调用（RPC）命令

#### 修复

你可以两种方法来解决这个问题 
A）杀死使用这些端口之一的任何进程。 
B) 指定OpenOCD可以免费使用的不同端口。

解决方案 A

Mac:
- 通过运行`sudo lsof -PiTCP -sTCP:LISTEN`获取使用端口的进程列表
- 通过记录进程的pid并为每个进程运行`kill [pid]`来终止阻塞关键端口的进程。（假设您可以确认他们没有在您的机器上运行任何关键任务！）

解决方案 B

All:
- 启动OpenOCD时，将配置详细信息发送给OpenOCD，以便它使用与任何进程默认端口不同的端口。
- 例如，要在4441而不是默认的4444上执行其telnet功能，需要运行`openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg -c "telnet_port 4441"`
- 有关OpenOCD配置阶段的更多详细信息，请参见他们的[在线官方文档](http://openocd.org/doc/html/Server-Configuration.html)。


## Cargo 问题

### "找不到`core` crate"

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

您使用的工具链早于`nightly-2018-04-08`并忘记调用`rustup target add thumbv7em-none-eabihf`。

#### 修复

更新并安装`thumbv7em-none-eabihf` target。

``` console
$ rustup update nightly

$ rustup target add thumbv7em-none-eabihf
```
