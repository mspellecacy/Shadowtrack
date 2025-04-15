mod app;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Shadowtrack",
        options,
        Box::new(|_cc| Ok(Box::new(app::ShadowtrackApp::default()))),
    )
    .expect("Failed to start application");
}

#[cfg(test)]
mod tests;
