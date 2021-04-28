pub trait PasswordService: Send + Sync {
    fn to_hash(&self, password: String) -> String;
    fn verify(&self, password: String, hash: String) -> bool;
}
