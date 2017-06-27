use std::collections::HashMap;
extern crate image;
use image::ImageBuffer;
use std::path::Path;
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
// Generate randomised data for (var i = 0; i < 2000; i++) { db.points.insert({point: [Math.floor(Math.random() * 10 * i), Math.floor(Math.random() * 10 * i )]}) }

fn main() {
    println!("Hello, world!");
    let diameter: i32 = 51;
    let radius: i32 = diameter / 2;
    let imageWidth: u32 = 10000;
    let imageHeight: u32 = 10000;
    let stamp = generate_circle_array(diameter);

    let mut occurances : Vec<Vec<u16>> = vec![];
    for _ in 0..imageHeight {
        let mut temp_x_vec: Vec<u16> = vec![];
        for _ in 0..imageWidth {
            temp_x_vec.push(0);
        }
        occurances.push(temp_x_vec);
    }

    let locations: Vec<(u32, u32)> = get_locations();
    mutate_occurances_from_locations(locations, &mut occurances, &stamp, radius, imageHeight, imageWidth);

    generate_png_from_occurances(occurances, imageHeight, imageWidth);
    println!("hey there");
}

fn generate_circle_array(diamator: i32) -> Vec<Vec<u8>> {
  let center: i32 = diamator / 2;
  let radius2: i32 = center.pow(2);

  let mut stamp = vec![];
  for y in 0..diamator {
      let mut pixels = vec![];
	  for x in 0..diamator {
		let dx: i32 = x as i32 - center;
		let dy: i32 = y as i32 - center;
		let dx2: i32 = dx.pow(2);
		let dy2: i32 = dy.pow(2);
		pixels.push((dx2 + dy2 <= radius2) as u8);
	  }
      println!("{:?}", pixels);
	  stamp.push(pixels);
  }
  stamp
}

struct Color {
    min_occurances: u16,
    rgba: [u8; 4]
}

fn generate_color_array() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 1,
            rgba: [90, 200, 80, 200]
        },
        Color {
            min_occurances: 2,
            rgba: [200, 240, 100, 200]
        }
    ]
}

fn get_color_for_occurance(mut available_colors: &Vec<Color>, no_of_occurances: u16) -> [u8; 4] {
    let length = available_colors.len();
    for (index, color) in &mut available_colors.iter().enumerate() {
        if color.min_occurances == no_of_occurances {
            return color.rgba.clone();
        }
        if color.min_occurances > no_of_occurances {
            return available_colors[index - 1].rgba.clone();
        }
    }
    available_colors[length - 1].rgba.clone()
}

fn generate_png_from_occurances(occurances: Vec<Vec<u16>>, imageHeight: u32, imageWidth: u32) {
    let ref path = &Path::new("test.png");
    let mut available_colors = generate_color_array();
    let img = ImageBuffer::from_fn( imageWidth, imageHeight, |x, y| {
        let yelem = y;
        let xelem = x;
        let val = occurances[yelem as usize][xelem as usize];
        image::Rgba(get_color_for_occurance(&available_colors, val))
    });
    let _ = img.save(path).unwrap();
}

fn mutate_occurances_from_locations(
    locations: Vec<(u32, u32)>,
    occurances: &mut Vec<Vec<u16>>,
    stamp: &Vec<Vec<u8>>,
    radius: i32,
    imageHeight: u32,
    imageWidth: u32
) {
    for (x, y) in locations.into_iter() {
        println!("{} {}", x, y);
        for (y_ind, y_val) in stamp.into_iter().enumerate() {
            let y_pos = y as i32 + ((y_ind as i32 + 1i32) - radius);
            if y_pos < 0 || y_pos > (imageHeight as i32 - 1) {
                continue;
            }
            for (x_ind, x_val) in y_val.into_iter().enumerate() {
                let x_pos: i32 = x as i32 + ((x_ind as i32 + 1i32) - radius);
                if x_pos < 0 || x_pos > (imageWidth as i32 - 1) {
                    continue;
                }
                let new_val: u16 = occurances[y_pos as usize][x_pos as usize] as u16 + *x_val as u16;
                occurances[y_pos as usize][x_pos as usize] = new_val;
                // println!("{}", x_pos);
            }
        }
    }

}

fn get_locations() -> Vec<(u32, u32)> {
    let client = Client::connect("localhost", 27017)
    .expect("Failed to initialize standalone client.");

    let coll = client.db("locations").collection("points");

    // let doc = doc! { "title" => "Jaws", "array" => [ 1, 2, 3 ] };
    let doc = doc! {};

    // Find the document and receive a cursor
    let mut cursor = coll.find(None, None)
    .ok().expect("Failed to execute find.");

    // let item = cursor.next();

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
