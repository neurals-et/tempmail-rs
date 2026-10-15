# tempmail-rs

> Fast, asynchronous unofficial Temp-Mail SDK and CLI in Rust.

Looking for the Python SDK & Interactive TUI? Check out [**tempmail-client**](https://github.com/etrnkz/tempmail-client).

Maintained by [NeuralS](https://github.com/neurals-et).

---

## Features

- **Blazing Fast**: Asynchronous I/O powered by `tokio` and `reqwest`.
- **Zero Configuration**: Uses the official mobile gateway (`mob2.temp-mail.org`) without API keys.
- **Automated OTP Extractor**: Built-in regex engine to automatically detect and extract 4–8 digit verification codes.
- **Full Email Inspection**: Inspect sender, subject, preview, full HTML, and raw RFC822 EML source.
- **CLI Commands**: Intuitive commands (`tempmail new`, `tempmail inbox`, `tempmail code`, `tempmail read`).

---

## Installation

```bash
git clone https://github.com/neurals-et/tempmail-rs.git
cd tempmail-rs
cargo build --release
```

Binary: `target/release/tempmail`

---

## CLI Usage

### Generate an Email
```bash
tempmail new
```

### Check Inbox
```bash
tempmail inbox
```

### Wait for Verification Code (OTP)
```bash
tempmail code --timeout 60
```

### Read Email Content
```bash
tempmail read <message_id>
tempmail source <message_id>
```

---

## Rust Library Usage

```rust
use tempmail::TempMailClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TempMailClient::new();

    // Create mailbox
    let mailbox = client.new_mailbox().await?;
    println!("Created: {}", mailbox.mailbox);

    // Fetch inbox
    let messages = client.get_messages(None).await?;
    for msg in messages {
        println!("Subject: {}", msg.subject);
    }

    // Wait for incoming OTP
    let otp = client.wait_for_otp(60, 3).await?;
    println!("Extracted OTP: {}", otp);

    Ok(())
}
```

---

## Related Projects

- [**tempmail-client**](https://github.com/etrnkz/tempmail-client) — Python SDK & interactive Textual TUI.

---

## License

MIT
