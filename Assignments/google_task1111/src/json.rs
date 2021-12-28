use std::collections::HashMap;
/// main is a function that can use to get the URL path of the request
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Return Result<()> type

pub fn google_hit() -> reqwest::Result<()> {
    env_logger::init();
    log::info!("starting");
    let content = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon-species/ditto")?
        .json::<HashMap<String, serde_json::Value>>()?;
    let _check_1 = &content["names"][1]["name"];
    let _check_2 = &content["egg_groups"][0]["name"];

    println!("{:#?}", content.get("names"));
    Ok(())
}
