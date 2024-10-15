pub fn sign(raw: String) -> String {
    let digest = md5::compute(raw.as_bytes());

    format!("{:x}", digest)
}
