    // the sync version of the Client is in the `sync`
use mongodb::{
    bson::doc,
    error::Result,
    sync::Client
};
fn _add_user() -> Result<()> {
    env_logger::init();
    log::info!("starting up");
    // Create the client by passing in a MongoDB connection string.
    //
    // coll: Get handle to the collection being used
    //
    // animals: animals is a database name which i created in MongoDB
    //
    // pets: pets is a collection name which i created in MongoDB
    let client =
        Client::with_uri_str("mongodb+srv://userid:password@cluster0.efr4z.mongodb.net/test?&w=majority")?;
    let coll = client
        .database("animals")
        .collection("pets");

    // new_pets: new_pets is a vector of documents to insert into the database
    //
    // doc: doc macro allows creation of BSON documents
    //
    // Insert the document into the database.
    let new_pets = vec![
        doc! { "type": "dog", "name": "Mikku" },
        doc! { "type": "rabbit", "name": "Tillu" },
        doc! { "type": "fish", "name": "Goldi" },
    ];
    coll.insert_many(new_pets, None)?;
    Ok(())
}

