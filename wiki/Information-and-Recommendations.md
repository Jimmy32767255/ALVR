## 电脑

- 高端电脑是必需的；ALVR 不是 PCVR HMD 的廉价替代品
- ALVR 分辨率配置和 SteamVR 多重采样可用于影响质量以支持性能或反之。
- 频繁丢帧会导致 ALVR 上的体验不佳；这可以使用 [OVR Advanced Settings](https://github.com/OpenVR-Advanced-Settings/OpenVR-AdvancedSettings) 等工具进行验证。
- 更高的比特率会导致更高的延迟。
- 确保所有相关软件都是最新的——尤其是显卡和网络驱动程序。
- 一个好的起点是 100% 分辨率（`非常低`分辨率预设）和 30mbit 恒定比特率。在此配置中，它应该非常流畅，几乎没有延迟或丢包；此时看到的丢包很可能是网络问题导致的。

## 网络

- 强烈建议电脑通过有线连接到网络。
- 建议使用支持至少 802.11ac（理想情况下为 802.11ax）的现代中高端路由器和/或接入点。

## 无线

### 一般 WiFi 配置最佳实践

- 任何可以有线连接的设备都应该有线连接——每个无线设备都会降低整个无线网络的整体速度
- 设备应尽量减少障碍物，并尽可能靠近接入点或路由器
- 任何其他无线网络（例如：打印机的默认无线网络）都应禁用；每个网络都会降低其他网络的速度
- 任何不需要高速但支持高速的设备（例如：恒温器）都应使用 2.4Ghz；通常中高端接入点和路由器支持“强制”客户端使用 2.4Ghz 的方法，有些甚至可以根据信号强度和连接速度自动执行此操作
- 只应启用必要的 WiFi 版本；较旧的标准，如 802.11b、802.11g，以及在较小程度上，802.11n，都会降低所有客户端的速度
- 需要高速的设备（例如独立头戴设备）应使用：
  - 仅 5GHz
  - 最新的 WiFi 规范（802.11ax，其次是 802.11ac）
  - 在大多数环境中，尽可能大的信道宽度（802.11ax 为 160MHz，802.11ac 实际为 80MHz）（**注意：某些供应商默认不将其设置为最大值**）
  - 最低的利用率，其次是最低的信道号（子频率）
- **手动选择信道只应在极端噪音的地方，或在较旧、质量较低或 ISP 提供的接入点或路由器上进行**——现代中高端路由器和接入点应该能很好地优化其信道，并且由于其他路由器和客户端的“信道跳跃”，静态设置通常效果较差
- 如果绝对需要特定的 WiFi 信道范围，请使用手机或电脑上的 WiFi 扫描工具来确定使用最少的信道——中高端接入点和路由器也可能提供此接口，但是，这有时会在扫描时导致断开连接
- **手动选择 WiFi 信号强度只应在极端噪音的地方进行**——现代路由器和接入点在这方面做得很好，这是一项复杂的任务
- 如果需要特定的发射功率，请记住，更强并不总是更好——随着发射功率的增加，失真可能会增加（导致速度*降低*），客户端的电池寿命可能会增加（由于接入点或路由器请求更高的功率），并且可能会出现粘性客户端问题（即使信号不好，设备仍保持连接到 WiFi）
- 如果您有大量设备，某些路由器和接入点支持诸如空闲时间公平性之类的功能，这有助于限制较慢客户端占用的空闲时间量，从而提高高速客户端的性能

### 配置无线网络和设备时需要记住的事项

- 同一频率上的所有设备都会相互影响（**包括同一信道上的其他 WiFi 网络**），因为一次只能有一个设备传输或接收数据，这意味着：
  - 如果一个设备大量使用 WiFi，它将影响所有其他客户端的延迟和吞吐量
  - 如果连接了慢速设备，它仍然会占用大量的“空闲时间”（该专用客户端向接入点或路由器传输/接收数据的时间），即使它以比其他客户端慢的速度进行
  - 每个连接的设备都需要额外的时间，无论它是否处于活动使用状态（并且设备通常在空闲时发送少量数据，例如 NTP 和 DHCP）
- WiFi 本质上是 [半双工](https://en.wikipedia.org/wiki/Duplex_(telecommunications)#Half_duplex) 的，因为它是一种射频，这意味着数据只能在同一频率上进行传输**或**接收，不能同时进行；双绞线（铜以太网电缆）是全双工的
- 无线频段（例如：2.4Ghz、5Ghz）具有独立的信道，如果需要可以静态分配，但**这些并非互斥，这意味着信道会显著重叠并相互干扰**
- 世界不同地区支持不同的信道（子频率）；在这些地区销售的设备通常被锁定在这些信道（例如：在美国，2.4Ghz 信道 12-13 仅限低功率，信道 14 仅限军事和 EMS 使用）
- 不同的无线设备支持不同的频率、标准、速度和功能；利用这些优势是获得最佳性能的关键

## 路由 / 交换 / 防火墙 / 一般信息

- 理想情况下，头戴设备和串流器应存在于相同的逻辑（第 2 层）网络和子网中——这可以避免路由开销，并通过 [mDNS](https://en.wikipedia.org/wiki/Multicast_DNS) 正确实现设备发现功能
- 双绞线（普通铜质以太网线）不应与电源线并行布设——这可能导致信号噪声，并导致丢帧和自动协商速度降低
- 现代网络应使用高质量的 CAT5E 或更高（理想情况下为 CAT6A 或 CAT7）千兆以上布线
- 在某些情况下，防火墙、防病毒、恶意软件或 EDR（增强检测和响应）软件可能会干扰网络流量——据报告，Windows Defender 和 Sophos Endpoint Protection 可以正常工作
- 应尽可能禁用暂停帧，因为它们会引入额外的延迟和缓冲

***

有人就其中一些观点撰写了一些博客文章：
<https://imaginevr.home.blog/author/imaginevrresearch/>

部分观点来自 [FingrMastr](https://github.com/FingrMastr)

## Linux

### 编码器要求

ALVR 使用 FFmpeg 进行所有编码，因此您需要确保您选择的编码器与 FFmpeg 兼容。
请务必查阅仪表板中的日志选项卡，它会告诉您编码器初始化失败的原因。

### VAAPI (AMD/Intel GPU)

需要 *libva* 和适用于您 GPU 的相应驱动程序。使用 `vainfo` 检查编解码器支持：

```sh
$ vainfo                                                                                                                                                                       130 ↵ !10090
Trying display: wayland
vainfo: VA-API version: 1.16 (libva 2.16.0)
vainfo: Driver version: Mesa Gallium driver 23.0.0-devel for Radeon RX 7900 XTX (gfx1100, LLVM 16.0.0, DRM 3.49, 6.1.1-zen1-1-zen)
vainfo: Supported profile and entrypoints
      VAProfileH264ConstrainedBaseline:    VAEntrypointVLD
      VAProfileH264ConstrainedBaseline:    VAEntrypointEncSlice
      VAProfileH264Main               :    VAEntrypointVLD
      VAProfileH264Main               :    VAEntrypointEncSlice
      VAProfileH264High               :    VAEntrypointVLD
      VAProfileH264High               :    VAEntrypointEncSlice
      VAProfileHEVCMain               :    VAEntrypointVLD
      VAProfileHEVCMain               :    VAEntrypointEncSlice
      VAProfileHEVCMain10             :    VAEntrypointVLD
      VAProfileHEVCMain10             :    VAEntrypointEncSlice
      VAProfileJPEGBaseline           :    VAEntrypointVLD
      VAProfileVP9Profile0            :    VAEntrypointVLD
      VAProfileVP9Profile2            :    VAEntrypointVLD
      VAProfileAV1Profile0            :    VAEntrypointVLD
      VAProfileNone                   :    VAEntrypointVideoProc
```

需要 *VAProfileH264High, VAProfileHEVCMain, VAProfileHEVCMain10* 编码器 (VAEntrypointEncSlice)。如果您在输出中没有看到这些，则您的驱动程序安装不正确或您的发行版决定在没有非自由编解码器的情况下构建 *mesa*。

#### 测试 ffmpeg 命令 (VAAPI)

```sh
# H264
ffmpeg -vaapi_device /dev/dri/renderD128 -f lavfi -i testsrc -t 30 -vf 'format=nv12,hwupload' -c:v h264_vaapi vaapi-h264.mp4

# HEVC
ffmpeg -vaapi_device /dev/dri/renderD128 -f lavfi -i testsrc -t 30 -vf 'format=nv12,hwupload' -c:v hevc_vaapi vaapi-hevc.mp4
```

### NVENC (Nvidia)

需要 *libcuda*。

#### 测试 ffmpeg 命令 (Nvidia)

```sh
# H264
ffmpeg -f lavfi -i testsrc -t 30 -vf 'format=nv12,hwupload' -c:v h264_nvenc nvenc-h264.mp4

# HEVC
ffmpeg -f lavfi -i testsrc -t 30 -vf 'format=nv12,hwupload' -c:v hevc_nvenc nvenc-hevc.mp4
```

### 软件 (任何 GPU)

软件编码器主要用作备用方案，因此应在所有 GPU 上工作，没有任何要求。
目前仅支持 H264 编码。
