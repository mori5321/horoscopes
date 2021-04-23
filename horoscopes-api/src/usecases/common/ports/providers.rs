pub trait IDProvider: Send + Sync{
    fn generate(&self) -> String;
}
