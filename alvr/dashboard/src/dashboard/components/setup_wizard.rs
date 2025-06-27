use alvr_packets::{FirewallRulesAction, ServerRequest};
use eframe::{
    egui::{Button, Label, Layout, RichText, Ui},
    emath::Align,
};

pub enum SetupWizardRequest {
    ServerRequest(ServerRequest),
    Close { finished: bool },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Welcome = 0,
    ResetSettings = 1,
    HardwareRequirements = 2,
    SoftwareRequirements = 3,
    Firewall = 4,
    Recommendations = 5,
    Finished = 6,
}

fn index_to_page(index: usize) -> Page {
    match index {
        0 => Page::Welcome,
        1 => Page::ResetSettings,
        2 => Page::HardwareRequirements,
        3 => Page::SoftwareRequirements,
        4 => Page::Firewall,
        5 => Page::Recommendations,
        6 => Page::Finished,
        _ => panic!("Invalid page index"),
    }
}

fn page_content(
    ui: &mut Ui,
    subtitle: &str,
    paragraph: &str,
    interactible_content: impl FnMut(&mut Ui),
) {
    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
        ui.add_space(60.0);
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.add_space(60.0);
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                ui.add_space(15.0);
                ui.heading(RichText::new(subtitle).size(20.0));
                ui.add(Label::new(RichText::new(paragraph).size(14.0)).wrap());
                ui.add_space(30.0);
                ui.vertical_centered(interactible_content);
            });
        })
    });
}

pub struct SetupWizard {
    page: Page,
}

impl SetupWizard {
    pub fn new() -> Self {
        Self {
            page: Page::Welcome,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<SetupWizardRequest> {
        let mut request = None;

        ui.horizontal(|ui| {
            ui.add_space(60.0);
            ui.vertical(|ui| {
                ui.add_space(30.0);
                ui.heading(RichText::new("欢迎使用 ALVR").size(30.0));
                ui.add_space(5.0);
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(15.0);
                if ui.button("❌").clicked() {
                    request = Some(SetupWizardRequest::Close { finished: false });
                }
            })
        });
        ui.separator();
        match &self.page {
            Page::Welcome => page_content(
                ui,
                "此设置向导将帮助您设置 ALVR。",
                "",
                |_| (),
            ),
            Page::ResetSettings => page_content(
                ui,
                "重置设置",
                "建议您每次更新 ALVR 时都重置设置。",
                |ui| {
                    if ui.button("重置设置").clicked() {
                        request = Some(SetupWizardRequest::ServerRequest(
                            ServerRequest::UpdateSession(Box::default()),
                        ));
                    }
                },
            ),
            Page::HardwareRequirements => page_content(
                ui,
                "硬件要求",
                r"ALVR 需要独立且较新的显卡。低端英特尔集成显卡可能无法正常工作。
请确保您至少有一个输出音频设备。",
                |_| (),
            ),
            Page::SoftwareRequirements => page_content(
                ui,
                "软件要求",
                if cfg!(windows) {
                    r"要在 Windows 上传输头戴式显示器麦克风，您需要安装 Virtual Audio Cable、VB-Cable、Voicemeeter"
                } else if cfg!(target_os = "linux") {
                    r"您需要 PipeWire (0.3.49+ 版本) 音频系统才能传输音频和使用麦克风。"
                } else {
                    r"不支持的操作系统"
                },
                #[allow(unused_variables)]
                |ui| {
                    #[cfg(windows)]
                    if ui.button("下载 Virtual Audio Cable (精简版)").clicked() {
                        ui.ctx().open_url(eframe::egui::OpenUrl::same_tab(
                            "https://software.muzychenko.net/freeware/vac470lite.zip",
                        ));
                    }
                },
            ),
            Page::Firewall => page_content(
                ui,
                "防火墙",
                r"为了与头戴式显示器通信，需要设置一些防火墙规则。
这需要管理员权限！",
                |ui| {
                    if ui.button("添加防火墙规则").clicked() {
                        request = Some(SetupWizardRequest::ServerRequest(
                            ServerRequest::FirewallRules(FirewallRulesAction::Add),
                        ));
                    }
                },
            ),
            Page::Recommendations => page_content(
                ui,
                "建议",
                r"ALVR 支持多种类型的 PC 硬件和头戴式显示器，但并非所有设备都能在默认设置下正常工作。如果您的 ALVR 体验不佳，请尝试调整分辨率、比特率、编码器等不同设置。",
                |_| (),
            ),
            Page::Finished => page_content(
                ui,
                "完成",
                r#"您可以随时从左侧的“安装”选项卡重新启动此设置向导。"#,
                |_| (),
            ),
        };

        ui.with_layout(Layout::bottom_up(Align::RIGHT), |ui| {
            ui.add_space(30.0);
            ui.horizontal(|ui| {
                ui.add_space(15.0);
                if self.page == Page::Finished {
                    if ui.button("完成").clicked() {
                        request = Some(SetupWizardRequest::Close { finished: true });
                    }
                } else if ui.button("下一步").clicked() {
                    self.page = index_to_page(self.page as usize + 1);
                }
                if ui
                    .add_visible(self.page != Page::Welcome, Button::new("上一步"))
                    .clicked()
                {
                    self.page = index_to_page(self.page as usize - 1);
                }
            });
            ui.separator();
        });

        request
    }
}
