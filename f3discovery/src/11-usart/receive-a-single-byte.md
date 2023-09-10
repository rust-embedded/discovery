# 接收单个字节

到目前为止，我们已经从微控制器向您的计算机发送了数据。是时候尝试相反的方法了：从计算机接收数据。

有一个`RDR`寄存器，将填充来自RX线的数据。如果我们读取该寄存器，我们将检索通道另一侧发送的数据。
问题是：我们如何知道我们收到了（新的）数据？状态寄存器`ISR`有一位用于此目的：`RXNE`。我们可以忙着等flag。

``` rust
{{#include examples/receive-a-single-byte.rs}}
```

让我们试试这个程序吧！使用`continue`让它自由运行，然后在minicom/PuTTY的控制台中键入一个字符。
发生了什么？`_byte`变量的内容是什么？

```
(gdb) continue
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
0x8003d48 in __bkpt ()

(gdb) finish
Run till exit from #0  0x8003d48 in __bkpt ()
usart::main () at src/11-usart/src/main.rs:19
19              aux11::bkpt();

(gdb) p/c _byte
$1 = 97 'a'
```
