# 如何使用GDB

下面是一些有用的GDB命令，可以帮助我们调试程序。这假设您已将[程序闪存](../../05-led-roulette/flash-it.md)
到微控制器上，并将GDB连接到`cargo-embed`会话。

## 常规调试

> **注意**：您在下面看到的许多命令可以使用简短的形式执行。例如，`continue`可以简单地用作`c`，
> 或`break $location`可以用作`b $location`。一旦你对下面的命令有了经验，试着看看在GDB无法识别它们之前你可以让这些命令运行多短！


### 处理断点

* `break $location`：在代码中的某个位置设置断点。`$location`的值可以包括：
  * `break *main` - 函数`main`的确切地址上的break
  * `break *0x080012f2` - 在准确的内存位置`0x080012f2`上中断
  * `break 123` - 在当前显示文件的第123行中断
  * `break main.rs:123` - 在文件`main.rs`的第123行中断
* `info break`: 显示当前断点
* `delete`: 删除所有断点
  * `delete $n`:  删除断点`$n` (`n`是一个数字。例如：`delete $2`)
* `clear`: 删除下一条指令的断点
  * `clear main.rs:$function`: 删除`main.rs`中`$function`条目处的断点
  * `clear main.rs:123`:  删除`main.rs`第123行上的断点
* `enable`: 启用所有设置的断点
  * `enable $n`: 启用断点`$n`
* `disable`: 禁用所有设置的断点
  * `disable $n`: 禁用断点`$n`

### 控制执行

* `continue`: 开始或继续执行程序
* `next`: 执行程序的下一行
  * `next $n`: 重复`next` `$n`多次
* `nexti`: 与`next`相同，但使用机器指令
* `step`: 执行下一行，如果下一行包含对另一个函数的调用，则进入该代码
  * `step $n`: 重复`step` `$n`多次
* `stepi`:  与`step`相同，但使用机器指令
* `jump $location`: 在指定位置继续执行：
  * `jump 123`: 在第123行继续执行
  * `jump 0x080012f2`: 在地址0x08001f2恢复执行

### 打印信息

* `print /$f $data` - 打印变量`$data`包含的值。可选地，使用`$f`格式化输出，包括：
    ```txt
    x: 十六进制
    d: 有符号十进制
    u: 无符号十进制
    o: 八进制
    t: 二进制
    a: 地址
    c: 字符
    f: 浮点
    ```
  * `print /t 0xA`: 将十六进制值`0xA`打印为二进制(0b1010)
* `x /$n$u$f $address`: 检查`$address`处的内存。可选，`$n`定义要显示的单位数，
  `$u` 单位大小(字节、半字、字等), `$f`以上定义任何的`print`格式
  * `x /5i 0x080012c4`: 打印5条机器指令，起始地址为`0x080012c4`
  * `x/4xb $pc`: 从`$pc`当前指向的位置开始打印4字节内存
* `disassemble $location`
  * `disassemble /r main`: 反汇编函数`main`，使用`/r`显示组成每个指令的字节


### 查看符号表

* `info functions $regex`: 打印与`$regex`匹配的函数的名称和数据类型，省略`$regex`以打印所有函数
  * `info functions main`: 打印包含单词`main`的已定义函数的名称和类型
* `info address $symbol`: 打印`$symbol`存储在内存中的位置
  * `info address GPIOC`: 打印变量`GPIOC`的内存地址
* `info variables $regex`: 打印与`$regex`匹配的全局变量的名称和类型，省略`$regex`打印所有全局变量
* `ptype $data`:  打印有关`$data`的更多详细信息
  * `ptype cp`: 打印变量`cp`的详细类型信息

### 浏览程序堆栈

* `backtrace $n`: 打印`$n`帧的跟踪，或省略`$n`以打印所有帧
  * `backtrace 2`: 前2帧的打印跟踪
* `frame $n`: 选择编号或地址为`$n`的帧，忽略`$n`以显示当前帧
* `up $n`: 选择帧`$n`帧向上
* `down $n`: S选择帧`$n`帧向下
* `info frame $address`: 在`$address`处描述帧，忽略当前选定帧的`$address`
* `info args`: 打印所选帧的参数
* `info registers $r`: 打印选定帧中寄存器`$r`的值，忽略所有寄存器`$r`
  * `info registers $sp`: 打印当前帧中堆栈指针寄存器`$sp`的值

### 远程控制OpenOCD

* `monitor reset run`: 重置CPU，重新开始执行
    * `monitor reset`: 同上
* `monitor reset init`: 重置CPU，开始时停止执行
* `monitor targets`: 显示当前目标的信息和状态
