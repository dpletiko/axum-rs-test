use std::env;
use lazy_static::lazy_static;
use mail_send::{SmtpClientBuilder, mail_builder::{MessageBuilder, headers::address::Address}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mailer {
    mailer: String,
    host: String,
    port: u16,
    username: String,
    secret: String,
}

pub struct Mailable<'x> {
    pub to: Address<'x>,
    pub content: String,
    pub subject: String,
}

impl Mailer {
    pub async fn send_mail<'x>(message: Mailable<'x>) {
        // Build a simple multipart message
        let message = MessageBuilder::new()
            .from(("John Doe", "john@example.com"))
            .to(vec![
                ("Jane Doe", "jane@example.com"),
                ("James Smith", "james@test.com"),
            ])
            .subject(message.subject)
            .html_body("<h1>Hello, world!</h1>")
            .text_body("Hello sss!");

        // Connect to the SMTP submissions port, upgrade to TLS and
        // authenticate using the provided credentials.
        SmtpClientBuilder::new(&*MAILER.host, MAILER.port)
            .implicit_tls(false)
            .credentials((&*MAILER.username, &*MAILER.secret))
            .connect()
            .await
            .unwrap()
            .send(message)
            .await
            .unwrap();
    }
}

lazy_static! {
    pub static ref MAILER: Mailer = Mailer {
        mailer: env::var("MAIL_MAILER").expect("Mail host not set"),
        host: env::var("MAIL_HOST").expect("Mail port not set"),
        port: env::var("MAIL_PORT").expect("Mail port not set").parse::<u16>().unwrap(),
        username: env::var("MAIL_USERNAME").expect("Mail username not set"),
        secret: env::var("MAIL_PASSWORD").expect("Mail password not set")
    };
}


// impl Into<Credentials<String>> for Mailer {
//     fn into(self) -> Credentials<std::string::String> {
//         Credentials::Plain {
//             username: String::from(self.username),
//             secret: String::from(self.secret)
//         }
//     }
// }
