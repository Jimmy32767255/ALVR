This document details some technologies used by ALVR.

如果您对本文档中的内容有任何疑问，可以联系@zarik5，最好是在Discord上。

本文档最后更新于2023年6月27日，并参考了master分支。

## 目录

* 架构
  * 打包的应用程序
  * 编程语言
  * 源代码组织
* 日志和错误管理
  * 事件系统
* 会话和设置
  * 代码和UI的程序化生成
* 控制面板
  * 用户界面
  * 驱动通信
  * 驱动生命周期
* 流媒体管道：概述
* 客户端-驱动通信
  * 发现
  * 流媒体
* SteamVR驱动
* 客户端和驱动合成器
  * 注视点渲染
  * 颜色校正
* 视频转码
* 音频
* 追踪和显示时序
* 其他流
*## 即将推出

ALVR正在不断发展，未来计划推出许多令人兴奋的功能。其中一些包括：

* **相位同步**：此功能旨在通过将视频流的相位与显示刷新率同步来进一步降低延迟。这将有助于消除抖动并提供更流畅的VR体验。
* **分片编码**：此功能旨在通过将视频帧分成多个切片并独立发送来提高性能。这将允许并行处理并减少数据包丢失的影响。
* **注视点编码**：此功能旨在通过将注视点渲染技术应用于视频编码过程来进一步优化视频质量并减少带宽。这将允许在注视点区域获得更高的质量，而在外围区域获得较低的质量，而不会牺牲整体视觉保真度。

这些只是ALVR计划的众多功能中的一小部分。开发团队致力于不断改进ALVR体验，并提供最佳的VR流媒体解决方案。

## Architecture

### 打包应用

ALVR由两个应用程序组成：串流器和客户端。串流器可以安装在Windows和Linux上，而客户端则安装在Android VR头显上。客户端通过TCP或UDP套接字与驱动程序通信。

客户端是一个统一的APK，名为`alvr_client_android.apk`。它由OpenXR提供支持，兼容Quest头显、Pico头显和HTC Focus 3以及XR Elite。

串流器由两部分组成：控制面板和驱动程序（也称为服务器）。驱动程序由SteamVR动态加载。以下是Windows上的文件结构：

* `bin/win64/`
  * `driver_alvr_server.dll`：主二进制文件，负责客户端发现和串流。由SteamVR加载。
  * `driver_alvr_server.pdb`：调试符号
  * `openvr_api.dll`：用于更新Chaperone的OpenVR SDK。
  * `vcruntime140_1.dll`：驱动程序中C++代码使用的Windows SDK。
* `ALVR Dashboad.exe`：用于更改设置、管理客户端、监控统计数据和执行安装操作的控制面板二进制文件。它可以启动SteamVR。
* `driver.vrdrivermanifest`：驱动程序使用的辅助配置文件。

运行时，还会创建其他一些文件：

* `session.json`：包含ALVR使用的统一配置数据，例如设置和客户端记录。
* `session_log.txt`：主日志文件。每行都是一个json结构，表示驱动程序生成的事件。每次客户端连接时都会清除此文件。
* `crash_log.txt`：辅助日志文件。与`session_log.txt`相同，但只保存错误日志，并且不会被清除。

### 编程语言

ALVR使用多种语言编写：Rust、C、C++、HLSL、GLSL。代码库中主要使用的语言是Rust，用于控制面板、网络、视频解码和音频代码。C和C++用于图形、视频编码和SteamVR集成。HLSL用于Windows驱动程序上的图形着色器，GLSL用于Linux驱动程序和客户端。未来，更多C/C++代码将重写为Rust，HLSL代码将迁移到GLSL或WGSL。

Rust是一种专注于内存安全和易用性的系统编程语言。它的性能与C++相当，但Rust代码受运行时错误影响的可能性较小。ALVR使用的主要Rust特性是枚举（enums），它对应于C++中的带标签联合（tagged unions）。Rust的枚举是一种数据类型，可以存储不同类型的数据，但一次只能访问一种类型。例如，`Result`类型可以包含`Ok`值或`Err`值，但不能同时包含两者。结合模式匹配，这是Rust应用程序中错误管理的基础。

### 源代码组织

ALVR代码托管在一个monorepo中。以下是git树的概述：

* `.github/`：包含GitHub CI使用的脚本。
* `alvr/`：每个子文件夹都是一个Rust crate（“crate”表示代码库或可执行文件）。
  * `audio/`：托管客户端和驱动程序共享的音频相关代码的实用crate。
  * `client_core/`：客户端的平台无关代码。它用作`alvr_client_openxr`的Rust库，也可以编译为带有.h头文件的C ABI共享库，以便与其他项目集成。
  * `client_mock/`：作为`alvr_client_core`的薄包装实现的客户端模拟。
  * `client_openxr/`：使用OpenXR实现的客户端，编译为APK二进制文件。
  * `common/`：其他crate共享的一些通用代码。它包含版本控制、日志记录、结构体原语和OpenXR路径的代码。
  * `dashboard/`：控制面板应用程序。
  * `events/`：托管与事件相关代码的实用crate。
  * `filesystem/`：托管Windows和Linux之间文件系统抽象代码的实用crate。
  * `packets/`：包含客户端、驱动程序和控制面板之间通信的数据包定义的实用crate。
  * `server/`：由SteamVR加载的驱动程序共享库。
  * `server_io/`：控制面板和驱动程序共享的通用功能，用于与主机系统交互。这使得控制面板和驱动程序可以独立工作。
  * `session/`：与会话文件和数据管理相关的实用crate。
  * `sockets/`：客户端和驱动程序共享的实用crate，包含套接字和协议实现。
  * `vrcompositor_wrapper/`：用于Linux的小脚本，用于SteamVR正确加载ALVR Vulkan层。
  * `vulkan_layer/`：用于Linux的Vulkan WSI层，用于解决OpenVR API在Linux上的限制。这主要是修补工作，希望将来能移除。
  * `xtask/`：托管各种脚本的实用CLI，用于环境设置、构建和打包ALVR。应使用`cargo xtask`调用。
* `resources/`：README的资源。
* `wiki/`：包含Github ALVR wiki的源代码。提交后更改会同步到实际的wiki。
* `about.toml`: Controls what dependency licenses are allowed in the codebase, and helps with generating the licenses file in the packaged ALVR streamer.
* `Cargo.lock`: Contains versioning information about Rust dependencies used by ALVR.
* `Cargo.toml`: Defines the list of Rust crates contained in the repository, and hosts some other workspace-level Rust configuration.

## 日志和错误管理

日志记录分为接口和实现两部分。接口在`alvr/common/src/logging.rs`中定义，实现则在每个二进制crate中定义为`logging_backend.rs`。

ALVR日志系统基于[log](https://crates.io/crates/log) crate。`log`本身已经非常强大，因为它的宏可以收集消息、调用文件和行号。

ALVR定义了一些结构体、宏和函数来简化错误管理。用于错误管理的基本类型是`StrResult<T>`，它是`Result<T, String>`的别名。有关Rust的Result类型的更多信息，请参阅[此处](https://doc.rust-lang.org/std/result/)。

ALVR中有许多日志记录方式，每种方式适用于不同的用例。要使用它们，您应该在Rust源文件的顶部添加`use alvr_common::prelude::*`。

* `error!()`、`warn!()`、`info!()`、`debug!()`（从`log` crate重新导出的宏）。日志根据日志后端进行处理。
* `show_e()`和`show_w()`用于记录字符串消息，并额外显示一个弹出窗口。
* `show_err()`、`show_warn()`的工作方式与`show_e()`和`show_w()`类似，但它们接受`Result<>`，并且仅在结果为`Err()`时才记录。
* `fmt_e!()`向消息添加跟踪信息并生成一个`Err()`，可以返回。
* `err!()`和`enone!()`分别与`.map_err()`和`.ok_or_else()`一起使用，将`Result`或`Option`映射到`StrResult`，并添加跟踪信息。
* 其他一些名称相似且功能相似的函数和宏。

### 事件系统

事件是驱动程序内部使用并发送到控制面板实例的消息。事件通过`send_event()`生成，并基于日志系统实现。

这是`Event`的JSON格式布局：

```json
{
  "timestamp": "<timestamp>",
  "event_type": {
    "id": "<EventType>",
    "content": { <depends on id> }
  }
}
```

日志是一种特殊类型的事件：

```json
{
  "timestamp": "<timestamp>",
  "event_type": {
    "id": "Log",
    "content": {
      "severity": "Error or Warn or Info or Debug",
      "content": "<the message>"
    }
  }
}
```

驱动程序将事件以JSON格式记录到`session.json`中，每行一个。

目前它的使用是有限的，但最终它将取代当前的日志系统，日志将建立在事件系统之上。目标是创建一个统一的星形网络，其中每个客户端和控制面板实例将事件发送到服务器，服务器将事件广播到所有其他客户端和控制面板实例。这还应该统一服务器与客户端和控制面板通信的方式，使控制面板成为另一个客户端。

## 会话和设置

ALVR使用统一的配置文件，即`session.json`。它在ALVR首次启动时生成。此文件包含以下顶级字段：

* `"server_version"`：串流器的当前版本。它有助于版本升级。
* `"drivers_backup"`：SteamVR驱动程序路径的临时存储。由控制面板使用。
* `"openvr_config"`：包含已检查差异的设置列表。它由驱动程序内部的C++代码使用。
* `"client_connections"`：包含与已知客户端对应的条目。
* `"session_settings"`：所有ALVR设置，以树形结构排列。

### 代码和UI的程序化生成

ALVR以树状结构布局设置，以便代码本身可以高效地利用。设置可以包含变体（在`session.json`中以PascalCase指定），它们表示互斥选项。

ALVR使用`settings-schema` crate中的`SettingsSchema`宏来生成辅助代码，即设置的模式和“默认表示”。这是一个专门为ALVR创建的crate，但也可以用于其他项目。

模式由嵌套的`SchemaNode`组成，其中包含元数据。一些元数据直接在结构体和枚举的内联属性中指定。

“默认表示”（类型名称通过将结构体/枚举名称与`Default`连接生成）是能够以不丢失未选择变体信息的方式保存设置的结构体；枚举被转换为结构体，包含值的变体被转换为字段。这样做的主要目标是满足用户在更改某些选项时不会丢失嵌套配置的期望。默认表示正是保存在`session.json`中`"session_settings"`内的内容。

有关各种模式节点类型的信息可以在[此处](https://github.com/zarik5/settings-schema-rs)找到。

控制面板利用模式元数据和默认表示来生成设置UI。最终结果是设置UI布局与代码内部使用的结构体紧密匹配，这有助于理解代码的内部工作原理。

升级ALVR时，会话布局可能会略有不同，通常会添加/删除/移动/重命名一些设置。ALVR能够通过外推过程来处理这个问题：它从默认会话开始，并在设置模式的帮助下替换从旧会话文件中获取的值。

## 控制面板

控制面板是与ALVR交互的主要方式。功能按选项卡组织。

### 用户界面

以下是主要组件：

TODO: 添加截图

* 侧边栏：用于选择主内容页面的选项卡。
* 设备选项卡：用于信任客户端或手动添加指定IP的客户端。
* 统计信息选项卡：显示延迟和FPS图表以及摘要页面。
* 设置选项卡：设置页面分为“预设”和“所有设置”。“所有设置”是根据模式程序化生成的。“预设”是修改其他设置的控件。
* 安装选项卡：安装实用程序：设置防火墙规则、注册驱动程序、启动设置向导。
* 日志选项卡：在表格中显示日志和事件。
* 调试选项卡：调试操作。
* 关于选项卡：有关ALVR的信息。
* 侧边栏底部按钮：根据驱动程序连接状态，可以是“启动SteamVR”或“重启SteamVR”。
* 通知栏：以非侵入性方式显示日志。

### 驱动通信

控制面板与驱动程序通信以更新其信息并保存配置。这是通过HTTP API完成的，基本URL为`http://localhost:8082`。以下是端点：

* `/api/dashboard-request`：这是控制面板用于向服务器发送消息和数据的主要URL。正文包含请求的特定类型和正文。
* `/api/events`：此端点升级为websocket，用于侦听来自驱动程序的事件。
* `/api/ping`：当驱动程序处于活动状态时返回代码200。

The dashboard retains some functionality when the driver is not launched. It can manage settings, clients and perform installation actions, but clients cannot be discovered. Once The driver is launched all these actions are performed by the server, requested with the HTTP API. This mechanism ensures that there are no data races.

### 驱动生命周期

ALVR驱动程序由SteamVR加载。SteamVR加载`steamvr.vrsettings`中`activeDrivers`字段指定的所有驱动程序。ALVR安装程序将ALVR驱动程序添加到此列表中。当SteamVR启动时，它会加载所有驱动程序并调用`HmdDriverFactory`函数，该函数返回`IVRServerDriver`的一个实例。

ALVR驱动程序在`alvr/server/src/lib.rs`中实现。`IVRServerDriver`接口由`AlvrDriver`结构体实现。`AlvrDriver`结构体负责初始化驱动程序、处理来自SteamVR的事件以及向客户端发送数据。

当SteamVR关闭时，它会卸载所有驱动程序并调用`IVRServerDriver`的`Cleanup`函数。`Cleanup`函数负责释放驱动程序使用的所有资源。

## 流媒体管道：概述

ALVR是一个流媒体应用程序，其主要目的是将视频和音频从PC发送到头显。这是通过一系列步骤的管道完成的，每个步骤都有自己的职责。管道如下：

1. **追踪**：头显将其姿态（位置和旋转）发送到PC。
2. **渲染**：SteamVR根据头显的姿态渲染场景。
3. **编码**：渲染的视频被编码成压缩视频流。
4. **传输**：压缩视频流通过网络发送到头显。
5. **解码**：头显解码压缩视频流。
6. **显示**：解码后的视频在头显上显示。

这是一个简化的概述。实际的管道更复杂，涉及许多其他步骤，例如音频流、输入流和错误处理。以下部分将更详细地描述每个步骤。

## 客户端-驱动通信

ALVR使用自定义协议进行客户端-驱动通信。ALVR支持UDP和TCP传输。虽然USB连接不是一流功能，但也受支持；您可以在[此处](https://github.com/Jimmy32767255/ALVR-CN/wiki/ALVR-wired-setup-(ALVR-over-USB))阅读更多关于它的信息。

### 发现

通常，建立连接的第一步是发现。当服务器发现客户端时，它会在“设备”选项卡的“新设备”部分显示该客户端。用户随后可以信任该客户端，然后建立连接。

ALVR使用9943端口的UDP套接字进行发现。客户端广播一个数据包并等待驱动程序响应。是客户端广播，然后驱动程序请求连接：这是因为两个对等方责任平衡。客户端成为PC的门户，PC可能包含敏感数据。因此，服务器在启动连接之前必须信任客户端。

这是发现数据包的布局：

|      前缀       | 协议ID | 主机名 |
| :---------------: | :---------: | :------: |
| "ALVR" + 0x0 x 12 |   8 字节   | 32 字节 |

* 前缀用于过滤数据包，确保数据包确实由ALVR客户端发送。
* 协议ID是根据客户端的语义版本计算出的唯一版本标识符。如果客户端版本与串流器“语义兼容”，则协议ID将匹配。
* 主机名：主机名是客户端的唯一标识符。当客户端首次启动时，会选择一个主机名，并在后续启动中保持不变。当应用程序升级或降级时，它会被重置。

数据包的格式可能在主要版本之间发生变化，但前缀必须保持不变，协议ID必须是8字节。

### 流媒体

ALVR使用两个套接字进行流媒体传输：控制套接字和流套接字。目前这些都是用异步代码实现的；有一个计划将其改回同步代码。

控制套接字使用TCP传输；它用于在客户端和服务器之间交换小消息，ALVR需要TCP来确保可靠性。

流套接字可以使用UDP或TCP；它用于发送大型数据包和/或不需要可靠性的数据包，ALVR对数据包丢失和数据包重排序具有鲁棒性。

由于ALVR使用多个抽象层来操作数据（bincode、tokio Length Delimited Coding），因此网络上使用的特定数据包格式没有明确定义。此外，数据包被分解成碎片，以确保在使用UDP时它们可以支持MTU。

由于流式传输的数据量很大，因此驱动程序端和客户端的套接字缓冲区大小都增加了。

## SteamVR驱动

驱动程序是负责大部分串流器功能的组件。它作为SteamVR加载的共享库实现。它实现了[OpenVR API](https://github.com/ValveSoftware/openvr)以与SteamVR接口。

ALVR使用OpenVR API，通过`vr::VRServerDriverHost()->TrackedDevicePoseUpdated()`将追踪和按钮数据推送到SteamVR。然后SteamVR返回一个带有用于渲染的姿态的渲染游戏帧。在Windows上，通过实现`IVRDriverDirectModeComponent`接口来检索帧：SteamVR调用`IVRDriverDirectModeComponent::Present()`。在Linux上，此API不起作用，因此ALVR使用WSI Vulkan层来拦截vrcompositor进行的显示驱动程序调用。与帧关联的姿态是通过libunwind从vrcompositor执行堆栈中获取的。

## 客户端和驱动程序合成器

ALVR在客户端和驱动程序端都使用合成器来优化渲染过程。驱动程序合成器负责应用注视点渲染和色彩校正，而客户端合成器负责显示解码后的视频。

### 注视点渲染

注视点渲染是一种技术，它在用户视野的周边降低渲染分辨率，同时在中央凹（视野的中心部分）保持高分辨率。这减少了GPU的计算负载，并在图像质量没有明显损失的情况下提高了性能。

ALVR在视频编码之前，在驱动程序端实现注视点渲染。这使得编码器可以使用较低分辨率的图像，从而进一步降低比特率并提高性能。ALVR使用的注视点渲染算法基于Oculus AADT算法。

### 色彩校正

色彩校正是一种调整视频流颜色以匹配头显显示器特性的技术。这确保了头显上显示的颜色准确且鲜艳。

ALVR在注视点渲染之后、视频编码之前，在驱动程序端实现色彩校正。这使得色彩校正可以应用于全分辨率图像，并确保在编码器压缩之前颜色是准确的。

## 视频转码

ALVR使用硬件加速视频编码器来压缩视频流。支持的编码器有H264、HEVC和AV1。编码器由用户在设置中选择，驱动程序会自动选择最佳可用硬件编码器。

ALVR使用以下库进行视频编码：

* **Nvidia**：NVENC（Nvidia视频编码器）用于Nvidia GPU。
* **AMD**：AMF（高级媒体框架）用于AMD GPU。
* **Intel**：Media SDK用于Intel GPU。
* **软件**：x264用于软件编码，速度较慢但兼容性更好。

ALVR还支持IDR（即时解码刷新）帧，这些帧是完整的帧，可以独立于以前的帧进行解码。IDR帧用于从数据包丢失中恢复，并允许客户端在视频流中查找。ALVR每隔几秒发送一次IDR帧，或者在客户端请求时发送。

## 音频

ALVR将音频从PC发送到客户端。音频从PC上的默认音频输出设备捕获，编码后发送到客户端。客户端随后通过其默认音频输出设备播放音频。

ALVR使用以下库进行音频编码：

* **Opus**：一种高度通用的音频编解码器，非常适合通过互联网进行交互式语音和音乐传输。它因其低延迟和高质量而被使用。

ALVR还处理音频数据包丢失。如果音频数据包丢失，ALVR可以尝试通过插值缺失的音频数据来隐藏丢失，或者直接丢弃音频。使用的方法取决于数据包丢失的严重程度和配置。

ALVR使用自定义音频管道来确保低延迟和高质量音频。音频以PCM波形捕获，然后编码为Opus，并通过UDP发送。在客户端，Opus音频被解码并播放。

为了避免音频故障，ALVR使用抖动缓冲区来平滑网络延迟的变化。抖动缓冲区临时存储传入的音频数据包，并以稳定的速率释放它们，即使网络条件波动也能确保连续的音频流。

## 追踪和显示时序

对于VR应用程序，处理头部和控制器追踪很棘手，对于VR流媒体应用程序更是如此。

在正常的原生VR应用程序中，追踪在渲染周期的开始时进行轮询，它用于从特定视角渲染眼睛视图并渲染控制器或手部模型。当游戏完成帧渲染后，它将其提交给VR运行时，VR运行时将在屏幕上显示它。从追踪轮询到帧在屏幕上显示之间，可能已经过去1个或更多帧持续时间（例如，在72fps时，帧持续时间为13毫秒）。我们的眼睛对延迟非常敏感，尤其是对于方向，因此VR运行时实现了图像重投影（Oculus称之为异步时间扭曲）。重投影通过在3D中旋转渲染帧来工作，以补偿在渲染周期开始时轮询的追踪姿态与图像应推送到显示器时垂直同步时的头显实际姿态之间的差异。为了能够正确旋转图像，运行时还需要知道用于轮询追踪的时间戳，这可以是轮询时间，或者更好的是，垂直同步的预测时间。如果使用未来的时间进行追踪轮询，则轮询的追踪将被外推。

对于VR流媒体应用程序，管道类似，只是追踪是在更远的未来时间点进行轮询，以补偿整个转码管道，并且决定未来预测多少并不简单。ALVR通过读取追踪轮询时间与使用相同追踪渲染的帧提交时间之间经过的时间来计算预测偏移。这些间隔样本被平均，然后用于未来的追踪轮询。（要计算正确的总延迟，您还需要添加VR运行时合成器延迟，这在控制面板延迟图中显示为“客户端垂直同步”）。

在串流器端，ALVR需要解决OpenVR API的一个限制。SteamVR返回带有其姿态的帧，但随后ALVR负责将该姿态与之前提交的姿态之一进行匹配，并重新匹配其时间戳。

## 其他流

除了视频和音频，ALVR还在客户端和驱动程序之间传输其他数据。这些包括：

* **按钮**：客户端将控制器上的按钮按下事件发送到驱动程序。驱动程序随后将其转换为OpenVR输入事件。
* **触觉**：驱动程序将触觉反馈命令发送到客户端。客户端随后相应地振动控制器。

这些流通常带宽较低，并通过控制套接字（可靠的TCP连接）发送。这确保了即使存在一些网络不稳定，按钮按下和触觉反馈也不会丢失。

## Upcoming

### 相位同步

相位同步并非单一算法，而是许多具有相似目标的算法，旨在减少渲染/串流管道中的延迟或抖动。“相位同步”一词源于Oculus，它描述了其在OpenXR运行时中通过尽可能晚地启动渲染周期来减少延迟的算法，以减少垂直同步前的等待时间。

通常，相位同步算法由两部分组成：一个用于保存数据资源或指针的队列，以及一个用于预测事件时间的统计模型。统计模型接收持续时间或其他类型的时序样本作为输入，并输出对重复事件的精确时间预测。统计模型可以很简单，只针对平均控制事件，也可以更复杂，旨在满足截止时间；后一种情况需要考虑时序样本的方差。与Oculus的实现不同，这些统计模型可以高度可配置，以调整目标均值或目标方差。

计划实现几种相位同步算法：帧提交时序（通过改变驱动渲染周期的相位来减少客户端的帧排队）、SteamVR追踪提交时序（确保SteamVR使用我们想要的精确追踪样本）和追踪轮询时序（减少服务器端的排队）。

## 分片编码

分片编码是Oculus展示的另一种算法，旨在通过并行化工作来减少延迟。在简单的串流管道中，帧是顺序处理的：渲染、编码、传输、解码。已经存在一定程度的并行性，因为渲染、编码、传输和解码可以同时发生。分片编码可以通过将帧分割成“切片”来帮助减少编码和解码时间。这使得硬件编码器/解码器能够更有效地利用，甚至可以并行使用硬件和软件编解码器。需要注意的是，网络延迟无法优化。在网络限制下，分片编码可以减少编码器/传输和传输/解码之间的等待时间，因为每个编码的切片都可以立即传输，而无需等待帧的其余部分被编码（解码端也适用类似的原理）。
