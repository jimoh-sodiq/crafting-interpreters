pub struct LoxError {
    pub had_error: bool,
}

impl LoxError {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn set_error(&mut self, error: bool) {
        self.had_error = error
    }

    pub fn report(&mut self, line: u32, message: String) {
        println!("[ Error on line {} ] :  {}", line, message);
        self.set_error(true);
    }
}
