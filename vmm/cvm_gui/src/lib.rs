use eframe::egui;
use cvm_controller;
use cvm_vm;
pub fn gui_main() {
    let linux_test = cvm_vm::generate_cfg(
        &cvm_vm::snix::Vm{
            kernel: "./bzImage".to_string(),
            kernel_params: vec!["test=value".to_string()],
            initrd: Some("./initrd.img".to_string()),
            disks: vec![
                cvm_vm::snix::Block{
                    path: "./rootfs".to_string(),
                    root: true,
                }
            ],
        }
    );

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(VmmGui::new(cc)))))
        .expect("Unable to create eframe native application");
}

#[derive(Default)]
struct VmmGui {
    socket: String,
    socket_controller: Option<cvm_controller::VmSocket>
}

impl VmmGui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for VmmGui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            if ui.text_edit_singleline(&mut self.socket).changed(){
                self.socket_controller = Some(cvm_controller::VmSocket::new()
                    .connect(&std::path::PathBuf::from(self.socket.clone()))
                    .unwrap_or(cvm_controller::VmSocket::new()));
            }
            if ui.button("Pause Server").clicked(){
                self.socket_controller.as_ref().unwrap()
                    .suspend_vm()
                    .expect("Unable to suspend VM");
            }
            if ui.button("Resume Server").clicked(){
                self.socket_controller.as_ref().unwrap()
                    .resume_vm()
                    .expect("Unable to suspend VM");
            }
            if ui.button("Stop Server").clicked(){
                self.socket_controller.as_ref().unwrap()
                    .stop_vm()
                    .expect("Unable to suspend VM");
            }
        });
    }
}