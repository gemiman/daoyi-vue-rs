use merge::Merge;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Merge)]
pub struct MailServerConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    enable: Option<bool>,
    #[merge(strategy = merge::option::overwrite_none)]
    send_mail_url: Option<String>,
}

impl MailServerConfig {
    pub fn enable(&self) -> bool {
        self.enable.unwrap_or(true)
    }
    pub fn send_mail_url(&self) -> &str {
        self.send_mail_url
            .as_deref()
            .unwrap_or("http://127.0.0.1:48001/admin-api/system/mail-account/send-mail-by-server")
    }
}
