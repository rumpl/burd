use std::error::Error;

use gui::BurdGui;

mod gui;

async fn get(url: &str) -> Result<String, reqwest::Error> {
    let result = reqwest::get(url).await?;
    result.text().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let text = get("https://browser.engineering/examples/example1-simple.html").await?;
    let text = get("https://browser.engineering/examples/xiyouji.html").await?;

    let options = eframe::NativeOptions::default();
    _ = eframe::run_native(
        "Burd",
        options,
        Box::new(|cc| Box::<BurdGui>::new(BurdGui::new(cc, text))),
    );

    Ok(())
}
