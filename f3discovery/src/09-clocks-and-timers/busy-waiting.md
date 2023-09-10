# 繁忙等待

现在应该正确初始化计时器。剩下的就是使用计时器实现`delay`功能。

我们要做的第一件事是设置自动释放寄存器 (`ARR`) ，使计时器在`ms`内关闭。
因为计数器在1 KHz下工作，所以自动减速值将与`ms`相同。

``` rust
    // Set the timer to go off in `ms` ticks
    // 1 tick = 1 ms
    tim6.arr.write(|w| w.arr().bits(ms));
```

接下来，我们需要启用计数器。它将立即开始计数。

``` rust
    // CEN: Enable the counter
    tim6.cr1.modify(|_, w| w.cen().set_bit());
```

现在我们需要等待，直到计数器达到自动释放寄存器的值`ms`，然后我们就知道`ms`已经过去了。 该条件称为*更新事件*，由状态寄存器
(`SR`)的`UIF`位指示。

``` rust
    // Wait until the alarm goes off (until the update event occurs)
    while !tim6.sr.read().uif().bit_is_set() {}
```

这种等待直到满足某个条件在这种情况下`UIF`变为`1`的模式被称为*繁忙等待*，您将在本文中多次看到`:-)`。

最后，我们必须清除 (设置为`0`) 这个`UIF`位。如果没有，下次我们进入`delay`函数时，我们会认为更新
事件已经发生，并跳过繁忙的等待部分。

``` rust
    // Clear the update event flag
    tim6.sr.modify(|_, w| w.uif().clear_bit());
```

现在，把所有这些放在一起，检查它是否按预期工作。
