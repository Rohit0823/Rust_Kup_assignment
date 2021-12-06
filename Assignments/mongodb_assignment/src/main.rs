use mongodb::{
    bson::doc,
    error::Result,

    //  the sync version of the Client is in the `sync`
    sync::Client
};

fn main() -> Result<()> {
    // Create the client by passing in a MongoDB connection string.
    let client =
        Client::with_uri_str("mongodb+srv://rohit:<password>@cluster0.qkiat.mongodb.net/myFirstDatabase?retryWrites=true&w=majority")?;

    // Get a handle to the collection being used.
    let coll = client
        .database("animals")
        .collection("pets");

    // Create a vector of documents to insert into the database by
    // using the `doc` macro, which allows creation of BSON documents
    let new_pets = vec![
        doc! { "type": "dog", "name": "Rondo" },
        doc! { "type": "cat", "name": "Smokey" },
        doc! { "type": "cat", "name": "Phil" },
    ];

    // Insert the document into the database.
    coll.insert_many(new_pets, None)?;

    Ok(())
}
