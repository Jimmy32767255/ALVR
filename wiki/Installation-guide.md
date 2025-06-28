## 启动器 (BETA)

启动器将允许您管理 ALVR 流媒体的旧版本、当前版本和新版本安装，并允许自动安装和升级到头显上的特定 ALVR 应用版本。

### 安装

* 从发布 [下载页面](https://github.com/Jimmy32767255/ALVR-CN/releases/latest) 下载 `alvr_launcher_windows.zip`（Windows 上）或 `alvr_launcher_linux.tar.gz`（Linux 上），并解压到一个只包含 ASCII 字符（仅英文）且无需管理员或 root 权限即可编辑的路径。
* 运行 `ALVR Launcher.exe`（Windows 上）或 `alvr_launcher_linux/ALVR Launcher`（Linux 上）
* 按下 `添加版本` 按钮
* 对于默认安装，保持通道和版本不变，然后按下 `安装`
* 等待下载和安装完成（取决于您的网络连接）
* 要在头显上安装 ALVR 应用，请使用 `安装 APK` 按钮
* 在列表中，要打开流媒体应用（PC），请按下 `启动`。您将看到一个设置向导。按照设置向导设置防火墙规则和其他设置。

### 使用

* 在通过 ALVR 启动 SteamVR 之前，请先安装它。首次启动会导致 SteamVR 为空白，ALVR 将无法工作 - 关闭并重新启动。它将注册驱动程序并应该可以工作。
* 在您的头显上启动 ALVR 应用。当头显屏幕亮起时，点击设备条目旁边的 `信任`（在 PC 上的 ALVR 流媒体应用中，`设备` 选项卡中）以开始流式传输。
* 您可以在 PC 上的 `设置` 选项卡中更改设置。大多数设置需要重新启动 SteamVR 才能生效。使用右下角的相应按钮。

如果遇到任何问题，请访问 [故障排除页面](https://github.com/Jimmy32767255/ALVR-CN/wiki/Troubleshooting)。

## Windows 上的麦克风设置

要在 Windows 上的 ALVR 中使用麦克风，您需要安装 **VB-Audio Cable**（或等效软件）。但是，如果 VB-Audio Cable 已安装但无法与 ALVR 配合使用，**或者您遇到任何问题**，则值得按照以下步骤重新安装和正确配置它。

### **1. 安装或重新安装虚拟音频线**
1. **下载** 最新精简版 [虚拟音频线](https://software.muzychenko.net/freeware/vac470lite.zip)。
2. **解压** ZIP 存档。
3. 打开解压后的文件夹，以管理员身份运行 **"setup64.exe"**。

### **2. 配置 Windows 声音设置**
1. **打开** Windows 声音设置（`Win + I` → “声音”）。
2. **在输出设备下**：
   - **不要将任何“虚拟音频线”设置为默认输出**，否则您会听到自己的声音。选择您的耳机或您正在使用的任何设备。

### **3. 配置 ALVR**
1. **打开 ALVR** 并转到**设置**。
2. 将**头显扬声器** → **系统默认**。
3. 将**头显麦克风** → **VB Cable**。

## 高级安装

### 使用 Sidequest 安装应用

* 在您的 PC 上安装 SideQuest 并启用头显上的开发者模式。您可以按照 [此指南](https://sidequestvr.com/setup-howto) 进行操作。
* 将您的头显连接到 Sidequest。如果您有 Quest、Pico 和其他兼容设备，请 [在此处](https://sidequestvr.com/app/9) 下载 ALVR 应用。

### 手动安装 ALVR 流媒体

还有一个适用于 PC 的便携版本，需要更多手动步骤才能使其工作。

#### Windows

* 从最新版本 [下载页面](https://github.com/Jimmy32767255/ALVR-CN/releases/latest) 下载 `alvr_streamer_windows.zip`。
* 解压到一个只包含 ASCII 字符且无需管理员权限即可编辑的路径。
* 运行

#### Linux

* 从发布 [下载页面](https://github.com/Jimmy32767255/ALVR-CN/releases/latest) 下载 `alvr_streamer_linux.tar.gz`，并解压。
* 运行 `bin/alvr_dashboard`

#### 每夜版

如果您想尽早获得新功能或想帮助测试，可以安装每夜版。

[在此处](https://github.com/alvr-org/ALVR-nightly/releases/latest) 下载最新的每夜版流媒体。

由于每夜版可能不稳定，请始终使用 PC 和头显的匹配版本。它们每天更新一次。

### Arch Linux (AUR)

* 如果您没有安装 `rustup` 和 rust 工具链，请安装它们：<https://wiki.archlinux.org/title/Rust#Arch_Linux_package>。
* 安装 [alvr](https://aur.archlinux.org/packages/alvr)<sup>AUR</sup>（稳定版，amdgpu），或 [alvr-nvidia](https://aur.archlinux.org/packages/alvr-nvidia)<sup>AUR</sup>（稳定版，nvidia），或 [alvr-git](https://aur.archlinux.org/packages/alvr-git)<sup>AUR</sup>（每夜版，不稳定）
* 安装 SteamVR，**启动一次**然后关闭它。
* 从您的 DE 应用程序启动器运行 `alvr_dashboard` 或 ALVR。

### Flatpak

对于 Flatpak 用户，请参阅 [此处](https://github.com/Jimmy32767255/ALVR-CN/wiki/Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak) 的说明。

## 高级用法

### 将 ALVR 与第三方驱动程序一起使用

默认情况下，ALVR 在启动前会禁用其他 SteamVR 驱动程序。在这些驱动程序中，有用于全身追踪的 [Driver4VR](https://www.driver4vr.com/)。ALVR 禁用这些驱动程序是为了最大限度地提高与每个 PC 设置的兼容性。您可以通过手动注册 ALVR 驱动程序来禁用此行为。转到 `安装` 选项卡并单击 `注册 ALVR 驱动程序`。下次启动 ALVR 时，您将能够同时使用其他驱动程序。

### 将 ALVR 与 SteamVR 一起启动

您可以跳过 ALVR 仪表板，并自动与 SteamVR 一起打开 ALVR。

**注意：** 您只能在 SteamVR 尚未运行时执行此操作。否则，驱动程序可能会在关机时取消注册。

打开 ALVR，转到 `安装` 选项卡并单击 `注册 ALVR 驱动程序`。

### 通过 USB 线将头显连接到 PC

请查看 [此处](https://github.com/Jimmy32767255/ALVR-CN/wiki/ALVR-wired-setup-(ALVR-over-USB)) 的指南。
