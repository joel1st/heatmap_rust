use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use std::fs::File;
use std::io::Read;

pub fn get_locations(location_source: &String) -> Vec<(u32, u32)> {
    match location_source.as_ref() {
        "mongo" => mongo(),
        "file" => read_file(),
        _ => mongo()
    }
}

pub fn read_file() -> Vec<(u32, u32)> {
    println!("Reading from file.");
    let mut file = File::open("points.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut points = vec![];
    let line_by_line = contents.split("\n");
    for line in line_by_line {
        let temp_collection: Vec<&str> = line.split(" ").collect();
        if temp_collection.len() == 2 {
            let x_point: u32 = temp_collection[0].parse().unwrap();
            let y_point: u32 = temp_collection[1].parse().unwrap();
            points.push((x_point, y_point));
        }
    }
    points
}

//+90 * 10 && +180 * 10
// Generate randomised data for (var i = 0; i < 2000; i++) { db.points.insert({point: [Math.floor(Math.random() * 10 * i), Math.floor(Math.random() * 10 * i )]}) }
pub fn mongo() -> Vec<(u32, u32)> {
    println!("Reading from mongo.");
    let client = Client::connect("127.0.0.1", 27017).expect("Failed to initialize standalone client.");

    let coll = client.db("local_chat").collection("location");

    // Find the document and receive a cursor
    let cursor = coll.find(None, None).ok().expect("Failed to execute find.");

    // cursor.next() returns an Option<Result<Document>>
    let mut denormalized_results = vec! [];
    for result in cursor {
        let val = match result {
            Ok(doc) => vec![doc.get("long").unwrap().clone(), doc.get("lat").unwrap().clone()],
            Err(_) => panic!("failed to get the things")
        };
        denormalized_results.push(
            (
                ((val[0].as_f64().unwrap() + 180.0) * 10.0) as u32 ,
                ((val[1].as_f64().unwrap() + 90.0) * 10.0) as u32
            )
        );
    }
    denormalized_results
}
