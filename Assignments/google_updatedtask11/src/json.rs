use std::collections::HashMap;

/// main is a function that can use to get the URL path of the request
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Return Result<std::string::String> type

pub fn google_hit(val: std::string::String) -> reqwest::Result<std::string::String> {
    log::info!("starting");
    let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto")?
        .json::<HashMap<String, serde_json::Value>>()?;
    let container = content["color"][val].to_owned();
    let var_name = match container {
        serde_json::Value::String(var_name) => var_name,
        _ => "red".to_string(),
    };
    println!("{:#?}", content.get("names"));
    Ok(var_name)
}
