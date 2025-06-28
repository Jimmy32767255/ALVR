use alvr_common::{LogEntry, LogSeverity};
use alvr_gui_common::theme::{self, log_colors};
use alvr_session::Settings;
use eframe::{
    egui::{self, Frame, Label, Layout, RichText, TopBottomPanel},
    emath::Align,
    epaint::{Color32, Stroke},
};
use rand::seq::IndexedRandom;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use instant::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

const TIMEOUT: Duration = Duration::from_secs(5);
const NO_NOTIFICATIONS_MESSAGE: &str = "没有新通知";
const NOTIFICATION_TIPS: &[&str] = &[
    // The following tips are ordered roughtly in the order settings appear
    r#"如果您在更改某些设置后开始出现崩溃，请通过从“安装”选项卡重新运行“运行设置向导”并单击“重置设置”来重置 ALVR。"#,
    r#"某些设置默认是隐藏的。单击某些设置旁边的“展开”按钮以展开子菜单。"#,
    r#"强烈建议在 ALVR 中保持音频设置默认，并在任务栏托盘中修改默认音频设备。"#,
    r#"增加“最大缓冲”可能会减少卡顿，但会增加延迟。"#,
    r#"有时在某些 GPU 上需要在 h264 和 HEVC 编解码器之间切换，以修复崩溃或回退到软件编码。"#,
    r#"如果您使用的是 NVIDIA GPU，最好使用高比特率 H264；如果您使用的是 AMD GPU，HEVC 可能会看起来更好。"#,
    r#"如果您遇到“白雪”闪烁，请将分辨率降低到“低”并禁用“注视点编码”。"#,
    r#"增加“色彩校正”->“锐度”可能会提高感知图像质量。"#,
    r#"如果您在将外部控制器或追踪器同步到 ALVR 追踪空间时遇到问题，请在“额外 OpenVR 属性”中添加一个元素，然后设置自定义的“追踪系统名称”。"#,
    r#"要更改控制器的视觉外观，请设置“头戴式显示器”->“控制器”->“模拟模式”。"#,
    r#"ALVR 支持自定义按钮绑定！如果您需要帮助，请在官方 Discord 服务器上提问。"#,
    r#"ALVR 支持手部追踪手势（“预设”->“手部追踪交互”->“ALVR 绑定”）。请查看维基百科了解如何正确使用它们：https://github.com/Jimmy32767255/ALVR-CN/wiki/Hand-tracking-controller-bindings。"#,
    r#"如果手部追踪手势很烦人，您可以在“控制器”->“手势”中禁用它们。或者，您可以启用“手势”->“仅触摸”。"#,
    r#"您可以通过“控制器”->“预测”微调控制器的响应速度。"#,
    r#"如果视觉控制器/手部模型与物理控制器不匹配，您可以在“头戴式显示器”->“控制器”->“左控制器位置/旋转偏移”中调整偏移（影响两个控制器）。"#,
    r#"当使用外部追踪器或控制器时，您应该将“位置/旋转重新居中模式”都设置为“禁用”。"#,
    r#"您可以启用倾斜模式。将“位置重新居中模式”设置为“本地”，将“旋转重新居中模式”设置为“倾斜”。"#,
    r#"如果您经常遇到图像故障，您可以使用“避免视频故障”来换取卡顿帧。"#,
    r#"您可以使用“连接/断开脚本”在头戴式显示器连接/断开时运行自定义命令/程序。"#,
    r#"如果您想报告错误，要获取日志文件，请启用“额外”->“日志记录”->“日志到磁盘”。日志将在“session_log.txt”中。"#,
    r#"出于黑客目的，您可以启用“日志追踪”、“日志按钮按下”和“日志触觉”。您可以使用 ws://localhost:8082/api/events 的 websocket 获取数据"#,
    r#"如果您想报告错误并分享您的日志，您应该启用“额外”->“日志记录”->“首选回溯”。"#,
    r#"您可以通过切换“显示通知提示”来快速浏览此类提示"#,
    r#"如果您想将身体追踪器或其他 SteamVR 驱动程序与 ALVR 一起使用，请将“驱动程序启动操作”设置为“在关机时注销 ALVR”"#,
    r#"启用“额外”->“SteamVR 启动器”->“随仪表板打开和关闭 SteamVR”很方便"#,
    r#"如果您想分享视频录制以报告错误，您可以启用“额外”->“捕获”->“滚动视频文件”以限制上传文件的大小"#,
    // Miscellaneous
    r#"如果您的头戴式显示器未出现在设备列表中，它可能位于不同的子网中。尝试使用设备内部显示的 IP“手动添加设备”。"#,
];

pub struct NotificationBar {
    message: String,
    current_level: LogSeverity,
    receive_instant: Instant,
    min_notification_level: LogSeverity,
    tip_message: Option<String>,
    expanded: bool,
}

impl NotificationBar {
    pub fn new() -> Self {
        Self {
            message: NO_NOTIFICATIONS_MESSAGE.into(),
            current_level: LogSeverity::Debug,
            receive_instant: Instant::now(),
            min_notification_level: LogSeverity::Debug,
            tip_message: None,
            expanded: false,
        }
    }

    pub fn update_settings(&mut self, settings: &Settings) {
        self.min_notification_level = settings.extra.logging.notification_level;

        if settings.extra.logging.show_notification_tip {
            if self.tip_message.is_none() {
                self.tip_message = NOTIFICATION_TIPS
                    .choose(&mut rand::rng())
                    .map(|s| format!("Tip: {s}"));
            }
        } else {
            self.tip_message = None;
        }
    }

    pub fn push_notification(&mut self, event: LogEntry, from_dashboard: bool) {
        let now = Instant::now();
        let min_severity = if from_dashboard {
            if cfg!(debug_assertions) {
                LogSeverity::Debug
            } else {
                LogSeverity::Info
            }
        } else {
            self.min_notification_level
        };

        if event.severity >= min_severity
            && (now > self.receive_instant + TIMEOUT || event.severity >= self.current_level)
        {
            self.message = event.content;
            self.current_level = event.severity;
            self.receive_instant = now;
        }
    }

    pub fn ui(&mut self, context: &egui::Context) {
        let now = Instant::now();
        if now > self.receive_instant + TIMEOUT {
            self.message = self
                .tip_message
                .clone()
                .unwrap_or_else(|| NO_NOTIFICATIONS_MESSAGE.into());
            self.current_level = LogSeverity::Debug;
        }

        let (fg, bg) = match self.current_level {
            LogSeverity::Error => (Color32::BLACK, log_colors::ERROR_LIGHT),
            LogSeverity::Warning => (Color32::BLACK, log_colors::WARNING_LIGHT),
            LogSeverity::Info => (Color32::BLACK, log_colors::INFO_LIGHT),
            LogSeverity::Debug => (theme::FG, theme::LIGHTER_BG),
        };

        let mut bottom_bar = TopBottomPanel::bottom("bottom_panel").frame(
            Frame::default()
                .inner_margin(egui::vec2(10.0, 5.0))
                .fill(bg)
                .stroke(Stroke::new(1.0, theme::SEPARATOR_BG)),
        );
        let alignment = if !self.expanded {
            bottom_bar = bottom_bar.max_height(26.0);

            Align::TOP
        } else {
            Align::Center
        };

        bottom_bar.show(context, |ui| {
            ui.with_layout(Layout::right_to_left(alignment), |ui| {
                if !self.expanded {
                    if ui.small_button("展开").clicked() {
                        self.expanded = true;
                    }
                } else if ui.button("收起").clicked() {
                    self.expanded = false;
                }
                ui.with_layout(Layout::left_to_right(alignment), |ui| {
                    ui.add(Label::new(RichText::new(&self.message).color(fg).size(12.0)).wrap());
                })
            })
        });
    }
}
