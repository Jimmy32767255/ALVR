<p align="center"> <img width="500" src="resources/ALVR-Grey.svg"/> </p>

# ALVR CN - 空气之光 VR 汉化版

[![badge-discord][]][link-discord] [![badge-matrix][]][link-matrix] [![badge-opencollective][]][link-opencollective]

通过 Wi-Fi 将 VR 游戏从您的 PC 流式传输到您的头戴设备。
这是 [ALVR](https://github.com/polygraphene/ALVR) 的一个分支。

### 最新版本直接下载：
### [Windows 启动器](https://github.com/alvr-org/ALVR/releases/latest/download/alvr_launcher_windows.zip) | [Linux 启动器](https://github.com/alvr-org/ALVR/releases/latest/download/alvr_launcher_linux.tar.gz)

## 兼容性

| VR 头戴设备 | 支持 |
| :--------------------------: | :------------------------------------------------------------------------------------: |
| Apple Vision Pro | :heavy_check_mark: ([商店链接](https://apps.apple.com/app/alvr/id6479728026)) |
| Quest 1/2/3/3S/Pro | :heavy_check_mark: ([商店链接](https://www.meta.com/experiences/7674846229245715) *) |
| Pico Neo 3/4/4 Ultra | :heavy_check_mark: |
| Play For Dream YVR 1/2/MR | :heavy_check_mark: |
| Vive Focus 3/Vision/XR Elite | :heavy_check_mark: |
| Lynx R1 | :heavy_check_mark: |
| PhoneVR (智能手机) | :heavy_check_mark: ** ([仓库](https://github.com/PhoneVR-Developers/PhoneVR)) |
| Android/Monado | :warning: ** |
| Oculus Go | :x: ([旧仓库](https://github.com/polygraphene/ALVR)) |

\* : Quest 1 版本的 ALVR 无法通过 Meta 商店获取。
\** : 仅在部分智能手机上可用，测试不足。

| PC 操作系统 | 支持 |
| :------------: | :---------------------------------------------------------------------------: |
| Windows 10/11 | :heavy_check_mark: ([商店链接](https://store.steampowered.com/app/3312710)) |
| Windows XP/7/8 | :x: |
| Linux | :heavy_check_mark:*** |
| macOS | :x: |

\*** : 请查看 Wiki 获取兼容性信息

### 要求

- 支持的独立 VR 头戴设备（参见上表兼容性）

- SteamVR

- 高端游戏 PC
    - 参见上表操作系统兼容性。
    - 支持 NVENC 的 NVIDIA GPU（1000 GTX 系列或更高版本）（或支持 AMF VCE 的 AMD GPU），并安装最新驱动程序。
    - 带有板载（Intel HD, AMD iGPU）和附加独立 GPU（NVidia GTX/RTX, AMD HD/R5/R7）的笔记本电脑：您应该将独立 GPU 或“高性能图形适配器”分配给 ALVR、SteamVR 应用程序，以获得最佳性能和兼容性。（NVidia：Nvidia 控制面板->3D 设置->应用程序设置；AMD：类似方式）

- 802.11ac 5Ghz 无线或以太网有线连接
    - 建议头戴设备使用 802.11ac 5Ghz，PC 使用以太网。
    - 您需要将 PC 和头戴设备都连接到同一路由器（或使用 [此处](https://github.com/alvr-org/ALVR/wiki/ALVR-v14-and-Above) 描述的路由连接）

## 安装

请遵循 [此处](https://github.com/alvr-org/ALVR/wiki/Installation-guide) 的安装指南。

## 故障排除

- 请查看 [故障排除](https://github.com/alvr-org/ALVR/wiki/Troubleshooting) 页面，如果适用，也请查看 [Linux 故障排除](https://github.com/alvr-org/ALVR/wiki/Linux-Troubleshooting)。
- 配置建议和信息可在 [此处](https://github.com/alvr-org/ALVR/wiki/Information-and-Recommendations) 找到。

## 卸载

打开 `ALVR Dashboard.exe`，进入 `Installation` 选项卡，然后点击 `Remove firewall rules`。关闭 ALVR 窗口并删除 ALVR 文件夹。

## 从源代码构建

您可以遵循 [此处](https://github.com/alvr-org/ALVR/wiki/Building-From-Source) 的指南。

## 许可证

ALVR 采用 [MIT 许可证](LICENSE) 授权。

## 隐私政策

ALVR 应用程序不直接收集任何类型的数据。

## 捐赠

如果您想支持本项目，可以向我们的 [Open Source Collective 账户](https://opencollective.com/alvr) 捐款。

[badge-discord]: https://img.shields.io/discord/720612397580025886?style=for-the-badge&logo=discord&color=5865F2 "加入我们的 Discord"
[link-discord]: https://discord.gg/ALVR
[badge-matrix]: https://img.shields.io/static/v1?label=chat&message=%23alvr&style=for-the-badge&logo=matrix&color=blueviolet "加入我们的 Matrix"
[link-matrix]: https://matrix.to/#/#alvr:ckie.dev?via=ckie.dev
[badge-opencollective]: https://img.shields.io/opencollective/all/alvr?style=for-the-badge&logo=opencollective&color=79a3e6 "捐赠"
[link-opencollective]: https://opencollective.com/alvr
