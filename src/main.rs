use std::collections::HashMap;
extern crate image;
use image::ImageBuffer;
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let diameter: i32 = 51;
    let stamp = generate_circle_array(diameter);

    let mut occurances : Vec<Box<[u16; 36000]>> = vec![];
    for _ in 0..18000 {
	    occurances.push(Box::new([0; 36000]))
    }

    let locations: Vec<(u32, u32)> = get_locations();
    mutate_occurances_from_locations(locations, &mut occurances, &stamp);

   generate_png_from_occurances(occurances);
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

fn generate_png_from_occurances(occurances: Vec<Box<[u16; 36000]>>) {
    let mut colors = HashMap::new();
    colors.insert(0, [0, 0, 0, 0]);
    colors.insert(1, [90, 200, 80, 200]);
    colors.insert(2, [200, 240, 100, 200]);

    let ref path = &Path::new("test.png");
    let img = ImageBuffer::from_fn( 36000, 16000, |x, y| {
        let yelem = y;
        let xelem = x;
        let val = occurances[yelem as usize][xelem as usize];

        match colors.get(&val)  {
            Some(rgba) => image::Rgba(rgba.clone()),
            None => {
                println!("no match!");
                image::Rgba([0, 0, 0, 0])
            }
        }
    });
    let _ = img.save(path).unwrap();
}

fn mutate_occurances_from_locations(locations: Vec<(u32, u32)>, occurances: &mut Vec<Box<[u16; 36000]>>, stamp: &Vec<Vec<u8>>) {
    let radius: i32 = 25;
    for (x, y) in locations.into_iter() {
        println!("{} {}", x, y);
        for (y_ind, y_val) in stamp.into_iter().enumerate() {
            let y_pos = y as i32 + ((y_ind as i32 + 1i32) - radius);
            if y_pos < 0 || y_pos > 17999 {
                continue;
            }
            for (x_ind, x_val) in y_val.into_iter().enumerate() {
                let x_pos: i32 = x as i32 + ((x_ind as i32 + 1i32) - radius);
                if x_pos < 0 || x_pos > 35999 {
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
	vec![ (200, 90), (2100, 180), (210, 500), (70, 80), (11, 8) ]
}
