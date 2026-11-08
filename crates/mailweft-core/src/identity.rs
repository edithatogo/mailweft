use crate::Config;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

pub(crate) fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

impl Config {
    pub(crate) fn id(&self, domain: &str, fields: &[&[u8]]) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key).expect("fixed-size HMAC key");
        for field in [
            b"mailweft-id-v1".as_slice(),
            self.scope.as_bytes(),
            domain.as_bytes(),
        ]
        .into_iter()
        .chain(fields.iter().copied())
        {
            mac.update(&(field.len() as u64).to_be_bytes());
            mac.update(field);
        }
        format!("{domain}_{:x}", mac.finalize().into_bytes())
    }
}
