pub mod localsmtp;

pub trait EmailClient {
    fn send(&self, sender: String, recipient: String, subject: String, body: String);
}