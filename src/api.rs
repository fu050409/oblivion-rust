use p256::{ecdh::EphemeralSecret, PublicKey};

use super::sessions::Session;

pub(crate) fn request(
    method: &str,
    olps: &str,
    data: Option<Vec<u8>>,
    file: Option<Vec<u8>>,
    key_pair: (EphemeralSecret, PublicKey),
    tfo: bool,
) -> String {
    let session = Session::new();
    session.request(method, olps)
}

#[allow(dead_code)]
pub(crate) fn get(olps: &str) -> String {
    request("get", olps)
}

#[allow(dead_code)]
pub(crate) fn post(olps: &str) -> String {
    request("post", olps)
}

#[allow(dead_code)]
pub(crate) fn forward(olps: &str) -> String {
    request("forward", olps)
}
