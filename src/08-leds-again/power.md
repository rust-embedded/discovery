<!-- # Power -->

# 電源

<!-- 
Turns out that, to save power, most peripherals start in a powered off state -- that's their state
right after the microcontroller boots.
 -->

電力を節約するために、ほとんどのペリフェラルは、電源が入っていない状態で起動します。
これが、マイクロコントローラが起動した直後のペリフェラルの状態です。

<!-- 
The Reset and Clock Control (`RCC`) peripheral can be used to power on or off every other
peripheral.
 -->

リセットとクロック制御（`RCC`）ペリフェラルは、全ての他のペリフェラルの電源をオン/オフするために使います。

<!-- You can find the list of registers in the `RCC` register block in: -->

`RCC`レジスタブロックのレジスタリストは、下記にあります。

> Section 9.4.14 - RCC register map - Page 166 - Reference Manual

<!-- The registers that control the power status of other peripherals are: -->

他のペリフェラルの電源状態を制御するレジスタには、次のものがあります。

- `AHBENR`
- `APB1ENR`
- `APB2ENR`

<!-- Each bit in these registers controls the power status of a single peripheral, including `GPIOE`. -->

これらのレジスタの各ビットは、1つのペリフェラルの電源状態を制御します。もちろん`GPIOE`も含まれています。

<!-- Your task in this section is to power on the `GPIOE` peripheral. You'll have to: -->

このセクションでのあなたの仕事は、`GPIOE`ペリフェラルの電源を入れることです。あなたは次のことに取り組む必要があります。

<!-- 
- Figure out which of the three registers I mentioned before has the bit that controls the power
  status.
- Figure out what value that bit must be set to,`0` or `1`, to power on the `GPIOE` peripheral.
- Finally, you'll have to change the starter code to *modify* the right register to turn on the
  `GPIOE` peripheral.
 -->

- 上述した3つのレジスタのうち、どのレジスタが電源状態を制御するビットを持つか、調べて下さい。
- `GPIOE`ペリフェラルの電源を入れるために、ビットを`0`か`1`の、どちらにしなければならないか、調べて下さい。
- 最後に、`GPIOE`ペリフェラルの電源を入れるために、正しいレジスタを*modify*するようにスターターコードを変更する必要があります。

<!-- 
If you are successful, you'll see that the `gpioe.odr.write` statement will now be able to modify
the value of the `ODR` register.
 -->

うまくいくと、`gpioe.odr.write`ステートメントが`ODR`レジスタの値を修正するようになります。

<!-- Note that this won't be enough to actually turn on the LEDs. -->

LEDを実際に点灯するには不十分であることに、留意して下さい。
