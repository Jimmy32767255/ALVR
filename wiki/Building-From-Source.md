ALVR can be built on Windows and Linux. The following instructions are for both OSes.

# 常见先决条件

首选 IDE（可选）：带有 rust-analyzer 扩展的 Visual Studio Code

您需要安装 [rustup](https://www.rust-lang.org/tools/install)。

在 Windows 上，您还需要 [Chocolatey](https://chocolatey.org/install)。

要克隆存储库，请使用 `git clone --recurse-submodules https://github.com/alvr-org/ALVR.git`。
如果您之前没有子模块克隆过存储库，只需在其内部运行 `git submodule update --init --checkout --recursive`。

# 流媒体构建

首先，您需要准备一些额外的资源以进行构建。

如果您使用的是 Linux，请安装以下附加软件包：

* **Arch**
  
  注意：在撰写本文时，Arch gcc 太新，无法与 nvcc 兼容。这意味着没有简洁的方法来编译 Nvidia 兼容的构建。推荐的解决方法是在某种容器化环境中构建。这已成功通过 nixos 和 flatpak 完成 - 但尚未记录。

  ```bash
  sudo pacman -S clang curl nasm pkgconf yasm vulkan-headers libva-mesa-driver unzip ffmpeg libpipewire
  ```
  
  * 也可以使用 [`alvr-git`](https://aur.archlinux.org/packages/alvr-git) [AUR 软件包](https://wiki.archlinux.org/title/Arch_User_Repository) 自动完成此操作。

* **Gentoo**
  
  * `media-video/ffmpeg >= 4.4 [encode libdrm vulkan vaapi]`
  * `sys-libs/libunwind`
  * `dev-lang/rust >= 1.72`
  * `media-video/pipewire [jacksdk]`

* **Debian 12 / Ubuntu 20.04 / Pop!_OS 20.04**
  
  ```bash
  sudo apt install pulseaudio-utils build-essential pkg-config libclang-dev libssl-dev libasound2-dev libjack-dev libgtk-3-dev libvulkan-dev libunwind-dev gcc yasm nasm curl libx264-dev libx265-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libdrm-dev libva-dev libvulkan-dev vulkan-headers libpipewire-0.3-dev libspa-0.3-dev git
  ```

  * 注意：Libpipewire/libspa 必须至少是 0.3.49 版本 - 确保使用上游 pipewire <https://github.com/pipewire-debian/pipewire-debian>

* **Fedora**
  
  ```bash
  sudo dnf groupinstall 'Development Tools' | 用于 c++ 和构建工具
  sudo dnf install nasm yasm libdrm-devel vulkan-headers pipewire-jack-audio-connection-kit-devel atk-devel gdk-pixbuf2-devel cairo-devel rust-gdk0.15-devel x264-devel vulkan-devel libunwind-devel clang openssl-devel alsa-lib-devel libva-devel pipewire-devel git
  ```
  
如果您使用的是 Nvidia，请参阅 [Fedora cuda 安装](https://github.com/Jimmy32767255/ALVR-CN/wiki/Building-From-Source#fedora-cuda-installation)

移动到项目的根目录，然后运行此命令（注意以下要点）：

```bash
cargo xtask prepare-deps --platform [您的平台] [--gpl] [--no-nvidia]
```

* 将 `[您的平台]` 替换为您的计算机操作系统，可以是 `windows` 或 `linux`
* **仅限 Windows：** 如果您想在 ALVR 流媒体中下载、构建和捆绑 FFmpeg，请使用 `--gpl` 标志。请记住，这仅用于软件编码。顾名思义，如果您使用此标志，则只能将最终软件包作为 GPLv2.0 许可重新分发；因为 x264 编码器。
* **仅限 Linux：** 如果您有 AMD GPU，请使用 `--no-nvidia` 标志。

接下来是流媒体的正确构建。运行以下命令：

```bash
cargo xtask build-streamer --release [--gpl]
```

**仅限 Windows：** 同样，只有当您想捆绑 FFmpeg 时才需要 `--gpl` 标志。

您可以在 `build/alvr_streamer_[您的平台]` 中找到生成的软件包

如果您想编辑和重新构建代码，可以跳过 `prepare-deps` 命令，只运行 `build-streamer` 命令。

# Fedora CUDA 安装

本节适用于希望在 Fedora 上使用 Nvidia GPU 并需要安装 CUDA 的用户。

**重要提示：**

*   **内核模块：** Nvidia 内核模块必须加载才能使 CUDA 正常运行。确保它们已正确安装和加载。
*   **驱动版本：** 使用最新的稳定 Nvidia 驱动。过时的驱动可能会导致问题。
*   **安全启动：** 如果启用了安全启动，您可能需要对 Nvidia 内核模块进行签名。请参阅您的发行版文档以获取说明。

**安装步骤：**

1.  **添加 RPM Fusion 仓库：**

    ```bash
    sudo dnf install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm
    ```

2.  **安装 Nvidia 驱动和 CUDA 工具包：**

    ```bash
    sudo dnf install akmod-nvidia # 适用于旧显卡的 akmod-nvidia-340xx
    sudo dnf install xorg-x11-drv-nvidia-cuda # 或 xorg-x11-drv-nvidia-cuda-340xx
    ```

    *   安装后重启系统，以确保新驱动已加载。

3.  **验证 CUDA 安装：**

    重启后，打开终端并运行：

    ```bash
    nvidia-smi
    ```

    您应该看到类似以下的输出，表明您的 Nvidia 驱动和 CUDA 正在工作：

    ```
    +-----------------------------------------------------------------------------+
    | NVIDIA-SMI 535.104.05   Driver Version: 535.104.05   CUDA Version: 12.2     |
    |-------------------------------+----------------------+----------------------+
    | GPU  Name        Persistence-M| Bus-Id        Disp.A | Volatile Uncorr. ECC |
    | Fan  Temp  Perf  Pwr:Usage/Cap|         Memory-Usage | GPU-Util  Compute M. |
    |                               |                      |               MEX    |
    |===============================+======================+======================|
    |   0  NVIDIA GeForce ...  Off  | 00000000:01:00.0 Off |                  0 |
    | N/A   40C    P8    10W / 120W |      1MiB /  8192MiB |      0%      Default |
    +-------------------------------+----------------------+----------------------+
    ```

    此外，您还可以使用以下命令检查 CUDA 版本：

    ```bash
    nvcc --version
    ```

    这将输出 CUDA 编译器版本。

4.  **设置环境变量（可选但推荐）：**

    将 CUDA 工具包添加到您的 PATH 和 LD_LIBRARY_PATH。您可以将这些行添加到您的 `~/.bashrc` 或 `~/.zshrc` 文件中：

    ```bash
    export PATH=/usr/local/cuda/bin${PATH:+:${PATH}}
    export LD_LIBRARY_PATH=/usr/local/cuda/lib64${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}
    ```

    添加后，请 source 您的 shell 配置文件：

    ```bash
    source ~/.bashrc # 或 source ~/.zshrc
    ```

    这些步骤应该能让 CUDA 在您的 Fedora 系统上运行，以便与 ALVR 一起使用。


# Android 应用构建

## 1. 安装必要的软件包

要构建应用，您需要安装：

* [Android Studio](https://developer.android.com/studio) 或 [sdkmanager](https://developer.android.com/studio/command-line/sdkmanager)
* Android SDK Platform-Tools 29 (Android 10)
* 最新的 Android NDK（目前为 v25.1.8937393）

在 Linux 上，Android 工具的特定软件包名称可能因发行版而异，请参阅 Wiki 以获取更多信息：

* Gentoo:
  * <https://wiki.gentoo.org/wiki/Android>
* Arch:
  * <https://wiki.archlinux.org/title/Android>
* Debian:
  * <https://wiki.debian.org/AndroidStudio>
* Ubuntu:
  * <https://help.ubuntu.com/community/AndroidSDK>
* Pop!_OS:
  * N/A

上述三个开发应用程序可以从上游安装；尽管负责所需工具的软件包和设置可能因发行版而异，包括：

* **Arch**
  * 软件包可能有所不同，请查阅 Arch Wiki 的 [Android](https://wiki.archlinux.org/title/Android) 页面。
* **Gentoo**
  * `dev-util/android-studio`
  * `dev-util/android-sdk-update-manager`
  * `dev-util/android-ndk >= 25.1`

对于 Debian，需要启用 `non-free` 仓库：

* **Debian 12 / Ubuntu 22.10 / Pop!_OS 22.10**
  
  ```bash
  sudo apt install android-sdk-platform-tools-common sdkmanager google-android-ndk-r26b-installer
  ```
  
## 2. 设置环境变量

对于 Windows，设置环境变量：

* `JAVA_HOME`:
  * 示例: `C:\Program Files\Android\Android Studio\jre`
* `ANDROID_HOME`:
  * 示例: `%LOCALAPPDATA%\Android\Sdk`
* `ANDROID_NDK_HOME`:
  * 示例：`%LOCALAPPDATA%\Android\Sdk\ndk\25.1.8937393`

对于 Linux，环境变量的正确目录可能因安装类型而异。有关更多信息，请参阅您的发行版维基页面：

* Gentoo：
  * <https://wiki.gentoo.org/wiki/Android>
* Ubuntu：
  * <https://help.ubuntu.com/community/AndroidSDK#Post-Installation_Configuration>

上面未列出的发行版维基没有提及环境变量，但通常它们会是：

* `JAVA_HOME`：
  * `/usr/lib/jvm/default-java/bin`
* `ANDROID_HOME`：
  * Arch：`~/Android/Sdk`
  * Gentoo：`~/Android`
  * Debian / Ubuntu / Pop!_OS：`~/AndroidSDK`
* `ANDROID_NDK_HOME`：
  * Arch：`/opt/android-sdk/ndk`
  * Linux：`/usr/lib/android-sdk/ndk`

## 3. 构建

首先，您需要准备一些额外的资源以进行构建。移动到项目的根目录，然后运行此命令：

```bash
cargo xtask prepare-deps --platform android
```

在构建应用程序之前，Android 要求我们同意许可证，否则构建应用程序将停止并失败。要接受协议，请按照相应操作系统的说明进行操作：

* Windows：
  
  ```shell
  cd "%ANDROID_SDK_ROOT%\tools\bin"
  sdkmanager.bat --licenses
  ```

* Linux：
  
  ```bash
  cd ~/AndroidSDK
  sdkmanager --licenses
  ```

接下来是应用程序的正确构建。运行以下命令：

```bash
cargo xtask build-client --release
```

构建的 APK 将位于 `build/alvr_client_quest` 中。然后您可以使用 adb 或 SideQuest 将其安装到您的头显上。

要构建和运行：

```bash
cd alvr/client_openxr
cargo apk run
```

您需要通过 USB 连接头显并打开屏幕才能成功启动调试器和 logcat。

# 故障排除 (Linux)

在某些发行版上，Steam Native 运行 ALVR 效果更好。要在 Ubuntu 上获取 Steam Native，请运行以下命令：

```bash
env STEAM_RUNTIME=0 steam
```

在 Arch Linux 上，您还可以通过从 multilib 存储库下载 `steam-native-runtime` 包来获取所有必需的库：

```bash
sudo pacman -S steam-native-runtime
```

此时可能缺少依赖项，因此请运行：

```bash
cd ~/.steam/root/ubuntu12_32
file * | grep ELF | cut -d: -f1 | LD_LIBRARY_PATH=. xargs ldd | grep 'not found' | sort | uniq
```

一些依赖项必须手动修复，例如，与其强制降级到 libffi 版本 6（这可能会降级系统中的许多内容），不如使用符号链接（需要测试）：

```bash
cd /lib/i386-linux-gnu
ln -s libffi.so.7 libffi.so.6
```

和

```bash
cd /lib/x86_64-linux-gnu
ln -s libffi.so.7 libffi.so.6
```

少数依赖项由发行版控制，您可以尝试自行承担风险导入软件包，可能需要使用 alien 或一些强制导入命令，但这不是推荐的（会将您的系统变成依赖项混合的混乱），也不受支持！
