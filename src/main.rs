mod editor;
mod llm;
mod app;

fn main() -> std::io::Result<()> {
    app::run_app()
}