This post will continue to evolve during ALVR development.

## 长期目标

创建一个连接所有 XR 设备的通用桥梁。

## 即将推出

* 渲染器重写
  * **目的**：增加对 Linux 上 FFR 和色彩校正的支持，为切片编码做准备
  * **状态**：所有平台上的 FFE 和色彩校正已完成
* 编码器重写
  * **目的**：使用 [Vulkan 视频扩展](https://www.khronos.org/blog/an-introduction-to-vulkan-video) 支持所有操作系统和硬件的单一 API
  * **状态**：受限于 AMD 和 Intel 的采用，以及稳定版 Nvidia 驱动中该功能的落地
* Monado 驱动
  * **目的**：支持流媒体的其他运行时
  * **状态**：受限于重构

由于开发能力有限，无法提供预计发布时间。新版本不会有固定的发布周期，也没有预定的功能。
