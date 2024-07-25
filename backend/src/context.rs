pub struct Context {
    pub env: String,
    pub db: String,
}

impl Context {
    pub fn new() -> Self {
        Self {
            env: "harukun".to_string(),
            db: "yuorei".to_string(),
        }
    }
}
