use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

// Generate randomised data for (var i = 0; i < 2000; i++) { db.points.insert({point: [Math.floor(Math.random() * 10 * i), Math.floor(Math.random() * 10 * i )]}) }
pub fn mongo() -> Vec<(u32, u32)> {
    let client = Client::connect("localhost", 27017)
    .expect("Failed to initialize standalone client.");

    let coll = client.db("locations").collection("points");

    // Find the document and receive a cursor
    let cursor = coll.find(None, None).ok().expect("Failed to execute find.");

    // cursor.next() returns an Option<Result<Document>>
    let mut denormalized_results = vec! [];
    for result in cursor {
        let val = match result {
            Ok(doc) => match doc.get("point"){
                Some(&Bson::Array(ref point)) => point.clone(),
                _ => panic!("Expected point to be a string!"),
            },
            Err(_) => panic!("failed to get the things")
        };
        denormalized_results.push((val[0].as_f64().unwrap() as u32, val[1].as_f64().unwrap() as u32));
    }
    denormalized_results
}
