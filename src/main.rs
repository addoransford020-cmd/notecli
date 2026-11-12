use clap::{Parser, Subcommand};
use colored::*;
use notecli::{print_note_full, print_note_summary, NoteStore};

#[derive(Parser)]
#[command(
    name = "notecli",
    version = "0.1.0",
    author = "Ransford Addo",
    about = "Take, search and manage notes right from your terminal",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new note
    Add {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        body: String,
        #[arg(long, default_value = "")]
        tags: String,
    },
    /// List all notes
    List,
    /// View a single note
    View {
        id: u64,
    },
    /// Search notes by keyword
    Search {
        query: String,
    },
    /// Edit an existing note
    Edit {
        id: u64,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        body: Option<String>,
        #[arg(long)]
        tags: Option<String>,
    },
    /// Delete a note by ID
    Delete {
        id: u64,
    },
    /// Show where notes are stored
    Path,
}

fn main() {
    let cli = Cli::parse();
    let mut store = NoteStore::load();

    match cli.command {
        Commands::Add { title, body, tags } => {
            let tag_list: Vec<String> = if tags.is_empty() {
                vec![]
            } else {
                tags.split(',')
                    .map(|t| t.trim().to_string())
                    .filter(|t| !t.is_empty())
                    .collect()
            };
            let id = store.add(title.clone(), body, tag_list);
            println!(
                "{} Note {} added: \"{}\"",
                "✓".green().bold(),
                format!("#{id}").yellow(),
                title
            );
        }

        Commands::List => {
            let notes = store.list();
            if notes.is_empty() {
                println!("{}", "No notes yet. Use `notecli add` to create one.".dimmed());
            } else {
                println!("{}\n", format!("── {} note(s) ──", notes.len()).cyan().bold());
                for note in notes {
                    print_note_summary(note);
                    println!();
                }
            }
        }

        Commands::View { id } => {
            match store.get(id) {
                Some(note) => print_note_full(note),
                None => eprintln!("{} Note #{} not found.", "✗".red().bold(), id),
            }
        }

        Commands::Search { query } => {
            let results = store.search(&query);
            if results.is_empty() {
                println!("{}", format!("No notes matched \"{}\".", query).dimmed());
            } else {
                println!(
                    "{}\n",
                    format!("── {} result(s) for \"{}\" ──", results.len(), query).cyan().bold()
                );
                for note in results {
                    print_note_summary(note);
                    println!();
                }
            }
        }

        Commands::Edit { id, title, body, tags } => {
            let parsed_tags = tags.map(|t| {
                t.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            });
            if store.edit(id, title, body, parsed_tags) {
                println!("{} Note {} updated.", "✓".green().bold(), format!("#{id}").yellow());
            } else {
                eprintln!("{} Note #{} not found.", "✗".red().bold(), id);
            }
        }

        Commands::Delete { id } => {
            if store.delete(id) {
                println!("{} Note {} deleted.", "✓".green().bold(), format!("#{id}").yellow());
            } else {
                eprintln!("{} Note #{} not found.", "✗".red().bold(), id);
            }
        }

        Commands::Path => {
            println!(
                "Notes stored at: {}",
                NoteStore::storage_path().display().to_string().cyan()
            );
        }
    }
}
