use anyhow::Result;
use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport};

#[derive(Clone)]
pub struct Client {
    transport: SmtpTransport,
}

impl Client {
    pub fn new(host: &str, port: u16) -> Result<Self> {
        let transport = SmtpTransport::builder_dangerous(host).port(port).build();
        Ok(Self { transport })
    }

    pub fn send(&self, sender: String, recipient: String, subject: String, body: String) {
        let html_body = if body.contains('<') && body.contains('>') {
            body
        } else {
            format!("<p>{}</p>", body.replace('\n', "<br>"))
        };

        let email = Message::builder()
            .from(sender.parse().unwrap())
            .to(recipient.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .unwrap();

        match self.transport.send(&email) {
            Ok(_) => println!("Email sent successfully to {}", recipient),
            Err(e) => eprintln!("Failed to send email to {}: {}", recipient, e),
        }
    }
}
