extern crate image;
extern crate bson;
extern crate mongodb;
use image::ImageBuffer;
use std::path::Path;
use std::env;
mod colors;
mod get_locations;
mod stamps;

fn main() {
    println!("Starting heatmap generation process");
    let configuration = get_configuration();
    let ref path = &Path::new(&configuration.file_name);
    let stamp = stamps::get_stamp(&configuration.stamp, configuration.diameter);

    println!("Retrieving list of location points");
    let locations: Vec<(u32, u32)> = get_locations::get_locations(&configuration.location_source);

    println!("Generating frequency location matrix");
    let mut occurances = initialize_frequency_location_matrix(configuration.image_height, configuration.image_width);
    mutate_matrix_from_locations(locations, &mut occurances, &stamp, configuration.radius, configuration.image_height, configuration.image_width);

    println!("Generating RGBA values from frequency location matrix");
    let image = convert_frequency_location_matrix_to_rgba_vals(occurances, &configuration);

    println!("Saving PNG from RGBA values");
    image.save(path).unwrap();

    println!("Successfully generated PNG");
    println!("Finished!");
}

struct Configuration {
    diameter: i32,
    radius: i32,
    image_width: u32,
    image_height: u32,
    file_name: String,
    color: String,
    stamp: String,
    location_source: String
}

fn get_configuration() -> Configuration {
    // for argument in env::args() {
    //     println!("{}", argument);
    // }
    let diameter: i32 = 500;
    let radius: i32 = diameter / 2;
    let image_width: u32 = 10000;
    let image_height: u32 = 10000;
    Configuration {
        diameter: diameter,
        radius: radius,
        image_width: image_width,
        image_height: image_height,
        file_name: String::from("test.png"),
        color: String::from("blue"),
        stamp: String::from("circle"),
        location_source: String::from("mongo")
    }
}

fn initialize_frequency_location_matrix(image_height: u32, image_width: u32) -> Vec<Vec<u16>>{
    let mut occurances : Vec<Vec<u16>> = vec![];
    for _ in 0..image_height {
        let mut temp_x_vec: Vec<u16> = vec![];
        for _ in 0..image_width {
            temp_x_vec.push(0);
        }
        occurances.push(temp_x_vec);
    }
    occurances
}

fn mutate_matrix_from_locations(
    locations: Vec<(u32, u32)>,
    occurances: &mut Vec<Vec<u16>>,
    stamp: &Vec<Vec<u8>>,
    radius: i32,
    image_height: u32,
    image_width: u32
) {
    for (x, y) in locations.into_iter() {
        for (y_ind, y_val) in stamp.into_iter().enumerate() {
            let y_pos = y as i32 + ((y_ind as i32 + 1i32) - radius);
            if y_pos < 0 || y_pos > (image_height as i32 - 1) {
                continue;
            }
            for (x_ind, x_val) in y_val.into_iter().enumerate() {
                let x_pos: i32 = x as i32 + ((x_ind as i32 + 1i32) - radius);
                if x_pos < 0 || x_pos > (image_width as i32 - 1) {
                    continue;
                }
                let new_val: u16 = occurances[y_pos as usize][x_pos as usize] as u16 + *x_val as u16;
                occurances[y_pos as usize][x_pos as usize] = new_val;
            }
        }
    }
}

fn convert_frequency_location_matrix_to_rgba_vals(occurances: Vec<Vec<u16>>, configuration: &Configuration)-> image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>{
    let available_colors = colors::get_color(&configuration.color);
    let img = ImageBuffer::from_fn( configuration.image_width, configuration.image_height, |x, y| {
        let yelem = y;
        let xelem = x;
        let val = occurances[yelem as usize][xelem as usize];
        image::Rgba(get_color_for_occurance(&available_colors, val))
    });
    img
}

fn get_color_for_occurance(mut available_colors: &Vec<colors::Color>, no_of_occurances: u16) -> [u8; 4] {
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
