use lettre::{SmtpClient, Transport};
use lettre_email::Email;
use std::process::{Command};

static SUBJECT: &str = "Procespy Report";
static MESSAGE: &str = "Sorry! we have to terminate ";
static SUCCESS_EMAIL: &str = "Email Sent";
static SUCCESS_POPUP: &str = "Popup Successfully Displayed";
static FAILURE_EMAIL: &str = "Invalid Email";
static FAILURE_POPUP: &str = "Unable to Display Popup";
static SENDER_EMAIL: &str = "pawan.bisht@knoldus.com";

pub fn send_email(receiver: &str, service_name: &str) -> &'static str {
    let mut body: String = MESSAGE.to_string();
    body.push_str(service_name);

    match Email::builder()
        .to(receiver)
        .from(SENDER_EMAIL)
        .subject(SUBJECT)
        .body(body)
        .build() {
        Ok(email) => {
            let mailer: SmtpClient = SmtpClient::new_unencrypted_localhost().unwrap();
            mailer.transport().send(email.clone().into()).unwrap();
            SUCCESS_EMAIL
        }
        Err(_error) => FAILURE_EMAIL
    }
}

pub fn display_popup(service_name: &str, popup_command: &str) -> &'static str {
    let mut message: String = "--text=Sorry! we have to terminate ".to_string();
    message.push_str(service_name);
    match  Command::new(popup_command)
        .args(&["--warning", message.as_str(),"--title=Application",])
        .output() {
        Ok(_success) => SUCCESS_POPUP,
        Err(_error) => FAILURE_POPUP
    }
}

#[cfg(test)]
mod test {
    use crate::notification::{display_popup, FAILURE_POPUP, SUCCESS_POPUP, SUCCESS_EMAIL, FAILURE_EMAIL, send_email};

    static INVALID_RECEIVER: &str = "pawan@gmail";
    static RECEIVER: &str = "pawanbisht62@gmail.com";
    static SERVICE_NAME: &str = "test";
    static POPUP_COMMAND: &str = "zenity";
    static INVALID_POPUP_COMMAND: &str = "zenit";

    #[test]
    fn test_send_email_success() {
        assert_eq!(send_email(RECEIVER, SERVICE_NAME), SUCCESS_EMAIL);
    }

    #[test]
    fn test_send_email_failure() {
        assert_eq!(send_email(INVALID_RECEIVER, SERVICE_NAME), FAILURE_EMAIL);
    }

    #[test]
    fn test_display_popup_success() {
        assert_eq!(display_popup(SERVICE_NAME, POPUP_COMMAND), SUCCESS_POPUP);
    }

    #[test]
    fn test_display_popup_failure() {
        assert_eq!(display_popup(SERVICE_NAME, INVALID_POPUP_COMMAND), FAILURE_POPUP);
    }


}