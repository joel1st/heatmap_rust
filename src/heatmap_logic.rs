use image;
use image::ImageBuffer;
use super::colors;
use super::configuration_logic::Configuration;

pub fn initialize_frequency_location_matrix(image_height: u32, image_width: u32) -> Vec<Vec<u16>>{
    let mut frequency_location_matrix : Vec<Vec<u16>> = vec![];
    for _ in 0..image_height {
        let mut temp_x_vec: Vec<u16> = vec![];
        for _ in 0..image_width {
            temp_x_vec.push(0);
        }
        frequency_location_matrix.push(temp_x_vec);
    }
    frequency_location_matrix
}

pub fn mutate_matrix_from_locations(
    locations: Vec<(u32, u32)>,
    location_matrix: &mut Vec<Vec<u16>>,
    stamp: &Vec<Vec<u8>>,
    configuration: &Configuration,
) {
    for (x, y) in locations.into_iter() {
        for (y_ind, y_val) in stamp.into_iter().enumerate() {
            let y_matrix_pos = matrix_position(y, y_ind, configuration.radius);
            if out_of_matrix_bounds(y_matrix_pos, configuration.image_height) {
                continue;
            }
            for (x_ind, x_val) in y_val.into_iter().enumerate() {
                let x_matrix_pos = matrix_position(x, x_ind, configuration.radius);
                if out_of_matrix_bounds(x_matrix_pos, configuration.image_width) {
                    continue;
                }
                location_matrix[y_matrix_pos as usize][x_matrix_pos as usize] += *x_val as u16;
            }
        }
    }
}

fn matrix_position(location_position: u32, stamp_index: usize, radius: i32) -> i32 {
    location_position as i32 + ((stamp_index as i32 + 1i32) - radius)
}

fn out_of_matrix_bounds(position: i32, max_dimension: u32) -> bool {
    position < 0 || position > (max_dimension as i32 - 1)
}

pub fn convert_frequency_location_matrix_to_rgba_vals(occurances: Vec<Vec<u16>>, configuration: &Configuration)-> image::ImageBuffer<image::Rgba<u8>, Vec<u8>>{
    let available_colors = colors::get_color(&configuration.color);
    let img = ImageBuffer::from_fn( configuration.image_width, configuration.image_height, |x, y| {
        let yelem = y;
        let xelem = x;
        let val = occurances[yelem as usize][xelem as usize];
        image::Rgba(get_color_for_occurance(&available_colors, val))
    });
    img
}

pub fn get_color_for_occurance(mut available_colors: &Vec<colors::Color>, no_of_occurances: u16) -> [u8; 4] {
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
