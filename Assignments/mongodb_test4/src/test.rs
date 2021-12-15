
pub fn _add_user(name: &str) -> String {
    format!("new_pets {}", name)
}
pub use crate::mongodb_rust::_add_user;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert_name_success() {
        let result = add_user("mikku");
        assert!(result.contains("mikku"));
    }
    #[test]
    fn insert_name_failure() {
        let result = add_user("boxer");
        assert!(result.contains("boxer"),
                "boxer is not inserted '{}'",
                result
        );
    }
}
