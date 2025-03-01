use eframe::egui;

struct Board {
    name: String,
    age: i32,
    grid: [[i32; 8]; 8]
}

impl Default for Board {
    fn default() -> Self {
        Self {
            name: "Smith".to_owned(),
            age: 30,
            grid: [ [1, 0, 1, 0, 1, 0, 1, 0], 
                    [0, 1, 0, 1, 0, 1, 0, 1], 
                    [1, 0, 1, 0, 1, 0, 1, 0], 
                    [0, 1, 0, 1, 0, 1, 0, 1],
                    [1, 0, 1, 0, 1, 0, 1, 0], 
                    [0, 1, 0, 1, 0, 1, 0, 1], 
                    [1, 0, 1, 0, 1, 0, 1, 0], 
                    [0, 1, 0, 1, 0, 1, 0, 1]
                ]
        }
    }
}

impl Board {
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        for r in 0..7 {
            for c in 0..7 {
                if self.grid[r][c] == 1 {
                    ui.horizontal(|ui| {
                        ui.allocate_space(egui::vec2(100.0, 100.0));

                        // TODO: Need to figure out position of each tile
            
                        ui.painter().rect_filled(
                            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::vec2(100.0, 100.0)),
                            0.0,
                            egui::Color32::from_rgb(255, 0,0),
                        );
                    });
                } else {
                    ui.horizontal(|ui| {
                        ui.allocate_space(egui::vec2(100.0, 100.0));
            
                        ui.painter().rect_filled(
                            egui::Rect::from_min_size(egui::Pos2::new(1.0, 1.0), egui::vec2(100.0, 100.0)),
                            0.0,
                            egui::Color32::from_rgb(184, 140,100),
                        );
                    });
                }
            }
        }

    }
}

impl eframe::App for Board {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My App");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            self.draw(ui);
            
            // ui.image(egui::include_image!(

            // ))
        });
    }
}

pub fn run() -> eframe::Result<()> {
    // env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My Egui App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            
            Ok(Box::<Board>::default())
        }),
    )
}
