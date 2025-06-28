## If you're looking for Linux troubleshooting, please check [here](https://github.com/Jimmy32767255/ALVR-CN/wiki/Linux-Troubleshooting) first, and only then this page.

适用于ALVR 20.0.0及更高版本
===

首先，请务必仔细阅读[安装指南](https://github.com/Jimmy32767255/ALVR-CN/wiki/Installation-guide)和[使用](https://github.com/Jimmy32767255/ALVR-CN/wiki/Usage)页面。

首先尝试删除PC上ALVR安装文件夹中的`session.json`文件。这会将所有设置重置为默认值。如果不起作用，请尝试重新安装ALVR。

请记住，有时重启ALVR/SteamVR/PC/头显就足以解决一些问题。

ALVR无法正常工作？
---

[我启动ALVR时遇到问题。](#trouble-starting-alvr)

[ALVR启动正常，但显示X错误。](#alvr-starts-fine-but)

[ALVR启动正常且未显示任何错误，但它无法识别（或连接到）我的头显。](#alvr-cant-see-my-headset)

如果您需要更多帮助，请加入官方[Discord](https://discord.gg/KbKk3UM)并在#help频道提问。寻求帮助时，请描述问题，如果收到错误消息，请复制它，并告诉我们您已经尝试了哪些修复方法。

ALVR启动问题
===

ALVR需要安装正常工作的图形驱动程序才能运行。

**在Linux上**，您还需要确保AMD显卡安装了`vaapi`，或NVIDIA显卡安装了`cuda`，以便硬件编码器正常工作。

ALVR启动时卡在“ALVR无响应...”
===

对于ALVR 20.0及更高版本，某些杀毒软件可能会阻止ALVR启动SteamVR。尝试禁用除Windows Defender之外的任何杀毒软件（如McAfee、Norton等），重启电脑，然后再次尝试。如果问题仍然存在，请确保没有ALVR或SteamVR的实例在后台运行（在任务管理器中检查）。如果您仍然遇到问题，请加入[ALVR Discord服务器](https://discord.gg/KbKk3UM)，我们将尽力帮助您解决问题。

ALVR启动正常，但出现问题
===

本节提供了一些关于ALVR显示错误（或有时是警告）弹出窗口时的建议。这可能是设置窗口（`ALVR Dashboard.exe`）中的黄色弹出窗口，也可能是连接头显时单独的弹出窗口。

[警告] 发现无效客户端
---

如果您在`ALVR Dashboard.exe`窗口中收到`clientFoundInvalid`的警告弹出窗口，请确保您在头显上安装的ALVR版本与您尝试在PC上运行的版本兼容。

最新版本可以在[这里](https://github.com/Jimmy32767255/ALVR-CN/releases/latest)找到，其中包含适用于您头显的`alvr_client.apk`文件和包含PC应用程序的`alvr_streamer_windows.zip`压缩包。

SideQuest商店中提供的ALVR版本与GitHub上的最新版本（上一个链接）兼容。请记住，GitHub发布新版本后，SideQuest上的版本可能需要一段时间才能更新。

初始化CEncoder失败
---

ALVR目前需要较新的AMD或Nvidia GPU才能运行，因为它利用了硬件视频编码（参见[要求](https://github.com/alvr-org/ALVR#requirements)）。如果您收到类似以下的错误信息：

```
Failed to initialize CEncoder. All VideoEncoder are not available. VCE: AMF Error 1. g_AMFFactory.Init(), NVENC: NvEnc NvEncoderD3D11 failed. Code=1 NvEncoder::LoadNvEncApi : NVENC library file is not found. Please ensure NV driver is installed at c:\src\alvr\alvr_server\nvencoder.cpp:70
```

并且您的GPU驱动程序是最新的，那么您的显卡不受支持。如果您使用的是带有足够强大独立GPU的笔记本电脑，您_可能_可以通过在Windows设置或Nvidia控制面板中强制SteamVR使用它来使ALVR工作。

如果您有兼容的GPU，您很可能在`VCE:`或`NVENC:`之后看到与上述不同的错误。在这种情况下，请尝试在ALVR设置中使用不同的视频编解码器。您也可以尝试降低视频分辨率设置。

启动音频捕获失败
---

![启动音频捕获失败](images/ALVR-audio-crash.png)

当您连接头显并启动SteamVR时，可能会出现此错误。请确保您在ALVR设置中选择的音频设备未被禁用，它应该是您通常用于游戏的设备（扬声器/耳机）。ALVR不会创建自己的音频设备。

您可以检查Windows设置中您的声音设备是否有“启用音频增强”选项，如果有，请确保其已禁用。

ALVR无法识别我的头显
===

即使您没有看到ALVR的任何错误弹出窗口，也可能出现一些问题，以下是一些建议。

头显上的ALVR卡在“正在搜索串流器...”
---

此问题可能有多种原因。问题很可能出在PC上的ALVR应用程序。请参阅下文了解更具体的问题。

ALVR设备列表为空
---

![ALVR设备列表为空](images/ALVRexe-no-devices.png)

请检查PC应用程序和头显应用程序是否运行最新版本的ALVR。如果您的版本是v2.3.1或v2.4.0-alpha5，那么您从错误的链接下载了ALVR。正确的链接是<https://github.com/alvr-org/ALVR>。

确保ALVR在PC和头显上都已运行。为了在设备列表中可见，头显上的ALVR会发送广播包，PC应用程序会监听这些包。如果头显和PC都通过无线连接，这些包可能会被您的防火墙或路由器阻止，如果路由器启用了AP隔离，也会导致此问题。

要解决此问题，您可以尝试以下方法：

* Ping头显以检查PC是否可以访问它——您可以通过打开CMD并输入`ping <头显IP>`（不带“<>”，您可以在SideQuest左上角找到头显的IP）来完成此操作——如果ping失败，请检查PC和头显是否连接到同一网络
* 您也可以尝试禁用防火墙进行测试，但不应为了使用ALVR而一直禁用它
* 在防火墙上打开端口9943和9944
* 禁用路由器上的PMF（受保护管理帧）设置

如果ping通但您仍然在串流器应用程序中看不到设备，那么头显和PC可能位于不同的子网中。要解决此问题，您可以手动添加设备。
在“设备”选项卡中，按“手动添加设备”。填写头显名称（您可以使用您想要的名称）、主机名（您可以在头显打开ALVR应用程序时的欢迎屏幕中读取）和头显IP，然后按“保存”。

SteamVR显示“未检测到头显”
---

![SteamVR未检测到头显](images/SteamVR-headset-not-detected.png)

此消息表示ALVR SteamVR驱动程序在SteamVR启动时未正确加载。

在Linux上，请仔细检查是否安装了软件和硬件编码器，否则驱动程序将无法加载。

检查SteamVR是否阻止了ALVR（请参阅SteamVR设置，启用高级设置并检查`启动/关机 -> 管理附加组件`）。

![SteamVR附加组件](images/SteamVR-add-ons.png)

如果您仍然收到此消息（或者在SteamVR窗口中没有头显图标），SteamVR日志（vrserver.txt）将包含有关驱动程序未加载原因的一些信息。您可以在Steam安装目录的`Steam\logs\vrserver.txt`中找到它。

### 值得关注的日志行及提示

`Unable to load driver alvr_server because of error VRInitError_Init_FileNotFound(103). Skipping.` - 这通常意味着ALVR所需的一个库缺失。请确保您仔细遵循了安装说明，安装了最新的Visual C++ Redistributable x64软件包，并且您解压ALVR的目录中没有文件缺失（尤其是在bin\win64目录中）。

`Skipping duplicate external driver alvr_server` - 此行表示已注册了另一个ALVR驱动程序。请前往ALVR的安装选项卡并删除所有驱动程序。

`Skipping external driver X:\path\to\your\alvr_streamer_windows because it is not a directory` - 如果您将ALVR放在OneDrive（或类似服务）目录中，或者ALVR的路径包含非UTF-8字符，则可能会发生这种情况。尝试将ALVR放在其他位置，最好是ALVR的路径只包含ASCII字符。

如果您在查看日志时遇到问题，或者以上提示均无效或不适用于您，请随时在官方[Discord](https://discord.gg/KbKk3UM)的#help频道提问（您可能会被要求在那里发布日志）。

ALVR识别到头显，SteamVR显示头显图标
---

![SteamVR等待中...](images/SteamVR-waiting.png)

在这种情况下，您在头显和PC上都打开了ALVR，您可以在设备列表中看到头显并信任它。当您尝试连接时，ALVR会自动启动SteamVR，并且SteamVR会显示头显（和控制器）的图标。

首先，请确保您的防火墙允许SteamVR（更具体地说，是vrserver.exe）的传入连接（UDP，端口9944）。您也可以尝试禁用防火墙进行测试，但为了使用ALVR，请保持其禁用状态。

您可以尝试重启头显和PC上的ALVR。在头显上连接时，您应该会看到转动头部时画面滞后（低于1帧/秒），这意味着头显在连接时收到了串流器的响应，并正在等待视频流开始。如果头显没有滞后，则表示PC的响应未到达头显。

## 常见性能相关问题

### 编码器过载

![编码器过载的延迟图](images/latency-graphs/overloaded-encoder.png)

症状：头显播放卡顿，串流器帧率稳定但低于目标刷新率。

解决方案：增加注视点渲染设置或降低刷新率。

### 解码器过载

![解码器过载的延迟图](images/latency-graphs/overloaded-decoder.png)

症状：控制器卡顿/冻结，头部追踪错误，图像上下颠倒，闪烁纯色。

解决方案：降低比特率。

### 网络过载

![网络过载的延迟图](images/latency-graphs/overloaded-network.png)

症状：串流冻结，图像出现故障。

解决方案：检查头显是否使用5G频率，并且没有其他设备连接到您的AP上的5G频段，降低比特率或使用有线连接。

### 串流器过载

![串流器过载的延迟图](images/latency-graphs/overloaded-streamer.png)

症状：头显播放卡顿，串流器帧率下降或波动，低于目标刷新率。

解决方案：

* 降低游戏中的图形设置
* 如果可能，使用游戏的本地放大解决方案（FSR/NIS/XeSS/DLSS…）
* 降低ALVR中的目标刷新率
* 降低SteamVR叠加层或ALVR视频设置中的渲染分辨率。（这将严重降低图像质量。）

### 微卡顿

![头显卡顿的延迟图](images/latency-graphs/not-enough-buffering.png)

症状：图像并非总是流畅，尤其是在高运动或快速场景中。

解决方案：增加maxBufferingFrames。


### Meta帧率缩放节流功能的临时修复

#### 问题  
当前版本的ALVR不支持Meta的帧率缩放节流功能。这可能导致头显和串流应用程序之间的帧率不一致，从而可能导致卡顿或节流。ALVR的未来更新有望解决此问题，但在此期间有一个临时解决方案。

#### 临时修复  
1. **重启您的头显**  
   - 首先重启您的VR头显。这可能无需进一步调整即可解决问题。

2. **手动设置帧率**  
   - 使用**SideQuest桌面应用程序**手动调整头显上ALVR Android客户端的帧率，使其与ALVR串流应用程序中设置的帧率匹配。  
     - 示例：如果ALVR串流器配置为90Hz，请在SideQuest中将头显的刷新率设置为90Hz。
     - 更多信息请参阅问题[#2537](https://github.com/Jimmy32767255/ALVR-CN/issues/2537)。

此调整绕过了帧率缩放节流功能，确保了更流畅的性能。
