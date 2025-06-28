FFmpeg hardware video offloading test commands to validate hardware encoding offloading is working.

Learn more at: <https://trac.ffmpeg.org/wiki/HWAccelIntro>

## 编解码器

* **高级视频编码 (AVC/h264)** - <https://en.wikipedia.org/wiki/Advanced_Video_Coding>

 高级视频编码 (AVC)，也称为 H.264 或 MPEG-4 第 10 部分，是一种基于块导向、运动补偿编码的视频压缩标准。它是目前用于视频内容录制、压缩和分发的最常用格式。它支持最高 8K UHD 分辨率。硬件编码支持广泛可用。

* **高效视频编码 (HEVC/h265)** - <https://en.wikipedia.org/wiki/High_Efficiency_Video_Coding>

 高效视频编码 (HEVC)，也称为 H.265 和 MPEG-H 第 2 部分，是 MPEG-H 项目中设计的一种视频压缩标准，旨在取代广泛使用的高级视频编码 (AVC、H.264 或 MPEG-4 第 10 部分)。与 AVC 相比，HEVC 在相同视频质量下提供 25% 到 50% 更好的数据压缩，或者在相同比特率下显著提高视频质量。它支持高达 8192×4320 的分辨率，包括 8K UHD，并且与主要为 8 位的 AVC 不同，HEVC 的更高保真度 Main 10 配置文件已集成到几乎所有支持硬件中。硬件编码支持广泛可用。

* **AOMedia Video 1 (AV1)** - <https://en.wikipedia.org/wiki/AV1>

 AOMedia Video 1 (AV1) 是一种开放、免版税的视频编码格式，最初设计用于互联网视频传输。它由开放媒体联盟 (AOMedia) 开发，作为 VP9 的继任者。AV1 比特流规范包含一个参考视频编解码器。硬件编码支持仅限于最新一代硬件。

### 图形编码 API

* **视频加速 API** - <https://en.wikipedia.org/wiki/Video_Acceleration_API>

  视频加速 API (VA-API) 是一个开源应用程序编程接口，允许 VLC 媒体播放器或 GStreamer 等应用程序使用硬件视频加速功能，这些功能通常由图形处理单元 (GPU) 提供。它由免费开源库 libva 实现，并结合硬件特定驱动程序，通常与 GPU 驱动程序一起提供。

  使用 `vainfo` 检查您当前的 VA-API 状态。

* **Vulkan Video** - <https://en.wikipedia.org/wiki/Vulkan>

 Vulkan 是一个低级、低开销、跨平台的 3D 图形和计算 API 及开放标准。它旨在解决 OpenGL 的缺点，并允许开发人员对 GPU 进行更多控制。它旨在支持各种 GPU、CPU 和操作系统，也旨在与现代多核 CPU 配合使用。支持即将推出。请参阅：<https://www.khronos.org/blog/khronos-releases-vulkan-video-av1-decode-extension-vulkan-sdk-now-supports-h.264-h.265-encode>

* **NVENC** - <https://en.wikipedia.org/wiki/Nvidia_NVENC>

 Nvidia NVENC 是 Nvidia 显卡中的一项功能，可执行视频编码，将此计算密集型任务从 CPU 卸载到 GPU 的专用部分。

* **AMD 高级媒体框架** - <https://gpuopen.com/advanced-media-framework/>

 AMD AMF 是一个用于最佳访问 AMD GPU 进行多媒体处理的 SDK。

### 测试源输入生成

请注意，使用测试源进行输入生成会产生 CPU 负载。在监控 GPU 卸载是否正常时，FFmpeg 仍然会产生预期的 CPU 负载。

```sh
ffmpeg -hide_banner -f lavfi -i testsrc2=duration=30:size=1280x720:rate=90
```

* **lavfi** - <https://ffmpeg.org/ffmpeg-devices.html#toc-lavfi>

 Libavfilter 输入虚拟设备。此输入设备从 libavfilter 滤镜图的开放输出垫读取数据。对于每个滤镜图开放输出，输入设备将创建一个相应的流，该流映射到生成的输出。滤镜图通过选项 `graph` 指定。

* **testsrc2** - <https://ffmpeg.org/ffmpeg-filters.html#allrgb_002c-allyuv_002c-color_002c-colorchart_002c-colorspectrum_002c-haldclutsrc_002c-nullsrc_002c-pal75bars_002c-pal100bars_002c-rgbtestsrc_002c-smptebars_002c-smptehdbars_002c-testsrc_002c-testsrc2_002c-yuvtestsrc>

 `testsrc2` 源生成一个测试视频模式，显示颜色模式、滚动渐变和时间戳。这主要用于测试目的。`testsrc2` 源类似于 `testsrc`，但支持更多像素格式，而不仅仅是 `rgb24`。这允许将其用作其他测试的输入，而无需格式转换。

  1) duration - 剪辑时长（秒）
  2) size - 视频尺寸
  3) rate - 帧率（每秒）

### 渲染回放

使用您喜欢的视频播放器验证视频是否正确渲染。

* **MPV** - <https://en.wikipedia.org/wiki/Mpv_(media_player)>

 mpv 是一款基于 MPlayer、mplayer2 和 FFmpeg 的免费开源媒体播放器软件。它运行在多种操作系统上，包括类 Unix 操作系统（Linux、基于 BSD 的、macOS）和 Microsoft Windows，并有一个名为 mpv-android 的 Android 移植版。它是跨平台的，运行在 ARM、PowerPC、x86/IA-32、x86-64 和 MIPS 架构上。

* **VLC** - <https://en.wikipedia.org/wiki/VLC_media_player>

 VLC 媒体播放器（以前称为 VideoLAN Client，通常简称为 VLC）是由 VideoLAN 项目开发的免费开源、便携式、跨平台媒体播放器软件和流媒体服务器。VLC 可用于桌面操作系统和移动平台，如 Android、iOS 和 iPadOS。VLC 也可在 Apple App Store、Google Play 和 Microsoft Store 等数字分发平台上获取。

### 英伟达 GPU

测试英伟达硬件编码流程。目前仅支持 NVENC，因为当前的英伟达 VA-API 驱动 (<https://github.com/elFarto/nvidia-vaapi-driver>) 仅支持 NVDEC。请在 <https://developer.nvidia.com/video-encode-and-decode-gpu-support-matrix-new> 检查您的硬件是否支持 NVENC。

监控工具：

* **nvtop** - <https://github.com/Syllo/nvtop>

 NVTOP 代表 Neat Videocard TOP，一个类似于 (h)top 的 AMD、Intel 和 NVIDIA GPU 任务监视器。它可以处理多个 GPU 并以 htop 熟悉的方式打印有关它们的信息。

* **nvidia-smi pmon** - <https://developer.nvidia.com/nvidia-system-management-interface>

 NVIDIA 系统管理界面 (nvidia-smi) 是一个命令行实用程序，基于 NVIDIA 管理库 (NVML) 构建，旨在帮助管理和监控 NVIDIA GPU 设备。`pmon` 命令列出了在每个设备上运行的所有计算和图形进程的统计信息。

Nvenc AVC (h264) 硬件编码：

```sh
ffmpeg -hide_banner \
-f lavfi -i testsrc2=duration=300:size=1280x720:rate=90 \
-c:v h264_nvenc -qp 18 \
nvidia-h264_nvec-90fps-300s.mp4
```

Nvenc HEVC (h265) 硬件编码：

```sh
ffmpeg -hide_banner \
-f lavfi -i testsrc2=duration=300:size=1280x720:rate=90 \
-c:v hevc_nvenc -qp 18 \
nvidia-hevc_nvec-90fps-300s.mp4
```

Nvenc AV1 硬件编码（Ada Lovelace 或更新的硬件）：

```sh
ffmpeg -hide_banner \
-f lavfi -i testsrc2=duration=300:size=1280x720:rate=90 \
-c:v av1_nvenc -qp 18 \
nvidia-av1_nvec-90fps-300s.mp4
```

### 英特尔 GPU

测试英特尔硬件编码流程。在基于 GEN 的图形硬件上，仅支持使用 intel-media-driver (<https://github.com/intel/media-driver>) 的 VA-API。请在 <https://www.intel.com/content/www/us/en/docs/onevpl/developer-reference-media-intel-hardware/1-1/overview.html> 检查您的硬件是否支持编码编解码器。

监控工具：

* **nvtop** - <https://github.com/Syllo/nvtop>

 NVTOP 代表 Neat Videocard TOP，一个类似于 (h)top 的 AMD、Intel 和 NVIDIA GPU 任务监视器。它可以处理多个 GPU 并以 htop 熟悉的方式打印有关它们的信息。

VA-API AVC (h264) 硬件编码：

```sh
ffmpeg -hide_banner \
-f lavfi -i testsrc2=duration=300:size=1280x720:rate=90 \
-vaapi_device /dev/dri/renderD128 -vf 'format=nv12,hwupload' \
-c:v h264_vaapi -qp 18 \
intel-h264_vaapi-90fps-300s.mp4
```

VA-API HEVC (h265) 硬件编码：

```sh
ffmpeg -hide_banner \
-f lavfi -i testsrc2=duration=300:size=1280x720:rate=90 \
-vaapi_device /dev/dri/renderD128 -vf 'format=nv12,hwupload' \
-c:v hevc_vaapi -qp 18 \
intel-hevc_vaapi-90fps-300s.mp4
```

VA-API AV1 硬件编码（仅限 Arc A 系列）：

```sh
ffmpeg -hide_banner \
-f lavfi -i testsrc2=duration=300:size=1280x720:rate=90 \
-vaapi_device /dev/dri/renderD128 -vf 'format=nv12,hwupload' \
-c:v av1_vaapi -qp 18 \
intel-av1_vaapi-90fps-300s.mp4
```

### AMD GPU

测试 AMD 硬件编码流程。在基于 AMD 的图形硬件上，仅支持使用 mesa-va-drivers (<https://mesa3d.org/>) 的 VA-API。请在 <https://en.wikipedia.org/wiki/Unified_Video_Decoder> 检查您的硬件是否支持编码编解码器。硬件编码需要 Video Core Next (VCN) 硬件。

监控工具：

* **nvtop** - <https://github.com/Syllo/nvtop>

 NVTOP 代表 Neat Videocard TOP，一个类似于 (h)top 的 AMD、Intel 和 NVIDIA GPU 任务监视器。它可以处理多个 GPU 并以 htop 熟悉的方式打印有关它们的信息。

VA-API AVC (h264) 硬件编码：

```sh
ffmpeg -hide_banner \
-f lavfi -i testsrc2=duration=300:size=1280x720:rate=90 \
-vaapi_device /dev/dri/renderD128 -vf 'format=nv12,hwupload' \
-c:v h264_vaapi -qp 18 \
amd-h264_vaapi-90fps-300s.mp4
```

VA-API HEVC (h265) 硬件编码：

```sh
ffmpeg -hide_banner \
-f lavfi -i testsrc2=duration=300:size=1280x720:rate=90 \
-vaapi_device /dev/dri/renderD128 -vf 'format=nv12,hwupload' \
-c:v hevc_vaapi -qp 18 \
amd-hevc_vaapi-90fps-300s.mp4
```

VA-API AV1 硬件编码（仅限 VCN 4.0+，Navi 3x）：

```sh
ffmpeg -hide_banner \
-f lavfi -i testsrc2=duration=300:size=1280x720:rate=90 \
-vaapi_device /dev/dri/renderD128 -vf 'format=nv12,hwupload' \
-c:v av1_vaapi -qp 18 \
amd-av1_vaapi-90fps-300s.mp4
```
