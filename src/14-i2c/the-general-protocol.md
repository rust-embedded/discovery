# General protocol

The I2C protocol is more elaborate than the serial communication protocol because it has to support
communication between several devices. Let's see how it works using examples:

## Master -> Slave

If the master wants to send data to the slave:

<p align="center">
  <img class="white_bg" height=180 title="I2C bus" src="https://upload.wikimedia.org/wikipedia/commons/3/3e/I2C.svg">
</p>

1. Master: Broadcast START
2. M: Broadcast slave address (7 bits) + the R/W (8th) bit set to WRITE
3. Slave: Responds ACK (ACKnowledgement)
4. M: Send one byte
5. S: Responds ACK
6. Repeat steps 4 and 5 zero or more times
7. M: Broadcast STOP OR (broadcast RESTART and go back to (2))

> **NOTE** The slave address could have been 10 bits instead of 7 bits long. Nothing else would have
> changed.

## Master <- Slave

If the master wants to read data from the slave:

<p align="center">
<img class="white_bg" height=180 title="I2C bus" src="https://upload.wikimedia.org/wikipedia/commons/3/3e/I2C.svg">
</p>

1. M: Broadcast START
2. M: Broadcast slave address (7 bits) + the R/W (8th) bit set to READ
3. S: Responds with ACK
4. S: Send byte
5. M: Responds with ACK
6. Repeat steps 4 and 5 zero or more times
7. M: Broadcast STOP OR (broadcast RESTART and go back to (2))

> **NOTE** The slave address could have been 10 bits instead of 7 bits long. Nothing else would have
> changed.
