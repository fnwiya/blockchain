impl Nonce {
    pub fn find_next(&self) -> u64 {
        let mut nonce = 1;
        while !Nonce::verify(self.current, nonce) {
            nonce += 1;
        }
        nonce
    }

    pub fn verify(current: u64, next: u64) -> bool {
        let message = format("{}{}", current, next);
        let digest = hash::sha256::digest(&message);
        digest.starts_with("000")
    }
}
