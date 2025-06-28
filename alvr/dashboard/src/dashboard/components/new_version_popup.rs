use std::{path::PathBuf, process::Command};

use alvr_gui_common::ModalButton;
use alvr_packets::{PathValuePair, ServerRequest};
use eframe::egui::{self, Context, OpenUrl, Ui};

pub struct NewVersionPopup {
    version: String,
    message: String,
    launcher_path: Option<PathBuf>,
}

impl NewVersionPopup {
    pub fn new(version: String, message: String) -> Self {
        let mut launcher_path = None;

        let layout = crate::get_filesystem_layout();
        if let Some(path) = layout.launcher_exe() {
            if path.exists() {
                launcher_path = Some(path);
            }
        }

        Self {
            version,
            message,
            launcher_path,
        }
    }

    pub fn ui(&self, context: &Context, shutdown_alvr_cb: impl Fn()) -> Option<ServerRequest> {
        let no_remind_button =
            ModalButton::Custom("此版本不再提醒".to_string());

        let result = alvr_gui_common::modal(
            context,
            "ALVR 新版本可用",

            Some(|ui: &mut Ui| {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);

                    ui.vertical(|ui| {
                        ui.heading(format!("ALVR v{}", self.version));

                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 5.0;
                            ui.style_mut().spacing.button_padding = egui::vec2(10.0, 4.0);

                            ui.heading("您可以使用启动器下载此版本：");

                            if let Some(path) = &self.launcher_path {
                                if ui.button("打开启动器").clicked()
                                    && Command::new(path).spawn().is_ok()
                                {
                                    shutdown_alvr_cb();
                                }
                            } else if ui.button("下载启动器").clicked() {
                                let base_url =
                                    "https://github.com/Jimmy32767255/ALVR-CN";
                                let file = if cfg!(windows) {
                                    "alvr_launcher_windows.zip"
                                } else {
                                    "alvr_launcher_linux.tar.gz"
                                };

                                context.open_url(OpenUrl::new_tab(format!("{base_url}{file}")));
                            }
                        });

                        ui.add_space(10.0);

                        ui.label(&self.message);
                        ui.hyperlink_to(
                            "发布页面",
                            "https://github.com/Jimmy32767255/ALVR-CN",
                        );
                    });

                    ui.add_space(10.0);
                });
            }),
            &[no_remind_button.clone(), ModalButton::Close],
            Some(490.0),
        );

        if let Some(button) = result {
            if button == no_remind_button {
                return Some(ServerRequest::SetValues(vec![
                    PathValuePair {
                        path: alvr_packets::parse_path(
                            "session_settings.extra.new_version_popup.HideWhileVersion",
                        ),
                        value: serde_json::Value::String(self.version.clone()),
                    },
                    PathValuePair {
                        path: alvr_packets::parse_path(
                            "session_settings.extra.new_version_popup.variant",
                        ),
                        value: serde_json::Value::String("HideWhileVersion".to_string()),
                    },
                ]));
            }
        }

        None
    }
}
