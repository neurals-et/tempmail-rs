use std::fs;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use tempmail::{TempMailClient, Result};

#[derive(Parser)]
#[command(name = "tempmail")]
#[command(about = "Fast, async unofficial Temp-Mail CLI in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a brand new temporary email address
    New,
    /// Check the active mailbox messages
    Inbox,
    /// Read full details of a specific message by ID
    Read {
        /// The ID of the message to read
        id: String,
    },
    /// Download raw RFC822 EML source of a message
    Source {
        /// The ID of the message
        id: String,
    },
    /// Wait for an incoming email and print the extracted OTP code
    Code {
        /// Timeout in seconds (default: 60)
        #[arg(short, long, default_value_t = 60)]
        timeout: u64,
    },
}

#[derive(Serialize, Deserialize, Default)]
struct SavedState {
    email: String,
    token: String,
}

fn config_path() -> PathBuf {
    dirs_next_or_home().join(".tempmail_rust.json")
}

fn dirs_next_or_home() -> PathBuf {
    if let Ok(home) = std::env::var("USERPROFILE") {
        PathBuf::from(home)
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
    } else {
        PathBuf::from(".")
    }
}

fn load_state() -> Option<SavedState> {
    let p = config_path();
    if p.exists() {
        if let Ok(content) = fs::read_to_string(&p) {
            return serde_json::from_str(&content).ok();
        }
    }
    None
}

fn save_state(email: &str, token: &str) {
    let state = SavedState {
        email: email.to_string(),
        token: token.to_string(),
    };
    if let Ok(json) = serde_json::to_string_pretty(&state) {
        let _ = fs::write(config_path(), json);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New => {
            let mut client = TempMailClient::new();
            let mb = client.new_mailbox().await?;
            if let Some(token) = &client.token {
                save_state(&mb.mailbox, token);
            }
            println!("{}", "✔ New temporary mailbox created:".green().bold());
            println!("  Mailbox: {}", mb.mailbox.cyan().bold());
            if let Some(token) = &mb.token {
                println!("  Token:   {}...", &token[..30.min(token.len())]);
            }
        }
        Commands::Inbox => {
            let state = load_state().expect("No mailbox active. Run 'tempmail new' first.");
            let client = TempMailClient::with_credentials(state.token, state.email.clone());
            println!("{} {}", "Active Mailbox:".bold(), state.email.cyan());
            
            let msgs = client.get_messages(None).await?;
            if msgs.is_empty() {
                println!("{}", "Inbox is empty (no messages received yet).".yellow());
            } else {
                println!("{} message(s) found:\n", msgs.len());
                for m in msgs {
                    println!("  {} {}", "• ID:".bold(), m._id.yellow());
                    println!("    From:    {}", m.from);
                    println!("    Subject: {}", m.subject.bold());
                    if let Some(prev) = m.body_preview {
                        println!("    Preview: {}", prev.dimmed());
                    }
                    println!();
                }
            }
        }
        Commands::Read { id } => {
            let state = load_state().expect("No mailbox active. Run 'tempmail new' first.");
            let client = TempMailClient::with_credentials(state.token, state.email);
            let detail = client.get_message_detail(&id).await?;
            
            println!("{} {}", "Subject:".bold(), detail.subject.green().bold());
            println!("{} {}", "From:   ".bold(), detail.from);
            println!("{} {}", "ID:     ".bold(), detail._id);
            println!("\n{}", "--- Body Preview ---".dimmed());
            println!("{}", detail.body_preview.unwrap_or_else(|| "(no preview)".into()));
            if let Some(html) = detail.body_html {
                println!("\n{}", "--- HTML Content ---".dimmed());
                println!("{}", html);
            }
        }
        Commands::Source { id } => {
            let state = load_state().expect("No mailbox active. Run 'tempmail new' first.");
            let client = TempMailClient::with_credentials(state.token, state.email);
            let raw = client.get_message_source(&id).await?;
            println!("{}", raw);
        }
        Commands::Code { timeout } => {
            let mut client = if let Some(state) = load_state() {
                TempMailClient::with_credentials(state.token, state.email)
            } else {
                let mut c = TempMailClient::new();
                let mb = c.new_mailbox().await?;
                if let Some(t) = &c.token {
                    save_state(&mb.mailbox, t);
                }
                c
            };

            let email_str = client.email.clone().unwrap_or_default();
            println!("{} {}", "Waiting for OTP code sent to:".yellow().bold(), email_str.cyan().bold());
            println!("Polling every 3s (timeout: {}s)...", timeout);

            match client.wait_for_otp(timeout, 3).await {
                Ok(code) => {
                    println!("\n{}", "🎉 VERIFICATION CODE EXTRACTED:".green().bold());
                    println!("  {}", code.bold().cyan());
                }
                Err(e) => {
                    eprintln!("{}", format!("Error: {}", e).red());
                }
            }
        }
    }

    Ok(())
}
