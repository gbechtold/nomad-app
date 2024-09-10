use crate::editor::Editor;
use crate::editor::FileOps;
use crate::llm::LlamaModel;
use std::io::{self, Write};

pub fn run_app() -> io::Result<()> {
    let llm = LlamaModel::new();

    loop {
        println!("Welcome to Nomad!");
        println!("1. Create/Edit a note");
        println!("2. Load a note");
        println!("3. Exit");
        print!("Choose an option (1, 2, or 3): ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" | "2" => {
                let mut editor = Editor::new()?;
                
                if choice.trim() == "2" {
                    print!("Enter the filename to load: ");
                    io::stdout().flush()?;
                    let mut filename = String::new();
                    io::stdin().read_line(&mut filename)?;
                    editor.load_file(filename.trim())?;
                }

                println!("Editor shortcuts:");
                println!("Ctrl-Q: Quit, Ctrl-S: Save, Ctrl-O: Open/Set filename");
                println!("Ctrl-G: Move to end, Ctrl-A: Move to start");
                println!("Ctrl-K: Cut line, Ctrl-U: Paste line");
                println!("Press Enter to start editing...");
                io::stdin().read_line(&mut String::new())?;

                editor.run()?;

                let content = editor.get_content();
                println!("Note content:\n{}", content);

                println!("Enter an instruction for the LLM (starting with @) or press Enter to skip:");
                let mut instruction = String::new();
                io::stdin().read_line(&mut instruction)?;

                if instruction.trim().starts_with('@') {
                    let instruction = instruction.trim_start_matches('@').trim();
                    let response = llm.process(instruction, content);
                    println!("LLM response:\n{}", response);
                }
            }
            "3" => {
                println!("Thank you for using Nomad. Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please choose 1, 2, or 3."),
        }

        println!("\nPress Enter to continue...");
        io::stdin().read_line(&mut String::new())?;
    }

    Ok(())
}