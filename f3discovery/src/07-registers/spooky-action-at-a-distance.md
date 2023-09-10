# 超距作用

`BSRR`不是唯一一个可以控制端口E引脚的寄存器。`ODR`寄存器还允许您更改引脚的值。
`ODR`还允许您检索端口E的当前输出状态。

`ODR`记录在：

> 第11.4.6节GPIO端口输出数据寄存器-第239页

让我们看看这个程序。该程序的关键是`fn iprint_odr`。此函数将`ODR`中的当前值打印到`ITM`控制台

``` rust
#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprintln, ITM};

// Print the current contents of odr
fn iprint_odr(itm: &mut ITM) {
    const GPIOE_ODR: u32 = 0x4800_1014;

    unsafe {
        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );
    }
}

#[entry]
fn main() -> ! {
    let mut itm= aux7::init().0;

    unsafe {
        // A magic addresses!
        const GPIOE_BSRR: u32 = 0x4800_1018;

        // Print the initial contents of ODR
        iprint_odr(&mut itm);

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        iprint_odr(&mut itm);

        // Turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
        iprint_odr(&mut itm);

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        iprint_odr(&mut itm);

        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
        iprint_odr(&mut itm);
    }

    loop {}
}
```

如果运行此程序
```
$ cargo run
(..)
Breakpoint 1, registers::__cortex_m_rt_main_trampoline () at src/07-registers/src/main.rs:22
22      #[entry]

(gdb) continue
Continuing.
```

您将在itmdump的控制台上看到：

``` console
$ # itmdump's console
(..)
ODR = 0x0000
ODR = 0x0200
ODR = 0x0a00
ODR = 0x0800
ODR = 0x0000
```

副作用！尽管我们多次读取同一地址，但实际上并未对其进行修改，但每次写入`BSRR`时，我们仍会看到其值发生变化。
