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
fn main() -> reqwest::Result<()> {
    env_logger::init();
    log::info!("starting");
    let content = reqwest::blocking::get("https://mocki.io/v1/4e423f92-67c3-4da4-94bb-3a14fab78454")?
        .json::<HashMap<String, i32>>()?;
    println!("{:#?}", content.values());
    Ok(())
}
