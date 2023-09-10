# `uprintln!`

在下一个练习中，我们将实现`uprint!`系列宏。您的目标是使这行代码正常工作：

``` rust
    uprintln!(serial, "The answer is {}", 40 + 2);
```

它必须通过串行接口发送字符串`"The answer is 42"`。

我们该怎么做？研究`println!`的`std`实现很有帮助。

``` rust
// src/libstd/macros.rs
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}
```

到目前为止看起来很简单。我们需要内置的`format_args!`宏 (它是在编译器中实现的，所以我们看不到它的实际功能)。
我们必须以完全相同的方式使用该宏。此`_print`函数的作用是什么？

``` rust
// src/libstd/io/stdio.rs
pub fn _print(args: fmt::Arguments) {
    let result = match LOCAL_STDOUT.state() {
        LocalKeyState::Uninitialized |
        LocalKeyState::Destroyed => stdout().write_fmt(args),
        LocalKeyState::Valid => {
            LOCAL_STDOUT.with(|s| {
                if s.borrow_state() == BorrowState::Unused {
                    if let Some(w) = s.borrow_mut().as_mut() {
                        return w.write_fmt(args);
                    }
                }
                stdout().write_fmt(args)
            })
        }
    };
    if let Err(e) = result {
        panic!("failed printing to stdout: {}", e);
    }
}
```

这*看起来*很复杂，但我们唯一感兴趣的部分是：`w.write_fmt(args)`和`stdout().write_fmt(args)`。
什么`print!`最终要做的是使用`format_args!`的输出调用`fmt::Write::write_fmt`方法！

幸运的是，我们也不必实现`fmt::Write::write_fmt`方法，因为它是默认方法。我们只需要实现`fmt::Write::write_str`方法。

让我们这样做吧。

这就是相等的macro的一面。剩下的工作是提供`write_str`方法的实现。

上面我们看到`Write`在`std::fmt`中。 我们无法访问`std`，但`Write`也可以在`core::fmt`中使用。

``` rust
{{#include examples/the-answer.rs}}
```
