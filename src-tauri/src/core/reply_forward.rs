use crate::error::DEmailError;
use crate::models::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyData {
    pub to: String,
    pub cc: Option<String>,
    pub subject: String,
    pub quoted_body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardData {
    pub subject: String,
    pub body_with_header: String,
}

pub fn prepare_reply(message: &Message, reply_all: bool) -> Result<ReplyData, DEmailError> {
    let to = message.from_header.clone();

    let cc = if reply_all {
        let mut cc_addresses = Vec::new();

        if let Some(original_to) = &message.to_header {
            if !original_to.is_empty() {
                cc_addresses.push(original_to.clone());
            }
        }

        if let Some(original_cc) = &message.cc_header {
            if !original_cc.is_empty() {
                cc_addresses.push(original_cc.clone());
            }
        }

        if cc_addresses.is_empty() {
            None
        } else {
            Some(cc_addresses.join(", "))
        }
    } else {
        None
    };

    let subject = if message.subject.starts_with("Re: ") {
        message.subject.clone()
    } else {
        format!("Re: {}", message.subject)
    };

    let original_body = message
        .body_plain
        .as_ref()
        .unwrap_or(&message.body_html.as_ref().unwrap_or(&String::from("")))
        .clone();

    let quoted_body = quote_original_message(&message.from_header, message.date, &original_body);

    Ok(ReplyData {
        to,
        cc,
        subject,
        quoted_body,
    })
}

pub fn prepare_forward(message: &Message) -> Result<ForwardData, DEmailError> {
    let subject = if message.subject.starts_with("Fwd: ") {
        message.subject.clone()
    } else {
        format!("Fwd: {}", message.subject)
    };

    let original_body = message
        .body_plain
        .as_ref()
        .unwrap_or(&message.body_html.as_ref().unwrap_or(&String::from("")))
        .clone();

    let body_with_header = format_forward_message(
        &message.from_header,
        &message.to_header.as_ref().unwrap_or(&String::from("")),
        &message.cc_header.as_ref().unwrap_or(&String::from("")),
        message.date,
        &message.subject,
        &original_body,
    );

    Ok(ForwardData {
        subject,
        body_with_header,
    })
}

fn quote_original_message(from: &str, date: i64, body: &str) -> String {
    let datetime = chrono::DateTime::from_timestamp(date, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Unknown date".to_string());

    let quoted_lines: Vec<String> = body.lines().map(|line| format!("> {}", line)).collect();

    format!(
        "\n\nOn {}, {} wrote:\n{}",
        datetime,
        from,
        quoted_lines.join("\n")
    )
}

fn format_forward_message(
    from: &str,
    to: &str,
    cc: &str,
    date: i64,
    subject: &str,
    body: &str,
) -> String {
    let datetime = chrono::DateTime::from_timestamp(date, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Unknown date".to_string());

    let mut forward_header = format!(
        "\n\n---------- Forwarded message ----------\nFrom: {}\nDate: {}\nSubject: {}\nTo: {}",
        from, datetime, subject, to
    );

    if !cc.is_empty() {
        forward_header.push_str(&format!("\nCc: {}", cc));
    }

    format!("{}\n\n{}", forward_header, body)
}
