## ALVR native wired mode support
从 v20.12 开始，ALVR 直接通过仪表板支持有线连接。
只需在设备屏幕上启用“有线连接”开关，插入头显
并接受头显上显示的“允许 USB 调试？”弹出窗口。

请注意，您的头显需要启用开发者模式和 USB 调试才能使用此功能。

对于 Quest 头显，请参阅[此处](https://developers.meta.com/horizon/documentation/native/android/mobile-device-setup/)获取说明。
关于安装 ADB 的最后一步应跳过，因为 ALVR 会自行下载 ADB 的副本并使用它。

如果您已成功执行所有这些步骤但仍无法连接，
请确保“连接 -> 有线客户端类型”设置与您安装客户端的位置匹配（对于启动器，也使用“Github”选项）。

## 已弃用（且笨拙）的方法：
以下部分列出了获取有线连接的旧的和已弃用的方法，仅供参考。

这与原生有线模式具有完全相同的要求，但需要额外的软件并且设置更复杂，因此应优先选择原生模式。

## ALVR 流媒体（PC）配置

* 在“设置”>“连接”中，**将连接流协议切换到 TCP**。
* 如果检测到您的头显，请单击“信任”。单击“编辑”，“添加新”并将 IP 地址更改为 `127.0.0.1`。
* 如果未检测到您的头显，请单击“手动添加设备”并使用 IP 地址 `127.0.0.1`。使用头显屏幕上显示的主机名。

## 让您的电脑与 HMD 通信

Quest、Pico HMD 是 Android 设备，因此我们可以使用 [Android 设备桥](https://developer.android.com/studio/command-line/adb) 命令来告诉 HMD 通过 USB 和 Wi-Fi 查找数据，使用端口转发。

您可以使用一些预制的应用程序/脚本（就在下面）来完成此操作，或者使用 [SideQuest](https://sidequestvr.com/setup-howto) 手动运行命令

如果您还没有，请将 USB 数据线从您的电脑连接到您的头显。USB 2.0 可以正常工作，但 3.0 及更高版本是最好的。

**如果您使用的是 Quest，请确保在头显中启用开发者帐户并授权电脑，或者在 Pico 设置中启用 USB 调试。**

### 选项 1 - 专用 ADB 应用程序

以下程序用于封装和简化手动 ADB 命令的过程，前两个程序还会在 USB 连接中断时自动重新连接头显。

* [**ADBForwarder（推荐）**](https://github.com/alvr-org/ADBForwarder)
  
  * 易于使用
  * 为您下载 ADB
  * 跨平台（Windows 和 Linux）

* [**Python 脚本**](https://gist.github.com/Bad-At-Usernames/684784f42cbb69e22688a21173ec263d)
  
  * 轻量且简单
  * 需要 [Python 3](https://www.python.org/downloads/) 和 [PyWin32](https://pypi.org/project/pywin32/)
  * 需要 [ADB Platform Tools](https://developer.android.com/studio/releases/platform-tools) 与 `main.py` 在同一目录中
    * 只需将 `platform-tools` 解压到您的桌面，然后将 `main.py` 放入该文件夹，运行脚本时应该可以工作

* [**批处理脚本**](https://gist.github.com/AtlasTheProto/1f03c3aeac70c4af5b4f2fcd9b9273c0)
  
  * 需要 [ADB Platform Tools](https://developer.android.com/studio/releases/platform-tools)，编辑第 2 行的路径以指向您解压 `platform-tools` 的目录
  * 每次您（重新）连接头显时都需要运行

### 选项 2 - [SideQuest](https://sidequestvr.com/setup-howto)

* 确保 SideQuest 正在运行，并且头显已授权 USB 连接到电脑
* 打开 SideQuest 中的“运行 ADB 命令”菜单（右上角，带箭头的框）
* 单击“自定义命令”并运行这些 adb 命令：
  * `adb forward tcp:9943 tcp:9943`
  * `adb forward tcp:9944 tcp:9944`
  * 每次您（重新）连接头显时都需要运行这些命令。
* 保持 SideQuest 打开，直到您想关闭连接。

***

完成后，头显现在应该通过 USB 建立连接。
