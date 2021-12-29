use crate::json::google_hit;
pub mod json;
mod test;

fn main() {

    let content = "base_happiness".to_string();
    google_hit(content).ok();
}