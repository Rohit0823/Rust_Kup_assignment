#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::Value::String;
    use std::string::String as OtherString;

    #[test]
    fn check_first_success() {
        let resp =   String(
            "Metamon".to_string(),
        );
        let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto").unwrap()
            .json::<HashMap< OtherString, serde_json::Value>>().unwrap();
        let check_1 = &content["names"][1]["name"];
        assert_eq!(*check_1,resp);
    }
    #[test]
    fn check_second_success() {
        let resp =   String(
            "ditto".to_string(),
        );
        let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto").unwrap()
            .json::<HashMap< OtherString, serde_json::Value>>().unwrap();
        let check_2 = &content["egg_groups"][0]["name"];
        assert_eq!(*check_2,resp);
    }
    #[test]
    fn check_failure() {
        let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto").unwrap()
            .json::<HashMap< OtherString, serde_json::Value>>().unwrap();
        let check_3 = &content["capture_rate"];
        assert_ne!(*check_3,70);
    }
    #[test]
    pub fn _google_hit() {
        let url = "https://pokeapi.co/api/v2/pokemon-species/ditto";
        let content = reqwest::blocking::get(url).unwrap();
        assert_eq!(content.url().as_str(), url);
        assert_eq!(content.status(), reqwest::StatusCode::OK);
    }

}