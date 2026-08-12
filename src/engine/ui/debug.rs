pub fn debug(raw_input: &egui::RawInput, egui_ctx: &egui::Context) -> egui::FullOutput{
    egui_ctx.run_ui(raw_input.clone(), |ctx| {
                egui::Window::new("Debug").show(ctx, |ui| {
                    ui.label("Hello from egui");
                    let resp = ui.button("Click me");
                    if resp.contains_pointer() && ctx.input(|i| i.pointer.press_origin()).is_some() {
                        println!("clicked");
                    }
                });
            })
}
