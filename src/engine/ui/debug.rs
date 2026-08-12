use std::default;

static mut CLICKED: bool = false;
pub fn debug(raw_input: &egui::RawInput, egui_ctx: &egui::Context, egui_events: &Vec<egui::Event>) -> egui::FullOutput{
    egui_ctx.run_ui(raw_input.clone(), |ctx| {
                egui::Window::new("Debug").show(ctx, |ui| {
                    ui.label("Hello from egui");
                    let resp = ui.button("Click me");
                    if resp.contains_pointer() && ctx.input(|i| i.pointer.press_origin()).is_some() && !unsafe { CLICKED } {
                        println!("clicked");
                        unsafe { CLICKED = true };
                    }
                    if ctx.input(|i| i.pointer.press_origin().is_none()) && unsafe { CLICKED } {
                        unsafe {CLICKED = false}
                    } 
                });
            })
}
