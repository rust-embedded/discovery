# Linux

If you have a graphical Bluetooth manager, you can use that to pair your laptop
to the Bluetooth module and skip most of these steps. You'll probably still have
to [this step] though.

[this step]: 12-bluetooth-setup/linux.html#rfcomm%20device

## Power up

First, your laptop's Bluetooth transceiver may be OFF. Check its status with
`hciconfig` and turn it ON if necessary:

```
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

Then you need to launch the BlueZ (Bluetooth) daemon:

- On systemd based Linux distributions, use:

```
$ sudo systemctl start bluetooth
```

- On Ubuntu (or upstart based Linux distributions), use:

```
$ sudo /etc/init.d/bluetooth start
```

You may also need to unblock your Bluetooth, depending on what `rfkill list`
says:

```
$ rkfill list
9: hci0: Bluetooth
        Soft blocked: yes <--
        Hard blocked: no

$ sudo rfkill unblock bluetooth

$ rkfill list
9: hci0: Bluetooth
        Soft blocked: no <--
        Hard blocked: no

```

## Scan

```
$ hcitool scan
Scanning ...
        20:16:05:XX:XX:XX       Ferris
                                ^^^^^^
```

## Pair

```
$ bluetoothctl
[bluetooth]# scan on
[bluetooth]# agent on
[bluetooth]# pair 20:16:05:XX:XX:XX
Attempting to pair with 20:16:05:XX:XX:XX
[CHG] Device 20:16:05:XX:XX:XX Connected: yes
Request PIN code
[agent] Enter PIN code: 1234
```

## rfcomm device

We'll create a device file for our Bluetooth module in `/dev`. Then we'll be
able to use it just like we used `/dev/ttyUSB0`.

```
$ sudo rfcomm bind 0 20:16:05:XX:XX:XX
```

Because we used `0` as an argument to `bind`, `/dev/rfcomm0` will be the device
file assigned to our Bluetooth module.

You can release (destroy) the device file at any time with the following
command:

```
# Don't actually run this command right now
$ sudo rfcomm release 0
```
