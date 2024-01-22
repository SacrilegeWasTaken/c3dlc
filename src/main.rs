mod modules;
fn main() -> Result<(), eframe::Error> {
    modules::files::makeall();
    modules::gui::gui()
}
