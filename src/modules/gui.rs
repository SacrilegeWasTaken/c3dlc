use std::io::ErrorKind;

use eframe::egui;
use egui::{text_edit, vec2, Align};

use super::colors::{convert_colors, extract_colors};
pub use super::files::SourceFiles;
use super::generate::generate;

pub fn gui() -> Result<(), eframe::Error>
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 500.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "convert3dlc",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
}

pub struct MyApp {
    width:          String,
    height:         String,
    boxsize:        String,
    elem_height:    f32,
    bitmode:        bool,
    filecontent:    String,
    console:        String,
    colorfile:      SourceFiles,
    number:         u16,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            boxsize:        String::new(),
            width:          String::new(),
            height:         String::new(),
            elem_height:    30f32,
            bitmode:        false,
            filecontent:    String::new(),
            console:        String::from("\t\tHi! You need to place your files into the source folder.\nSource code: SacrilegeWasTaken/3DLC_CustomCheckerReader"),
            colorfile:      SourceFiles::new(),
            number:         1u16,
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let input_width     = text_edit::TextEdit::singleline(&mut self.width);
                    let input_height    = text_edit::TextEdit::singleline(&mut self.height);
                    let input_boxsize   = text_edit::TextEdit::singleline(&mut self.boxsize);
                    ui.add_sized(
                        [ui.available_width() / 3f32, self.elem_height],
                        input_width
                            .hint_text("Width")
                            .horizontal_align(Align::Center)
                            .vertical_align(Align::Center),
                    );
                    ui.add_sized(
                        [ui.available_width() / 2f32, self.elem_height],
                        input_height
                            .hint_text("Height")
                            .horizontal_align(Align::Center)
                            .vertical_align(Align::Center),
                    );
                    ui.add_sized(
                        [ui.available_width(), self.elem_height],
                        input_boxsize
                            .hint_text("Box Size")
                            .horizontal_align(Align::Center)
                            .vertical_align(Align::Center),
                    );
                });
                self.fix_fields();

                ui.horizontal(|ui| {
                    let gen_filled  = egui::Button::new("Generate Filled");
                    let gen_unique  = egui::Button::new("Generate Unique");
                    let bitmode     = egui::Button::new("Toggle 32-bit mode");
                    if ui
                        .add_sized([ui.available_width() / 3f32, self.elem_height], gen_unique).clicked()
                    {
                        //generate();
                        match generate(
                            self.bitmode.clone(),
                            false,
                            self.width.clone(),
                            self.height.clone(),
                            self.boxsize.clone(),
                            convert_colors(extract_colors(&self.filecontent)).clone(),
                            self.number.clone(),
                        ) {
                            Ok(()) => {
                                self.console = format!(
                                    "Generated {} unique image",
                                    (|| -> &str {
                                        if self.bitmode == false {
                                            "16-bit"
                                        } else {
                                            "32-bit"
                                        }
                                    })()
                                );
                                self.number += 1;
                            }
                            Err(error) => {
                                if let Some(io_error) = error.downcast_ref::<std::io::Error>(){
                                    match io_error.kind() {
                                        ErrorKind::InvalidData => self.console = String::from(
                                            "\t\t\t\t\t\t\tGeneration error.\nMake sure that (width, height)%boxsize == 0"
                                        ),
                                        ErrorKind::WriteZero =>  self.console = String::from("\t\tGeneration error.\nZero boxsize is forbidden."),
                                        _ => self.console = String::from("Unknown generation error.")
                                    }
                                }
                            }
                        }
                    };
                    if ui
                        .add_sized([ui.available_width() / 2f32, self.elem_height], gen_filled)
                        .clicked()
                    {
                        //generate();
                        match generate(
                            self.bitmode.clone(),
                            true,
                            self.width.clone(),
                            self.height.clone(),
                            self.boxsize.clone(),
                            convert_colors(extract_colors(&self.filecontent)).clone(),
                            self.number.clone(),
                        ) {
                            Ok(()) => {
                                self.console = format!(
                                    "Generated {} filled image",
                                    (|| -> &str {
                                        if self.bitmode == false {
                                            "16-bit"
                                        } else {
                                            "32-bit"
                                        }
                                    })()
                                );
                                self.number += 1;
                            }
                            Err(error) => {
                                if let Some(io_error) = error.downcast_ref::<std::io::Error>(){
                                    match io_error.kind() {
                                        ErrorKind::InvalidData => self.console = String::from(
                                            "\t\t\t\t\t\t\tGeneration error.\nMake sure that (width, height)%boxsize == 0"
                                        ),
                                        ErrorKind::WriteZero => self.console = String::from("\t\tGeneration error.\nZero boxsize is forbidden."),
                                        _ => self.console = String::from("Unknown generation error.")
                                    }
                                }
                            }
                        }

                    };
                    if ui
                        .add_sized([ui.available_width(), self.elem_height], bitmode)
                        .clicked()
                    {
                        self.bitmode = !self.bitmode;
                        self.console = format!(
                            "Mode: {}",
                            match self.bitmode {
                                true => "32-bit",
                                false => "16-bit",
                            },
                        );
                    }
                });
                ui.horizontal(|ui| {
                    ui.spacing();
                    let openfile = egui::Button::new("Open file");
                    let prevfile = egui::Button::new("Prev file");
                    let nextfile = egui::Button::new("Next file");
                    let savefile = egui::Button::new("Save file");
                    if ui
                        .add_sized([ui.available_width() / 4f32, self.elem_height], openfile)
                        .clicked()
                    {
                        let content = self.colorfile.get_content();
                        match content.as_str() {
                            "" => self.console   = "Error with opening file.".to_string(),
                            _ => {
                                self.console     = format!("Opened: {}", self.colorfile.relativepath());
                                self.filecontent = content
                            }
                        }
                    }
                    if ui
                        .add_sized([ui.available_width() / 3f32, self.elem_height], prevfile)
                        .clicked()
                    {
                        self.colorfile.prevf();
                        self.filecontent    = self.colorfile.get_content();
                        self.console        = format!("Opened previous: {}", self.colorfile.relativepath());
                    }
                    if ui
                        .add_sized([ui.available_width() / 2f32, self.elem_height], nextfile)
                        .clicked()
                    {
                        self.colorfile.nextf();
                        self.filecontent = self.colorfile.get_content();
                        self.console     = format!("Opened next: {}", self.colorfile.relativepath());
                    }
                    if ui
                        .add_sized([ui.available_width(), self.elem_height], savefile)
                        .clicked()
                    {
                        self.colorfile.savef(&self.filecontent);
                        self.console = format!("Saved: {}", self.colorfile.relativepath());
                    }
                });
                ui.horizontal(|ui| {
                    ui.set_min_size(vec2(ui.available_width(), 325f32));
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let editor = egui::TextEdit::multiline(&mut self.filecontent);
                        ui.add_sized(
                            [ui.available_width(), 325f32],
                            editor.min_size(
                                vec2(ui.available_width(), ui.available_height()).round(),
                            ),
                        );
                    });
                });
                ui.horizontal(|ui| {
                    let console = egui::Label::new(&self.console);
                    ui.add_sized([ui.available_width(), self.elem_height * 2f32], console);
                })
                .inner;
            });
        });
    }
}
impl MyApp {
    fn fix_fields(&mut self) {
        self.boxsize    = self.boxsize.chars().filter(|&c| c.is_digit(10)).collect();
        self.height     = self.height.chars().filter(|&c| c.is_digit(10)).collect();
        self.width      = self.width.chars().filter(|&c| c.is_digit(10)).collect();
    }
}
