<!-- # Spooky action at a distance -->

# 異なる場所での不気味な動作

<!-- 
`BSRR` is not the only register that can control the pins of Port E. The `ODR` register also lets
you change the value of the pins. Furthermore, `ODR` also lets you retrieve the current output
status of Port E.
 -->

ポートEのピンを制御できるレジスタは、`BSRR`だけではありません。`ODR`レジスタもピンの値を変更できます。
さらに、`ODR`を使って、ポートEの現在の出力状態を取得できます。

<!-- `ODR` is documented in: -->

`ODR`については、下記に書かれています。

> Section 11.4.6 GPIO port output data register - Page 239

<!-- Let's try this program: -->

次のプログラムを試してみましょう。

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

        // 北のLEDの（赤）を点灯
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

    unsafe {
        // A magic addresses!
        const GPIOE_BSRR: u32 = 0x4800_1018;

        // 東のLEDの（緑）を点灯
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        iprint_odr(&mut itm);

        // 北のLEDのを消灯
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        iprint_odr(&mut itm);

        // 東のLEDのを消灯
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
        iprint_odr(&mut itm);
    }

    loop {}
}
```

<!-- If you run this program, you'll see: -->

このプログラムを実行すると、次の出力が得られます。

``` console
$ # itmdump's console
(..)
ODR = 0x0000
ODR = 0x0200
ODR = 0x0a00
ODR = 0x0800
ODR = 0x0000
```

<!-- 
Side effects! Although we are reading the same address multiple times without actually modifying it,
we still see its value change every time `BSRR` is written to.
 -->

副作用！実際の値を変更することなしに、複数回同じアドレスを読み込んでいるにも関わらず、毎回`BSRR`に書き込んだ値に変化していることがわかります。
