# 通用协议

I2C协议比串行通信协议更复杂，因为它必须支持多个设备之间的通信。让我们使用示例看看它是如何工作的：

## 主 -> 从

如果master要向slave发送数据：

<p>
  <img class="white_bg" height=180 title="I2C bus" src="https://upload.wikimedia.org/wikipedia/commons/3/3e/I2C.svg">
</p>

1. Master: 广播开始
2. M: 广播从机地址（7 位）+ R/W（第 8 位）设置为WRITE
3. Slave: 响应ACK（确认）
4. M: 发送一个字节
5. S: 响应ACK
6. 重复步骤4和5零次或多次
7. M: 广播停止或（广播重启并返回（2））

> **注意**：从机地址可以是10位而不是7位长。其他一切都不会改变。

## 主 <- 从

如果master要从slave读取数据：

<p>
<img class="white_bg" height=180 title="I2C bus" src="https://upload.wikimedia.org/wikipedia/commons/3/3e/I2C.svg">
</p>

1. M: 广播开始
2. M: 广播从机地址（7 位）+ R/W（第 8 位）设置为 READ
3. S: 以ACK响应
4. S: 发送字节
5. M: 以ACK响应
6. 重复步骤4和5零次或多次
7. M: 广播停止或（广播重启并返回（2））

> **注意**：从机地址可以是10位而不是7位长。其他一切都不会改变。
