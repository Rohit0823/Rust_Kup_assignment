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
pub fn hit_point() -> reqwest::Result<()> {
    env_logger::init();
    log::info!("starting");
    let content = reqwest::blocking::get("https://mocki.io/v1/82ddb819-004f-4c1c-ac7c-7b2a9157c058")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", content);
    Ok(())
}