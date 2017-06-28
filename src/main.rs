extern crate image;
extern crate bson;
extern crate mongodb;
use std::path::Path;
mod get_locations;
mod heatmap_logic;
mod stamps;
mod colors;
mod configuration_logic;

fn main() {
    println!("Starting heatmap generation process");
    let configuration = configuration_logic::get_configuration();
    let ref path = &Path::new(&configuration.file_name);
    let stamp = stamps::get_stamp(&configuration.stamp, configuration.diameter);

    println!("Retrieving list of location points");
    let locations: Vec<(u32, u32)> = get_locations::get_locations(&configuration.location_source);

    println!("Generating frequency location matrix");
    let mut occurances = heatmap_logic::initialize_frequency_location_matrix(configuration.image_height, configuration.image_width);
    heatmap_logic::mutate_matrix_from_locations(locations, &mut occurances, &stamp, &configuration);

    println!("Generating RGBA values from frequency location matrix");
    let image = heatmap_logic::convert_frequency_location_matrix_to_rgba_vals(occurances, &configuration);

    println!("Saving PNG from RGBA values");
    image.save(path).unwrap();

    println!("Successfully generated PNG");
    println!("Finished!");
}
