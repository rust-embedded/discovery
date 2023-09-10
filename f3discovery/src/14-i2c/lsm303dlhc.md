# LSM303DLHC

**注意**：最新的 (从2020/09开始) Discovery开发板可能具有[LSM303AGR][agr]，而不是[LSM303DLHC][Data Sheet]。
有关更多详细信息，请查看下面的github [issues][gh-issue-274]。

[agr]: https://www.st.com/resource/en/datasheet/lsm303agr.pdf
[gh-issue-274]: https://github.com/rust-embedded/discovery/issues/274

F3中的两个传感器，磁力计和加速计，封装在一个组件中：LSM303DLHC集成电路。这两个传感器可以通过I2C总线访问。
每个传感器的行为类似于I2C从设备，并且具有*不同*的地址。

每个传感器都有自己的存储器，存储感知环境的结果。我们与这些传感器的交互主要涉及读取它们的记忆。

这些传感器的存储器被建模为字节可寻址寄存器。这些传感器也可以配置；这是通过写入他们的寄存器来完成的。
因此，在某种意义上， 这些传感器与微控制器*内部*的外围设备非常相似。不同之处在于，它们的寄存器没有映射到微控制器的内存中。
相反，它们的寄存器必须通过I2C总线访问。

LSM303DLHC的主要信息来源是其[数据表][Data Sheet]。通读它，看看如何读取传感器的寄存器。该部分位于：

[Data Sheet]: http://www.st.com/resource/en/datasheet/lsm303dlhc.pdf

> 第5.1.1节I2C操作-第20页-LSM303DLHC数据表

与本书相关的文件的另一部分是登记册的说明。该部分位于：

> 第7节寄存器说明-第25页-LSM303DLHC数据表
