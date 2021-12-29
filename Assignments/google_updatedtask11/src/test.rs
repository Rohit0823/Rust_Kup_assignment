#[cfg(test)]
mod tests {
    use crate::google_hit;
    #[test]
    pub fn string_success() {
        env_logger::init();
        let content = "name".to_string();
        assert_eq!(google_hit(content).unwrap(),"purple".to_string());
    }
    #[test]
    pub fn string_failure() {
        let content = "game".to_string();
        assert_eq!(google_hit(content).unwrap(),"red".to_string());
    }

}