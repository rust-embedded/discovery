# 校准

在使用传感器并尝试使用它开发应用程序之前要做的一件非常重要的事情是验证它的输出实际上是正确的。
如果不是这种情况，我们需要校准传感器（或者它也可能损坏，但在这种情况下不太可能）。

在我的情况下，在两个不同的micro:bit's上，没有校准的磁力计与它应该测量的值相差很大。
因此，为了本章的目的，我们将假设必须校准传感器。

校准涉及相当多的数学（矩阵），因此我们不会在这里介绍，但如果您有兴趣，本[设计说明]会描述该过程。

[设计说明]: https://www.st.com/resource/en/design_tip/dt0103-compensating-for-magnetometer-installation-error-and-hardiron-effects-using-accelerometerassisted-2d-calibration-stmicroelectronics.pdf

幸运的是，为构建原始软件的团队已经在[这里]的C++中实现了校准机制。

[这里]: https://github.com/lancaster-university/codal-microbit-v2/blob/006abf5566774fbcf674c0c7df27e8a9d20013de/source/MicroBitCompassCalibrator.cpp

您可以在`src/calibration.rs`中找到它到Rust的翻译。在默认的`src/main.rs`中演示了它的用法。
校准工作方式如本视频所示：

<p>
<video src="https://video.microbit.org/support/compass+calibration.mp4" loop autoplay>
</p>

您必须上倾斜micro:bit，直到LED矩阵上的所有LED都亮起。

如果您不想在开发期间每次重新启动应用程序时都玩游戏，请随意修改`src/main.rs`模板，
以便在获得第一个静态校准后使用相同的静态校准。

现在我们已经完成了传感器校准，让我们来看看实际构建这个应用程序！
