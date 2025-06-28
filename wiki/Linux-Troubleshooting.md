## （！强制性，如果尚未应用请立即修复！）即使 SteamVR 显示运动，头戴设备仍黑屏，仪表板未检测到已启动的 ALVR/SteamVR

SteamVR 运行的 Steam 运行时会破坏 SteamVR 加载的 ALVR 驱动程序。
这会导致头戴设备屏幕保持黑色，或者报告 pipewire 设备丢失的错误，甚至可能导致 SteamVR 崩溃。

### 修复

将 `~/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%` 添加到 SteamVR 的命令行选项中（SteamVR -> 管理/右键单击 -> 属性 -> 通用 -> 启动选项）。

此路径可能因您的 Steam 安装而异，在这种情况下 SteamVR 将根本无法启动。如果出现这种情况，您可以通过 Steam 设置 -> 存储来找出实际路径。
然后选择带有星形表情符号 (⭐) 的存储位置，并获取使用情况统计信息正上方的路径。将此路径添加到 `steamapps/common/SteamVR/bin/vrmonitor.sh` 的前面。
最后，将整个路径放入 SteamVR 命令行选项中，而不是其他路径。

### Hyprland/Sway/Wlroots Qt 修复

如果您使用的是 hyprland、sway 或其他基于 wlroots 的 Wayland 合成器，您可能需要在命令行前加上 `QT_QPA_PLATFORM=xcb`，这样 SteamVR 的完整命令行将变为：
`QT_QPA_PLATFORM=xcb ~/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%`。

相关问题：
[[BUG] 在基于 wlroots 的 Wayland 合成器（sway、hyprland 等）上没有 SteamVR UI 的解决方法](https://github.com/ValveSoftware/SteamVR-for-Linux/issues/637)。


## SteamVR 未检测到 ALVR 驱动程序（即使在 vrmonitor 修复后）

可能与 Arch AUR 软件包有关（可能是在基于 Nvidia 的系统上未安装 Nvidia 版本的 `alvr-nvidia`，或者只是普遍情况）。

### 修复

尝试使用发布页面上的启动器或便携式 .tar.gz 版本。

## 伪影、无 SteamVR 叠加层或流式传输视图中出现图形故障

可能与您的系统上存在 AMD amdvlk 或 amdgpu-pro 驱动程序有关。

如果您的系统上安装了 Amdvlk，它会覆盖其他 Vulkan 驱动程序并导致 SteamVR 崩溃。请改用 `vulkan-radeon` 驱动程序（又名 radv）。

### 修复

通过查看 `ls /usr/share/vulkan/icd.d/ | grep -e amd_icd -e amd_pro` 是否显示任何内容，检查是否安装了 amdvlk 或 amdgpu-pro。
如果是，请从您的系统中卸载 amdvlk 和/或 amdgpu-pro 驱动程序。（此方法可能无法捕获所有安装，因为发行版存在差异）

在 Arch 上，首先安装 `vulkan-radeon` 并卸载其他驱动程序。

## 创建 VAAPI 编码器失败

游戏流出现块状或崩溃，然后桌面上出现错误窗口，显示：
> 无法创建 VAAPI 编码器：无法打开视频编码器编解码器：功能未实现。请确保您已安装 VAAPI 运行时。

### 修复

对于 Fedora：
 * 从 `mesa-va-drivers` 切换到 `mesa-va-drivers-freeworld`。请参阅 [如何操作的指南](https://fostips.com/hardware-acceleration-video-fedora/) 或 [RPM 文档](https://rpmfusion.org/Howto/Multimedia)
对于 Arch（不要将 vaapi 用于 Nvidia）：
 * 按照 [此页面](https://wiki.archlinux.org/title/Hardware_video_acceleration#Installation) 进行操作
然后重启您的机器。

对于其他发行版（例如 Manjaro）：
 * 安装包含 h264/hevc 编码所需专有编解码器的非自由版本 mesa/vaapi 驱动程序

## Nvidia 驱动版本要求

ALVR 要求驱动版本至少为 535，CUDA 版本至少为 12.1。如果不是，SteamVR 或编码器可能无法工作。

### 修复

安装至少所需版本的驱动程序，并确保您已安装 CUDA 且版本至少为 12.1。

如果持续出现提示未检测到 CUDA 的错误，请尝试使用最新的 ALVR 每夜构建版本。

## 仅使用集成显卡运行 ALVR

请注意，**仅**使用集成显卡运行 ALVR 是非常不明智的，因为在大多数情况下，这会导致非常差的性能（即使在 Steam Deck 等更强大的设备上，它仍然非常慢）。
在这种情况下，也不要期望一切都能完美运行，因为一些较旧的集成显卡可能根本没有最好的 Vulkan 支持，甚至可能完全无法工作。


## 混合显卡建议

### 一般建议

如果您有台式电脑并且可以从 BIOS/UEFI 中禁用集成 GPU，强烈建议您这样做，以避免处理混合显卡带来的多个问题。
如果您使用的是笔记本电脑并且（在大多数情况下）不允许禁用集成显卡，则必须采用以下方法。

### Amd/Intel 集成显卡 + Amd/Intel 独立显卡

将 `DRI_PRIME=1 ~/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%`（根据您的发行版调整 vrmonitor 路径）放入 SteamVR 的命令行选项以及您打算使用 ALVR 玩的所有 VR 游戏的命令行选项中。

### Amd/Intel 集成显卡 + Nvidia 独立显卡

将 `__NV_PRIME_RENDER_OFFLOAD=1 __VK_LAYER_NV_optimus=NVIDIA_only __GLX_VENDOR_LIBRARY_NAME=nvidia ~/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%`（根据您的发行版调整 vrmonitor 路径）放入 SteamVR 的命令行选项以及您打算使用 ALVR 玩的所有 VR 游戏的命令行选项中。

如果这导致诸如 `error in encoder thread: Failed to initialize vulkan frame context: Invalid argument` 之类的错误，请尝试以下方法：

`__NV_PRIME_RENDER_OFFLOAD=1 __VK_LAYER_NV_optimus=NVIDIA_only __GLX_VENDOR_LIBRARY_NAME=nvidia VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/nvidia_icd.json ~/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%`

- 同样，根据您的发行版调整 vrmonitor 路径
- 转到 `/usr/share/vulkan/icd.d` 并确保 `nvidia_icd.json` 存在。它也可能以 `nvidia_icd.x86_64.json` 的名称存在，在这种情况下，您应该相应地调整 `VK_ICD_FILENAMES`。

### Nvidia 独立 GPU 上 SteamVR 仪表板未在 VR 中渲染
如果您遇到 SteamVR 仪表板未在 VR 中渲染的问题，您可能需要通过 PRIME 渲染卸载来运行整个 Steam 客户端本身。如果 Steam 客户端已打开，请先完全关闭它，您可以通过单击左上角的 Steam 下拉菜单并选择退出。然后从终端运行：`__NV_PRIME_RENDER_OFFLOAD=1 __GLX_VENDOR_LIBRARY_NAME=nvidia steam-runtime`

## Wayland

在使用旧版 Gnome（< 47 版本）的 Wayland 下，您可能需要在 SteamVR 命令行选项中添加 `WAYLAND_DISPLAY='' ~/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%`（根据您的发行版调整 vrmonitor 路径）以强制 SteamVR 使用 XWayland。这解决了 DRM 租用不可用的问题。

## 画面抖动

与 SlimeVR 相关，可能会在 ALVR 的未来更新中修复。

### 修复

仅在您至少连接并获得 ALVR 图像一次后才启动 SlimeVR 服务器。

## 109 错误

出现 109 错误或其他错误。

### 修复

在通过 ALVR 启动 SteamVR 之前，请先启动 Steam。如果 SteamVR 已经启动，请重新启动它。

## 无音频或麦克风

即使在预设中启用了音频或麦克风，仍然听不到声音或没有人能听到我的声音。

### 修复

确保在连接头戴设备**后**，在设备列表中选择 `ALVR Audio` 和 `ALVR Microphone` 作为默认设备。一旦头戴设备断开连接，设备将被移除。如果将其设置为默认设备，则每当它们出现时都会自动选择，您无需再次手动操作。
如果您似乎没有音频设备，或者日志中出现 pipewire 错误，请使用命令 `pipewire --version` 检查是否安装了 `pipewire` 并且版本至少为 `0.3.49`。
对于较旧的（<=22.04 或 debian <=11）Ubuntu 或基于 Debian 的发行版，您可以查看 [pipewire-upstream](https://github.com/pipewire-debian/pipewire-debian) 页面以安装较新的 pipewire 版本。

## AMDGPU 性能低下和卡顿

这可能是由 [[PERF] 由于错误的电源配置文件模式导致 GPU 性能不佳 · Issue #469 · ValveSoftware/SteamVR-for-Linux · GitHub](https://github.com/ValveSoftware/SteamVR-for-Linux/issues/469) 引起的。

### 修复

强烈建议使用 CoreCtrl（使用您的发行版包管理器安装），并在设置中将您的 GPU 设置为 VR 配置文件，以及将 CPU 设置为性能配置文件（如果是旧的 Ryzen CPU）。

## OVR 高级设置

禁用 OVR 高级设置驱动程序，不要将其与 ALVR 一起使用。
它不兼容，并且会产生梯形延迟图，导致非常严重的视觉偏移。


## 绑定不工作/由于绑定 UI 导致 CPU 使用率过高

SteamVR 无法正确更新绑定、打开菜单，并且可能会占用过多的 CPU。

此问题是由于 SteamVR 的 Web 服务器发送大量请求，导致 Chromium UI 停滞并占用大量 CPU 引起的。

### 修复

应用以下补丁：`https://github.com/alvr-org/ALVR-Distrobox-Linux-Guide/blob/main/patch_bindings_spam.sh`
假设 Arch、Fedora 的默认路径 - 一行命令：`curl -s https://raw.githubusercontent.com/alvr-org/ALVR-Distrobox-Linux-Guide/main/patch_bindings_spam.sh | sh -s ~/.steam/steam/steamapps/common/SteamVR`
