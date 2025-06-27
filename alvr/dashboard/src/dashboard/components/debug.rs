use alvr_packets::ServerRequest;
use eframe::egui::Ui;

pub fn debug_tab_ui(ui: &mut Ui) -> Option<ServerRequest> {
    let mut request = None;

    ui.label(
        "使用以下按钮从 ALVR 录制不适合捕捉游戏画面。\n为此，请使用其他录制方式，例如通过头显或桌面 VR 输出。",
    );

    ui.columns(4, |ui| {
        if ui[0].button("捕获帧").clicked() {
            request = Some(ServerRequest::CaptureFrame);
        }

        if ui[1].button("插入 IDR").clicked() {
            request = Some(ServerRequest::InsertIdr);
        }

        if ui[2].button("开始录制").clicked() {
            request = Some(ServerRequest::StartRecording);
        }

        if ui[3].button("停止录制").clicked() {
            request = Some(ServerRequest::StopRecording);
        }
    });

    request
}
