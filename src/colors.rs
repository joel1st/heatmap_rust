use super::configuration_logic::Configuration;

pub struct Color {
    pub min_occurances: u16,
    pub rgba: [u8; 4]
}

impl Color {
    fn update_occurance(&mut self, new_min_occurance: u16) {
        self.min_occurances = new_min_occurance
    }
}

pub fn get_color_scheme(configuration: &Configuration, max_frequency: u16) -> Vec<Color> {
    let mut colour_scheme = match configuration.color.as_ref() {
        "blue" => blue(),
        "red" => red(),
        _ => red()
    };
    if true {
        let occurances = generate_min_occurances(colour_scheme.len() as u16, max_frequency);
        println!("{:?}", occurances);
        for (index, color) in colour_scheme.iter_mut().enumerate() {
            color.update_occurance(occurances[index]);
        }
    }

    colour_scheme
}

pub fn generate_min_occurances (number_of_colors: u16, max_frequency: u16)-> Vec<u16> {
    if number_of_colors == 1 {
        panic!("Colour schemes should have at least 2 colours.");
    }
    let mut occurances_vec = vec![0, 1];
    let movable_colors: u16 = number_of_colors - 1u16;
    let steps: u16 = max_frequency / movable_colors;
    for i in 1..movable_colors {
        occurances_vec.push(steps * i);
    }
    occurances_vec
}

pub fn blue() -> Vec<Color> {
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
            min_occurances: 11,
            rgba: [220, 40, 70, 200]
        },
        Color {
            min_occurances: 10,
            rgba: [200, 240, 100, 240]
        },
        Color {
            min_occurances: 10,
            rgba: [250, 240, 190, 250]
        },
        Color {
            min_occurances: 10,
            rgba: [255, 255, 255, 250]
        }
    ]
}

pub fn red() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 1,
            rgba: [99, 0, 89, 250]
        },
        Color {
            min_occurances: 10,
            rgba: [130, 0, 99, 250]
        }
    ]
}
