# 反转字符串

好的，接下来让我们让服务器用发送的文本的反面来响应客户端，从而使服务器更有趣。
每次按下ENTER键时，服务器都会响应客户端。每个服务器响应都将在一行中。

这次你需要一个缓冲区；你可以使用[`heapless::Vec`]。以下是启动代码：

[`heapless::Vec`]: https://docs.rs/heapless/latest/heapless/struct.Vec.html

``` rust
{{#include examples/reverse-string.rs}}
```
