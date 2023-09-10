# Linux

如果您有图形蓝牙管理器，您可以使用它将计算机与蓝牙模块配对，并跳过大部分步骤。不过，您可能仍需要执行[此步骤]。

[此步骤]: #rfcomm-device

## 通电

首先，计算机的蓝牙收发器可能处于关闭状态。使用`hciconfig`检查其状态，必要时将其打开：

``` console
$ hciconfig
hci0:   Type: Primary  Bus: USB
        BD Address: 68:17:29:XX:XX:XX  ACL MTU: 310:10  SCO MTU: 64:8
        DOWN  <--
        RX bytes:580 acl:0 sco:0 events:31 errors:0
        TX bytes:368 acl:0 sco:0 commands:30 errors:0

$ sudo hciconfig hci0 up

$ hciconfig
hci0:   Type: Primary  Bus: USB
        BD Address: 68:17:29:XX:XX:XX  ACL MTU: 310:10  SCO MTU: 64:8
        UP RUNNING  <--
        RX bytes:1190 acl:0 sco:0 events:67 errors:0
        TX bytes:1072 acl:0 sco:0 commands:66 errors:0
```

然后，您需要启动BlueZ (蓝牙) 守护程序：

- 在基于systemd的Linux发行版上，请使用：

``` console
$ sudo systemctl start bluetooth
```

- 在Ubuntu (或基于新的Linux发行版)上，使用：

``` console
$ sudo /etc/init.d/bluetooth start
```

您可能还需要解锁蓝牙，具体取决于`rfkill list`列表中的内容：

``` console
$ rfkill list
9: hci0: Bluetooth
        Soft blocked: yes # <--
        Hard blocked: no

$ sudo rfkill unblock bluetooth

$ rfkill list
9: hci0: Bluetooth
        Soft blocked: no  # <--
        Hard blocked: no

```

## 扫描

``` console
$ hcitool scan
Scanning ...
        20:16:05:XX:XX:XX       Ferris
$ #                             ^^^^^^
```

## 配对

``` console
$ bluetoothctl
[bluetooth]# scan on
[bluetooth]# agent on
[bluetooth]# pair 20:16:05:XX:XX:XX
Attempting to pair with 20:16:05:XX:XX:XX
[CHG] Device 20:16:05:XX:XX:XX Connected: yes
Request PIN code
[agent] Enter PIN code: 1234
```

## 射频通信设备

我们将在`/dev`中为蓝牙模块创建一个设备文件。然后我们就可以像使用`/dev/ttyUSB0`一样使用它了。

``` console
$ sudo rfcomm bind 0 20:16:05:XX:XX:XX
```

因为我们使用`0`作为`bind`的参数，所以`/dev/rfcomm0`将是分配给蓝牙模块的设备文件。

您可以随时使用以下命令释放（销毁）设备文件：

``` console
$ # Don't actually run this command right now!
$ sudo rfcomm release 0
```
