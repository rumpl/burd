pub struct BurdGui {
    html: Vec<char>,
}

impl BurdGui {
    pub fn new(cc: &eframe::CreationContext<'_>, text: String) -> Self {
        let html = parse_html(&text);
        println!("{}", html.clone().into_iter().collect::<String>());
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "cn".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "/usr/share/fonts/adobe-source-han-sans/SourceHanSansCN-Regular.otf"
            )),
        );
        fonts
            .families
            .insert(egui::FontFamily::Proportional, vec!["cn".to_owned()]);
        cc.egui_ctx.set_fonts(fonts);

        Self { html }
    }
}

fn parse_html(html: &str) -> Vec<char> {
    let mut in_tag = false;
    let mut result = vec![];
    for c in html.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            if c == ' ' && result[result.len() - 1] == ' ' {
                continue;
            }
            result.push(c);
        }
    }
    result
}

impl eframe::App for BurdGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(self.html.clone().into_iter().collect::<String>());
            });
        });
    }
}
