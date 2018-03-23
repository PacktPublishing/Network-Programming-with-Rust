extern crate uuid;
extern crate lettre;
extern crate native_tls;

use std::env;
use lettre::{SendableEmail, EmailAddress, EmailTransport};
use lettre::smtp::{SmtpTransportBuilder, SUBMISSION_PORT};
use lettre::smtp::authentication::Credentials;
use lettre::smtp::client::net::ClientTlsParameters;

use native_tls::TlsConnector;

struct CrashReport {
    to: Vec<EmailAddress>,
    from: EmailAddress,
    message_id: String,
    message: Vec<u8>,
}

impl CrashReport {
    pub fn new(from_address: EmailAddress,
        to_addresses: Vec<EmailAddress>,
        message_id: String,
        message: String) -> CrashReport {
            CrashReport { from: from_address,
            to: to_addresses,
            message_id: message_id,
            message: message.into_bytes()
            }
        }
}

impl<'a> SendableEmail<'a, &'a [u8]> for CrashReport {
    fn to(&self) -> Vec<EmailAddress> {
        self.to.clone()
    }

    fn from(&self) -> EmailAddress {
        self.from.clone()
    }

    fn message_id(&self) -> String {
        self.message_id.clone()
    }

    fn message(&'a self) -> Box<&[u8]> {
        Box::new(self.message.as_slice())
    }
}

fn main() {
    let server = "smtp.foo.bar";
    let connector = TlsConnector::builder().unwrap().build().unwrap();
    let mut transport = SmtpTransportBuilder::new((server, SUBMISSION_PORT), lettre::ClientSecurity::Opportunistic(<ClientTlsParameters>::new(server.to_string(), connector)))
    .expect("Failed to create transport")
    .credentials(Credentials::new(env::var("USERNAME").unwrap_or_else(|_| "user".to_string()),
                    env::var("PASSWORD").unwrap_or_else(|_| "password".to_string())))
    .build();
    let report = CrashReport::new(EmailAddress::new("foo@bar.com".to_string()), vec![EmailAddress::new("foo@bar.com".to_string())], "foo".to_string(), "OOPS!".to_string());
    transport.send(&report).expect("Failed to send the report");
}
