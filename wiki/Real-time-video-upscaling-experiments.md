## Why?

Quest头显可以显示接近4K的分辨率。渲染游戏、编码和解码这种分辨率对PC和Quest来说都非常耗费资源。因此通常会在Quest上显示较低分辨率的图像。

理想情况下，这种放大后的图像输出应该与屏幕像素1:1匹配。但由于异步时间扭曲(Asynchronous Timewarp)步骤，这在Quest上是不可能的。OVR只接受未失真的帧。

目前ALVR在图像映射到OpenGL纹理之前不进行放大。这个纹理会被OVR插值以匹配屏幕像素。对于高于100%的视频分辨率使用双线性插值，对于低于100%的分辨率使用最近邻插值。

There's a lot of good info on this topic in this issue: <https://github.com/Jimmy32767255/ALVR-CN/issues/39>

## Lanczos重采样

这种传统的放大方法似乎是比基本双线性插值更好的选择，并且对GPU资源消耗相对较少。

一个GPL 2协议的Lanczos着色器实现可以在这里找到：<https://github.com/obsproject/obs-studio/blob/6943d9a973aa3dc935b39f99d06f4540ea79da61/libobs/data/lanczos_scale.effect>

## 神经网络图像超分辨率

我对使用AI放大器获得比传统信号处理方法更好结果的可能性进行了一些基础研究。

### XR2上的硬件加速

在Quest的SoC上运行快速神经网络似乎有3种途径。

首先是[Qualcomm Neural Processing SDK](https://developer.qualcomm.com/software/qualcomm-neural-processing-sdk/tools)，它能自动检测系统能力并选择正确的硬件(GPU、DSP或AI加速器)来运行神经网络。

[TensorFlow Lite NNAPI delegate](https://www.tensorflow.org/lite/performance/nnapi)依赖于Android Neural Networks API的硬件和驱动程序支持。

还有专门针对Snapdragon DSP的[TensorFlow Lite Hexagon delegate](https://www.tensorflow.org/lite/performance/hexagon_delegate)。

I only tested an example image super-resolution app from the [tensorflow respository](https://github.com/tensorflow/examples/tree/master/lite/examples/super_resolution) in CPU and generic GPU (OpenCL) accelerated modes. Even upscaling tiny 50x50 images took around 500ms with this. Even though better hardware acceleration could improve this I do not expect 100x improvements. The only hope for NN super-resolution to be viable would be to find a significantly faster neural net, which leads us into the next topic.

### 现有的神经网络

一个成熟的实时放大器是[Anime4K](https://github.com/bloc97/Anime4K/)。它声称可以在Vega64 GPU上在3毫秒内实现1080p到2160p的放大。根据[粗略估计](https://uploadvr.com/oculus-quest-2-benchmarks/)，Quest 2的性能比这类高端桌面GPU低10倍。通过一些优化和降低放大质量，这似乎并非完全不可能实现，但还有更多坏消息。Anime4K的信噪比(PSNR)相当差。它能做到这一点是因为动漫风格化的外观对重度过滤相当宽容。

对于具有更好PSNR的放大器，有很多选择但很少有能实时运行的。我能找到的最小的神经网络是[SubPixel-BackProjection](https://github.com/supratikbanerjee/SubPixel-BackProjection_SuperResolution)。它获得了不错的结果，但在我的测试中使用CUDA加速从720p放大到1080p需要3秒。这远远超出了XR2芯片的能力范围。

因此，结论是XR2似乎没有足够的性能来在如此高的分辨率下进行实时神经网络放大。我们更可能从传统技术中获得更好的结果。
