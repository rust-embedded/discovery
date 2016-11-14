# Windows tooling

Before plugging the Serial module, run the following command on the terminal:

```
$ mode
```

It will print a list of devices that are connected to your laptop. The ones that
start with `COM` in their names are serial devices. This is the kind of device
we'll be working with. Take note of all the `COM` devices `mode` outputs
*before* plugging the serial module.

Now, plug the Serial module and run the `mode` command again. You should see a
new `COM` command appear on the list. That's the COM port assigned to the serial
module.

Now launch `putty`. A GUI will pop out.

<p align="center">
<img title="PuTTY settings" src="assets/putty-settings.png">
</p>

On the default screen, "Session", pick "Serial" as the "Connection type". On the
"Serial line" field enter the `COM` device you got on the previous step, for
example `COM3`.

Next, pick the "Connection/Serial" menu item from the menu on the left. On this
new view, make sure that the serial port is configured as follows:

- "Speed (baud)": 115200
- "Data bits": 8
- "Stop bits": 1
- "Parity": None
- "Flow control": None

Finally, click the Open button. A console will show up now:

<p align="center">
<img title="PuTTY console" src="assets/putty-console.png">
</p>

If you type on this console, the TX (red) LED on the Serial module should blink.
Each key stroke should make the LED blink once. Note that the console won't echo
back what you type so the screen will remain blank.
