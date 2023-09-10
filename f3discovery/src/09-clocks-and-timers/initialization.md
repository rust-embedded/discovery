# 初始化

与其他所有外设一样，我们必须先初始化这个计时器，然后才能使用它。正如前一节所述，初始化将涉及两个步骤：启动计时器，然后配置计时器。

给计时器加电很简单：我们只需将`TIM6EN`位设置为1。该位位于`RCC`寄存器块的`APB1ENR`寄存器中。

``` rust
    // Power on the TIM6 timer
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());
```

配置部分稍微复杂一些。

首先，我们必须将计时器配置为在单脉冲模式下运行。

``` rust
    // OPM Select one pulse mode
    // CEN Keep the counter disabled for now
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
```

然后，我们希望`CNT`计数器以1 KHz的频率工作，因为我们的`delay`函数以毫秒为自变量，1 KHz产生1毫秒的周期。
为此，我们必须配置预分频器。

``` rust
    // Configure the prescaler to have the counter operate at 1 KHz
    tim6.psc.write(|w| w.psc().bits(psc));
```

我会让你算出预分频器的值，`psc`。记住计数器的频率是`apb1 / (psc + 1)`并且`apb1`是 8 MHz。
