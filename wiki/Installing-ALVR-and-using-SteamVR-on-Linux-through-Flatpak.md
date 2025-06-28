## 免责声明

1. Flatpak 支持是实验性的——但它似乎确实有效。需要一些手动步骤！

2. 不支持也不测试原生 Linux SteamVR 实用程序应用程序，例如 OpenVRAS，请自行承担风险使用

3. 防火墙配置不起作用

4. 任何影响主机的脚本都将在沙盒中运行

5. 有时，启动仪表板时会启动一个新的 Steam 实例。要解决此问题，请关闭 ALVR 和 Steam，然后启动 Steam。一旦 Steam 打开到商店界面，就启动 ALVR 仪表板。

6. 用户必须自行设置 xdg 快捷方式——见下文。如果没有 xdg 条目，启动器必须从终端运行。

```sh
flatpak run --command=alvr_launcher com.valvesoftware.Steam
```

8. 这似乎适用于 Steam Flatpak 和原生 Steam——它通过 xdg-open 调用。但不建议同时安装两个版本的 Steam，因为这会造成歧义。

## 依赖项

首先，必须从您的发行版存储库安装 flatpak。请参阅 [此页面](https://flatpak.org/setup/) 查找您的发行版的说明。

## 设置

与原生 Steam 相比，Flatpak Steam 需要额外的步骤。安装 SteamVR 后，运行以下命令：

```sh
sudo setcap CAP_SYS_NICE+eip ~/.var/app/com.valvesoftware.Steam/data/Steam/steamapps/common/SteamVR/bin/linux64/vrcompositor-launcher
```

此命令通常由 SteamVR 运行，但由于 Flatpak 沙盒中缺少 sudo 访问权限，因此必须在 Flatpak 沙盒外部运行。运行命令后，运行一次 SteamVR，然后关闭它。

### SteamVR 自定义启动选项

将以下内容添加到 SteamVR 的启动选项中：

```
steam flatpak: --enable-standalone-mode
native steam: -enable-standalone-mode
```

这是为了防止 SteamVR 尝试启动 ALVR。
```

### 创建 pipewire 错误失败
使用 flatseal 为 Steam 添加权限 - 在文件系统部分 - “其他文件” - 添加新条目，内容为：“xdg-run/pipewire-0”
应该在那里看到一些其他权限，例如“xdg-music:ro”、“xdg-pictures:ro”，以及可能更多用于其他集成（如 Discord）的权限。
TODO：添加一张关于这具体是什么样子的漂亮图片，或者一个 shell 命令来完成它

## 安装

1. 从 Flathub 安装 ALVR：

```sh
flatpak install flathub rs.alvr.ALVR
```

2. 运行 ALVR：

```sh
flatpak run rs.alvr.ALVR
```

3. 在您的头戴设备上安装 ALVR。您可以在 ALVR 仪表板中找到 APK，或者从 [ALVR 网站](https://alvr-org.github.io/ALVR/download) 下载。

4. 将您的头戴设备连接到 ALVR。


## 注意事项

### 运行启动器

建议用户设置 xdg 快捷方式——但启动器也可以通过以下命令从终端运行：
```sh
flatpak run --command=alvr_launcher com.valvesoftware.Steam
```

`alvr/xtask/flatpak` 目录中提供了名为 `com.valvesoftware.Steam.Utility.alvr.desktop` 的图标和桌面文件。将其移动到系统上其他桌面文件所在的位置，以便无需终端即可运行仪表板。

```sh
# systemwide shortcut
# sudo cp com.valvesoftware.Steam.Utility.alvr.desktop /var/lib/flatpak/exports/share/applications/ 

# users local folder
cp com.valvesoftware.Steam.Utility.alvr.desktop $HOME/.local/share/flatpak/exports/share/applications/

# install icon as well
xdg-icon-resource install --size 256 alvr_icon.png application-alvr-launcher
```

快捷方式可能直到桌面会话刷新（例如注销然后重新登录）后才会出现。

### 实验性 - 通过 flatpak 启动器安装 APK
首先需要在主机上设置 adb，并在设备上启用 USB 调试。验证运行“adb devices”时设备是否显示并已授权。
脚本假定用户已安装 AndroidStudio 并在默认位置 ($HOME/.android/adbkey.pub) 拥有密钥——如有必要请更改。
git 中提供了便利脚本：run_with_adb_keys.sh
如果密钥暴露给默认位置的 flatpak，它很可能无需更多更改即可工作。
```
export ADB_VENDOR_KEYS=~/.android/adbkey.pub
flatpak override --user --filesystem=~/.android com.valvesoftware.Steam.Utility.alvr
flatpak run --env=ADB_VENDOR_KEYS=$ADB_VENDOR_KEYS --command=alvr_launcher com.valvesoftware.Steam
```

### Wayland 变量导致 SteamVR 错误：
确保 QT_QPA_PLATFORM 变量允许 x11 选项——否则 SteamVR 会崩溃。从终端启动以查看错误。
如果您已全局修改此变量以强制某些程序（如 GameScope）使用 Wayland，这可能会成为问题。
您可以通过设置传递给 SteamVR 的变量来解决此问题。
SteamVR 的自定义启动选项示例——包括 QT_QPA_PLATFORM 和 vrmonitor 修复：

```
QT_QPA_PLATFORM=xcb ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%
```

### 混合显卡
如果使用台式机，建议禁用集显——这样会更简单。
如果使用笔记本电脑，则必须传递额外选项以确保使用独显。这些选项是除了前面提到的其他选项之外的。

#### AMD/Intel 集成显卡 + AMD/Intel 独立显卡
将 DRI_PRIME=1 %command% 放入 SteamVR 的命令行选项以及您打算与 ALVR 一起玩的所有 VR 游戏的选项中。
```
DRI_PRIME=1 QT_QPA_PLATFORM=xcb ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%
```

#### AMD/Intel 集成显卡 + Nvidia 独立显卡
将 __NV_PRIME_RENDER_OFFLOAD=1 __VK_LAYER_NV_optimus=NVIDIA_only __GLX_VENDOR_LIBRARY_NAME=nvidia %command% 放入 SteamVR 的命令行选项以及您打算与 ALVR 一起玩的所有 VR 游戏的选项中。同样——除了其他选项之外。
```
__NV_PRIME_RENDER_OFFLOAD=1 __VK_LAYER_NV_optimus=NVIDIA_only __GLX_VENDOR_LIBRARY_NAME=nvidia QT_QPA_PLATFORM=xcb ~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%
```

### 其他应用程序

由于 Flatpak 沙盒的限制，不支持通过 Steam 以外的方式启动的其他应用程序。

各种 SteamVR 实用程序，例如 [WlxOverlay](https://github.com/galister/WlxOverlay) 和 [OpenVR-AdvancedSettings](https://github.com/OpenVR-Advanced-Settings/OpenVR-AdvancedSettings)，由于它们使用 AppImage，无法在 Flatpak 沙盒中运行。但是，解压提供的 AppImage 或从源代码构建实用程序并在沙盒中运行它们的二进制文件（类似于 `alvr_dashboard`）可能有效，但不能保证它们能正常工作。

（撰写本文时它确实有效）
下载 wlx-overlay-s appimage。
使其可执行 (chmod +x Wlx-Overlay-xxx.Appimage)。
解压它 (./Wlx-Overlay-xxx.Appimage --app-image-extract)
使用 flatseal 或终端将文件夹暴露给 steam flatpak（例如 ~/test，应该与上面 pipewire 修复在同一部分）
将解压的文件复制到暴露的文件夹中。
从终端测试：flatpak run --command=bash com.valvesoftware.Steam (cd ~/test/squasroot-fs && ./Apprun)
要创建桌面快捷方式，请使用类似 flatpak run --command=~/test/squashroot-fs/Apprun com.valvesoftware.Steam 的命令。



某些应用程序（例如 [Godot](https://godotengine.org)）支持 OpenXR。但是，除非它们在 Steam Flatpak 沙盒中启动，否则它们将无法与 Steam Flatpak 配合使用。有关更多详细信息，请参阅 [此处](https://github.com/flathub/com.valvesoftware.Steam/issues/1010)。
