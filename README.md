# tempmail-rs

Fast, async, unofficial Rust client library and CLI for the **Temp-Mail** disposable email service (`com.tempmail`), reverse-engineered from the official Android mobile app.

Maintained by [NeuralS](https://github.com/neurals-et).

---

## Features

- **Blazing Fast**: Asynchronous I/O powered by `tokio` and `reqwest`.
- **Zero API Keys**: Uses the official mobile gateway (`https://mob2.temp-mail.org`) for free, immediate temporary email generation.
- **Automated OTP Extractor**: Built-in regex engine to automatically detect and extract 4–8 digit verification codes from incoming message subjects and preview bodies.
- **Full Email Inspection**: Read subject, sender, preview, full HTML body, and download raw RFC822 EML source.
- **Convenient CLI**: Intuitive subcommands (`tempmail new`, `tempmail inbox`, `tempmail code`, `tempmail read`).

---

## Installation

### From Source

```bash
git clone https://github.com/neurals-et/tempmail-rs.git
cd tempmail-rs
cargo build --release
```

Binary will be located at `target/release/tempmail`.

---

## CLI Usage

### 1. Generate a New Email
```bash
tempmail new
```
Output:
```
✔ New temporary mailbox created:
  Mailbox: felika7319@rexhot.com
  Token:   eyJhbGciOiJIUzI1NiIsInR5cCI6...
```

### 2. View Inbox
```bash
tempmail inbox
```

### 3. Automatically Wait and Extract OTP Code
```bash
tempmail code --timeout 60
```
Output:
```
Waiting for OTP code sent to: felika7319@rexhot.com
Polling every 3s (timeout: 60s)...

🎉 VERIFICATION CODE EXTRACTED:
  849201
```

### 4. Read Message Body & Source
```bash
tempmail read <message_id>
tempmail source <message_id>
```

---

## Rust Library Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
tempmail-rs = { git = "https://github.com/neurals-et/tempmail-rs" }
tokio = { version = "1.0", features = ["full"] }
```

### Example:

```rust
use tempmail::TempMailClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TempMailClient::new();

    // Create mailbox
    let mailbox = client.new_mailbox().await?;
    println!("Created: {}", mailbox.mailbox);

    // Check inbox
    let messages = client.get_messages(None).await?;
    for msg in messages {
        println!("Subject: {}", msg.subject);
    }

    // Wait for OTP verification code
    let otp = client.wait_for_otp(60, 3).await?;
    println!("Extracted OTP: {}", otp);

    Ok(())
}
```

---

## License

MIT License. Designed and maintained by NeuralS.
