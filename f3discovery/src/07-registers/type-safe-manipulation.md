# 类型安全操作

我们使用的最后一个寄存器`ODR`在其文档中有这样的内容：

> Bits 31:16保留，必须保持在重置值

我们不应该写入寄存器中的那些位，否则可能会发生糟糕的事情。

此外，寄存器具有不同的读/写权限。其中一些是只读的，其他的可以读写，必须有其他的是只读的。

最后，直接使用十六进制地址容易出错。您已经看到，尝试访问无效的内存地址会导致异常，从而中断程序的执行。

如果我们有一个API来以"safe"的方式操作寄存器，这不是很好吗？理想情况下，API应该对我提到的这三点
进行编码：不要乱用实际地址，应该尊重读/写权限，并且应该防止修改寄存器的保留部分。

好吧，我们做到了！`aux7::init()`实际上返回一个值，该值提供了一个类型安全API来操作`GPIOE`外设的寄存器。

您可能还记得：与外围设备相关联的一组寄存器称为寄存器块，它位于内存的连续区域中。在这种类型安全的API中，
每个寄存器块都被建模为一个`struct`，其中每个字段代表一个寄存器。每个寄存器字段都是不同的新类型，例如`u32`
它公开了以下方法的组合：根据其读/写权限进行`read`，`write`或`modify`。最后，这些方法不采用像`u32`这样的原始值，而是
采用另一种新类型，该类型可以使用构建器模式构建，并防止修改寄存器的保留部分。

熟悉此API的最佳方法是将我们的运行示例移植到它。

``` rust
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprintln, ITM, RegisterBlock};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    // Turn on the North LED
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    // Turn on the East LED
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    // Turn off the North LED
    gpioe.bsrr.write(|w| w.br9().set_bit());

    // Turn off the East LED
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}
```

你注意到的第一件事是：没有魔法地址。相反，我们使用更人性化的方式，例如`gpioe.bsrr`，以引用`GPIOE`寄存器块中的`BSRR`寄存器。

然后我们有一个`write`方法，它需要一个闭包。如果使用标识闭包 (`|w| w`)，则此方法将寄存器设置为其*默认*（重置）值，
即微控制器通电/重置后的值。对于`BSRR`寄存器，该值为`0x0`。由于我们想向寄存器中写入一个非零值，
所以我们使用`bs9`和`br9`等构建器方法来设置默认值的一些位。

让我们运行这个程序！在调试程序时，我们可以做*一些*有趣的事情。

`gpioe`是对`GPIOE`寄存器块的引用。`print gpioe`将返回寄存器块的基址。

```
$ cargo run
(..)

Breakpoint 1, registers::__cortex_m_rt_main_trampoline () at src/07-registers/src/main.rs:7
7       #[entry]

(gdb) step
registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:9
9           let gpioe = aux7::init().1;

(gdb) next
12          gpioe.bsrr.write(|w| w.bs9().set_bit());

(gdb) print gpioe
$1 = (*mut stm32f3::stm32f303::gpioc::RegisterBlock) 0x48001000
```

但是如果我们改为 `print *gpioe`，我们将得到寄存器块的*完整视图*：它的每个寄存器的值都将被打印出来。

```
(gdb) print *gpioe
$2 = stm32f3::stm32f303::gpioc::RegisterBlock {
  moder: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_MODER> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 1431633920
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_MODER>
  },
  otyper: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_OTYPER> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_OTYPER>
  },
  ospeedr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_OSPEEDR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_OSPEEDR>
  },
  pupdr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_PUPDR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_PUPDR>
  },
  idr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_IDR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 204
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_IDR>
  },
  odr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_ODR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_ODR>
  },
  bsrr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_BSRR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_BSRR>
  },
  lckr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_LCKR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_LCKR>
  },
  afrl: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_AFRL> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_AFRL>
  },
  afrh: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_AFRH> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_AFRH>
  },
  brr: stm32f3::generic::Reg<u32, stm32f3::stm32f303::gpioc::_BRR> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<stm32f3::stm32f303::gpioc::_BRR>
  }
}
```

所有这些新类型和闭包听起来都会生成大型、臃肿的程序，但如果您在启用[LTO]的发布模式下编译程序，
您会发现它生成的指令与使用`write_volatile`和十六进制地址的"unsafe"版本完全相同！

[LTO]: https://en.wikipedia.org/wiki/Interprocedural_optimization

使用`cargo objdump`获取汇编程序代码`release.txt`:
``` console
cargo objdump --bin registers --release -- -d --no-show-raw-insn --print-imm-hex > release.txt
```

然后在`release.txt`中搜索`main`
```
0800023e <main>:
 800023e:      	push	{r7, lr}
 8000240:      	mov	r7, sp
 8000242:      	bl	#0x2
 8000246:      	trap

08000248 <registers::__cortex_m_rt_main::h199f1359501d5c71>:
 8000248:      	push	{r7, lr}
 800024a:      	mov	r7, sp
 800024c:      	bl	#0x22
 8000250:      	movw	r0, #0x1018
 8000254:      	mov.w	r1, #0x200
 8000258:      	movt	r0, #0x4800
 800025c:      	str	r1, [r0]
 800025e:      	mov.w	r1, #0x800
 8000262:      	str	r1, [r0]
 8000264:      	mov.w	r1, #0x2000000
 8000268:      	str	r1, [r0]
 800026a:      	mov.w	r1, #0x8000000
 800026e:      	str	r1, [r0]
 8000270:      	b	#-0x4 <registers::__cortex_m_rt_main::h199f1359501d5c71+0x28>
```

最棒的是，没有人需要编写一行代码来实现GPIOE API。所有代码都是使用[svd2rust]工具从系统视图描述（SVD）文件自动生成的。
该SVD文件实际上是微控制器供应商提供的XML文件，包含其微控制器的寄存器映射。
该文件包含寄存器块的布局、基址、每个寄存器的读/写权限、寄存器的布局、寄存器是否有保留位以及许多其他有用信息。

[svd2rust]: https://crates.io/crates/svd2rust
