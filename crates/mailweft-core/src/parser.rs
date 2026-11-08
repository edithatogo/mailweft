use crate::identity::sha256;
use crate::{Attachment, Config, Error, ObservedMessage, Party, Permission, RawEmail};
use chrono::DateTime;
use mail_parser::{Address, MessageParser, PartFlags};

pub(crate) fn parse(input: &RawEmail, c: &Config) -> Result<ObservedMessage, Error> {
    if input.permission == Permission::Unknown {
        return Err(Error("SOURCE_NOT_AUTHORISED"));
    }
    if input.source_ref.trim().is_empty() || input.source_ref.len() > 4096 {
        return Err(Error("INVALID_SOURCE_REFERENCE"));
    }
    if input.bytes.len() > c.max_message_bytes {
        return Err(Error("MESSAGE_TOO_LARGE"));
    }
    if input.bytes.is_empty() {
        return Err(Error("INVALID_MESSAGE"));
    }
    let msg = MessageParser::new()
        .max_parts(c.max_parts)
        .max_depth(c.max_depth)
        .parse(&input.bytes)
        .ok_or(Error("INVALID_MESSAGE"))?;
    let headers = msg.headers();
    if headers.is_empty() {
        return Err(Error("INVALID_MESSAGE"));
    }
    if msg.parts().any(|p| {
        p.flags()
            .intersects(PartFlags::LIMIT_REACHED | PartFlags::NESTING_LIMIT)
    }) {
        return Err(Error("MIME_LIMIT_REACHED"));
    }
    if msg.parts().any(|p| !p.flags().is_empty()) {
        return Err(Error("MIME_STRUCTURE_INVALID"));
    }
    let raw_hash = sha256(&input.bytes);
    let message_id = c.id("msg", &[raw_hash.as_bytes()]);
    let copy_id = c.id("copy", &[input.source_ref.as_bytes(), raw_hash.as_bytes()]);
    let mut warnings = Vec::new();
    let mut addresses = Vec::new();
    for (header_name, role) in [
        ("From", "from"),
        ("Sender", "sender"),
        ("To", "to"),
        ("Cc", "cc"),
        ("Bcc", "bcc"),
        ("Reply-To", "reply_to"),
    ] {
        for header in headers.all(header_name) {
            if let Some(list) = header.value().as_address() {
                if list.iter().any(|a| matches!(a, Address::Group(_))) {
                    warnings.push("ADDRESS_GROUP_STRUCTURE_NOT_PROJECTED".into());
                }
                for mailbox in list.mailboxes() {
                    let (party_id, identity_state) = if let Some(address) = mailbox.address() {
                        if let Some((local, domain)) = address
                            .rsplit_once('@')
                            .filter(|(l, d)| !l.is_empty() && !d.is_empty())
                        {
                            let normal = format!("{}@{}", local, domain.to_ascii_lowercase());
                            (c.id("party", &[normal.as_bytes()]), "explicit_address")
                        } else {
                            (
                                c.id(
                                    "unresolved",
                                    &[message_id.as_bytes(), role.as_bytes(), address.as_bytes()],
                                ),
                                "unresolved",
                            )
                        }
                    } else {
                        (
                            c.id(
                                "name",
                                &[
                                    message_id.as_bytes(),
                                    mailbox.name().unwrap_or("").as_bytes(),
                                ],
                            ),
                            "display_name_only",
                        )
                    };
                    addresses.push(Party {
                        party_id,
                        role: role.into(),
                        identity_state: identity_state.into(),
                    });
                }
            } else if !header.raw_value().iter().all(u8::is_ascii_whitespace) {
                warnings.push("ADDRESS_HEADER_UNRESOLVED".into());
            }
        }
    }
    if !addresses.iter().any(|a| a.role == "bcc") {
        warnings.push("BCC_VISIBILITY_UNKNOWN".into());
    }
    let dates: Vec<_> = headers.all("Date").collect();
    let header_date = if dates.len() == 1 && !dates[0].raw_value().windows(5).any(|s| s == b"-0000")
    {
        msg.date()
            .filter(|d| d.is_valid())
            .and_then(|d| DateTime::parse_from_rfc3339(&d.to_rfc3339()).ok())
    } else {
        None
    };
    if header_date.is_none() {
        warnings.push("HEADER_DATE_INVALID_OR_AMBIGUOUS".into());
    }
    let header_message_id_hash = if headers.all("Message-ID").count() == 1 {
        msg.message_id().map(|id| c.id("mid", &[id.as_bytes()]))
    } else {
        None
    };
    if header_message_id_hash.is_none() {
        warnings.push("MESSAGE_ID_MISSING_OR_AMBIGUOUS".into());
    }
    if headers.get("In-Reply-To").is_some() || headers.get("References").is_some() {
        warnings.push("THREAD_RESOLUTION_NOT_IMPLEMENTED".into());
    }
    if [
        "Resent-From",
        "Resent-To",
        "Resent-Cc",
        "Resent-Bcc",
        "Resent-Date",
        "Resent-Sender",
    ]
    .iter()
    .any(|n| headers.get(*n).is_some())
    {
        warnings.push("RESENT_HEADERS_NOT_PROJECTED".into());
    }
    if msg.messages().len() > 1 {
        warnings.push("NESTED_MESSAGE_NOT_PROJECTED".into());
    }
    let mut attachments = Vec::new();
    for part in msg.root().attachments() {
        // Transfer decoding is bounded by the admitted raw-message limit. No archive,
        // document converter, script, remote resource or model is ever executed.
        let (decoded, problems) = part.decoded_checked();
        if !problems.is_empty() {
            return Err(Error("MIME_DECODING_FAILED"));
        }
        if decoded.len() > c.max_message_bytes {
            return Err(Error("ATTACHMENT_TOO_LARGE"));
        }
        let mime_type = part
            .content_type()
            .map(|t| format!("{}/{}", t.ctype(), t.subtype().unwrap_or("octet-stream")))
            .unwrap_or_else(|| "application/octet-stream".into());
        attachments.push(Attachment {
            part_id: c.id(
                "part",
                &[message_id.as_bytes(), &u64::from(part.id()).to_be_bytes()],
            ),
            mime_type,
            size_bytes: decoded.len() as u64,
            filename_hash: part
                .attachment_name()
                .map(|name| c.id("filename", &[name.as_bytes()])),
            content_sha256: Some(sha256(&decoded)),
        });
    }
    warnings.sort();
    warnings.dedup();
    Ok(ObservedMessage {
        schema_version: "1.0.0".into(),
        source_copy_id: copy_id,
        message_candidate_id: message_id,
        source_sha256: raw_hash,
        source_kind: if input.permission == Permission::Synthetic {
            "synthetic_eml"
        } else {
            "eml"
        }
        .into(),
        observed_at: input.observed_at,
        header_date,
        source_received_at: None,
        header_message_id_hash,
        source_ref: Some(c.id("source", &[input.source_ref.as_bytes()])),
        observation_state: "observed_source_copy".into(),
        addresses,
        attachments,
        thread_evidence: Vec::new(),
        parse_warnings: warnings,
    })
}
