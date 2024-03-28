use std::fs::{self, File};

mod editor;
mod editor_config;
mod screen;
mod types;

use env_logger::{Builder, Target};

fn main() -> anyhow::Result<()> {
    let file = File::create("output.log").expect("Failed to create file");
    let mut builder = Builder::new();
    builder.target(Target::Pipe(Box::new(file)));
    builder.filter_level(log::LevelFilter::Debug);
    builder.init();
    log::info!("Starting editor");

    let mut editor = editor::Editor::new()?;
    let schema = editor_config::EditorConfig::generate_schema();
    fs::write("schema.json", schema)?;
    let config = editor_config::EditorConfig::load("config.json")?;
    editor.run(config)?;
    Ok(())
}
