## ALVR v14 及更高版本

这里解释了两种远程连接 PC 和头显的方法：端口转发和 ZeroTier。其主要目的是将头显连接到云 PC（如 ShadowPC）。

## 安全注意事项

* ALVR 协议没有任何加密或身份验证（除了 ALVR 流媒体中显示的 ALVR 设备 IP 地址以及在 ALVR 流媒体上添加设备的要求）。
* 建议通过互联网使用加密隧道 (VPN) 运行 ALVR。如果 VPN 不可行，则应通过 Windows 防火墙限制对 ALVR 流媒体（UDP 端口 9943 和 9944）的访问（只允许来自已知 ALVR 设备 IP 地址的连接），并且不应让 ALVR 流媒体无人看管地运行。
* **警告！** SteamVR 允许从 VR 头显控制桌面（即，**恶意 ALVR 设备可能会接管 PC**）。
* 正如许可证所述，ALVR “按原样”提供，不附带任何形式的担保（有关法律文本/定义，请参阅此 GitHub 存储库中的 `LICENSE` 文件）。您需要自行承担风险（尤其是在没有 VPN 的情况下通过互联网运行 ALVR）。

## 端口转发

端口转发允许连接位于不同 NAT（即本地网络）后面的设备。您需要拥有路由器的管理员权限。此方法具有最佳的流媒体性能。

**重要提示**：ALVR 不对流数据使用端到端加密。使用此方法，您需要注意连接容易受到“中间人”攻击。

1. 记下头显的公共 IP。您可以使用在线工具 [WhatIsMyIP](https://www.whatismyip.com/)。
2. 在路由器网页界面或应用程序中，为您的头显添加端口转发规则。您需要为 TCP 和 UDP 指定端口 9943 和 9944。
3. 连接到远程 PC 并打开 ALVR。在“设备”选项卡中，按“手动添加设备”。填写头显名称（您可以使用任何您想要的名称）、主机名（您可以在头显中打开 ALVR 应用程序时的欢迎屏幕中读取）、头显的远程 IP（即您在步骤 1 中获得的 IP），然后按“保存”。

您现在可以使用 ALVR 连接到您的远程 PC。

**注意**：公共 IP 经常变化。每次您想使用 ALVR 时，都需要检查您当前的公共 IP 是否与上次相同。如果 IP 发生变化，您可以使用流媒体上头显名称旁边的“编辑”按钮访问的“编辑连接”界面进行更新。

## ZeroTier

[ZeroTier](https://www.zerotier.com/) 是一款隧道软件，它使远程设备能够像在同一个本地网络中一样相互连接。

与端口转发方法相比：

优点：

* 无需访问路由器界面。
* 您无需经常在流媒体上更新公共 IP。
* 连接已加密。

缺点：

* 流媒体性能较差。您可能会遇到更多的图像和音频故障以及质量损失。

### 要求

* 适用于您 PC 的 [ZeroTier](https://www.zerotier.com/)
* 适用于您的 Quest 的 ZeroTier APK（您可以在线找到）
* SideQuest 或其他将 ZeroTier APK 安装到您的头显上的方法

### 安装

使用 SideQuest 的“安装 APK”功能将 ZeroTier APK 安装到您的 Quest，并在您的 PC 上下载并安装 ZeroTier。安装 ZeroTier 后，请按照 ZeroTier 官方的 [入门指南](https://zerotier.atlassian.net/wiki/spaces/SD/pages/8454145/Getting+Started+with+ZeroTier) 设置 ALVR 网络。在 Quest 和 PC 上都加入该网络。在 Quest 上，确保通过 ZeroTier 应用程序列表中网络的滑块打开网络（您可能会被提示允许 ZeroTier 创建 VPN 连接）。

在您的 PC 和 Quest 都连接到同一个 ZeroTier 网络后，我们需要手动将您的 Quest 添加到 ALVR 控制面板。为此，我们需要找到您的 Quest 的 ZeroTier IP。有两种方法可以做到这一点。

* 转到 ZeroTier 网络页面，在“成员”下找到您的 Quest，然后从那里复制托管 IP
* 或者，在您的 Quest 上的 ZeroTier 应用程序中，单击您创建的网络。IP 位于底部的“托管 IP”部分。

IP 应该类似于 `192.168.143.195`。如果末尾有 `/` 和几个数字，请将其与斜杠一起删除。

接下来，我们需要将 Quest 添加到 ALVR 控制面板。在您的头显上，启动 ALVR。然后在您 PC 上的 ALVR 控制面板上，单击“手动添加设备”按钮，提供名称和主机名（您可以从 Quest 上的 ALVR 的“信任”屏幕获取），然后输入我们从 ZeroTier 获取的 IP 地址。

至此，您应该已准备就绪。在 VR 中玩得开心！

### 故障排除

* 如果您的 Quest 无法连接到 ALVR，并且停留在“信任”屏幕，请尝试 ping 您的 Quest 的托管 IP 地址（我们之前获得的那个）。如果它显示“无路由到主机”或类似内容，则您的 Quest 无法看到您的 PC。请尝试按照上述步骤操作，以确保您没有遗漏任何内容。

## Tailscale

ZeroTier 的替代方案，设置过程几乎相同。根据您到数据中心的距离，这可能会有更好的延迟。
<https://tailscale.com/>

## n2n

[n2n](https://github.com/ntop/n2n) 是另一种 P2P VPN 解决方案，就像 ZeroTier 一样。您需要在具有公共可访问 IP 和端口的服务器上运行 _supernode_（或者至少您的 PC 和 Quest 可以访问它），并在您的 PC 和 Quest 上运行 _edge_ 节点。

它的优缺点与 ZeroTier 相似，但如果您关心隐私，它是自托管和开源的，尽管您需要一些网络和服务器部署知识。

### 要求

* 从源代码编译 [n2n](https://github.com/ntop/n2n)
  * 或者您可以直接从 [这里](https://github.com/lucktu/n2n) 获取由 lucktu 编译的预构建二进制文件。
  * 某些 Linux 发行版可能包含 n2n，但请确保您使用的是相同版本。由于源代码是 v3，以下步骤也将使用 v3 作为示例。
* 如果您使用的是 Windows PC，则需要 [TAP-Windows 驱动程序](https://community.openvpn.net/openvpn/wiki/GettingTapWindows) 或 [OpenVPN](https://openvpn.net/community/)（包含 TAP-Windows）
* [hin2n](https://github.com/switch-iot/hin2n) APK
* 具有公共 IP 并允许公共端口的服务器
* SideQuest 或其他将 hin2n APK 安装到您的头显上的方法

### 安装

我们将使用 n2n v3，并将 _supernode_ 的端口设置为 `1234` 作为示例。您可以将 `1234` 更改为任何端口，但低于 `1024` 的端口需要 root 权限。

* 在服务器的防火墙上打开端口 `1234`（通常是 `iptables`，如果您不知道如何操作，请咨询 Google）。
* 将 _supernode_ 二进制文件上传到您的服务器，运行 `./supernode -p 1234`。
* 如果您使用的是 Windows，请在您的 PC 上安装 TAP-Windows 驱动程序或 OpenVPN。
* 将 _edge_ 二进制文件上传到您的 PC，运行 `./edge -c [network-name] -k [secret-password] -a 192.168.100.1 -l [your-server-ip]:1234` 连接到 _supernode_，将 IP `192.168.100.1` 分配给 PC，并使用您提供的数据加密密码。
* 一旦您看到 `[OK] edge <<< ================ >>> supernode`，您的 PC 就完成了，否则您需要查看错误日志以了解问题所在。
* 在您的 Quest 上安装并打开 _hin2n_，单击右上角的加号按钮添加新配置并将 `192.168.100.2` 分配给您的 Quest：
  * N2N 版本：v3
  * Supernode：`[your-server-ip]:1234`
  * Community：`[network-name]`
  * Encrypt key：`[secret-password]`
  * IP address：`192.168.100.2`
  * Subnet mask：`255.255.255.0`
* 单击连接按钮下方的“当前设置”，选择我们刚刚创建的配置，然后单击连接按钮。如果系统要求您允许 hin2n 创建 VPN 连接，请允许。
* 一旦您看到 `[OK] edge <<< ================ >>> supernode`，您的 Quest 就完成了。
* 在您的头显上打开 ALVR，记录它显示的主机名。
* 在您的 PC 上打开 ALVR 控制面板，单击“手动添加设备”按钮，输入您刚刚记录的主机名，并将 IP 地址设置为刚刚分配给 Quest 的 `192.168.100.2`。
* 完成后，您就一切就绪了。

### 故障排除

* 确保您可以访问 supernode，您的 supernode 应该运行在具有公共 IP 的服务器上，并且您可以在您的 PC 上 ping 通它。
* 如果您的 Quest 无法连接到 ALVR 控制面板，请 ping 您在 hin2n 中分配给 Quest 的 IP。如果失败，请尝试重新执行设置步骤。
* 如果 edge 二进制文件或 hin2n 显示 IP 已被分配且未被 supernode 释放，您可以将 IP 地址设置为同一子网中的另一个 IP，例如 `192.168.100.123`，以重新分配新 IP 给设备。
* 如果您通过 WAN 玩游戏，您可能会看到更多的故障、更高的流延迟或 TCP 响应滞后。使用自适应比特率和 UDP 可能会改善您的体验。
