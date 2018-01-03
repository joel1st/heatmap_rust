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
        "blues" => blues(),
        "blues1" => blues1(),
        "blues2" => blues2(),
        "blues3" => blues3(),
        "blues4" => blues4(),
        "blues5" => blues5(),
        "blues6" => blues6(),
        "blues7" => blues7(),
        "blues8" => blues8(),
        "blues9" => blues9(),
        "blues10" => blues10(),
        "blues11" => blues11(),
        "blues12" => blues12(),
        "blues13" => blues13(),
        "blues14" => blues14(),
        "blues15" => blues15(),
        "reds" => reds(),
        "purples" => purples(),
        _ => red()
    };
    if configuration.automated {
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
        if i == movable_colors - 1 {
            occurances_vec.push(max_frequency);
            break;
        }
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
            rgba: [20, 90, 50, 200]
        },
        Color {
            min_occurances: 2,
            rgba: [80, 100, 70, 200]
        },
        Color {
            min_occurances: 3,
            rgba: [90, 200, 80, 200]
        },
        Color {
            min_occurances: 4,
            rgba: [220, 40, 70, 200]
        },
        Color {
            min_occurances: 5,
            rgba: [200, 240, 100, 240]
        },
        Color {
            min_occurances: 6,
            rgba: [250, 240, 190, 250]
        },
        Color {
            min_occurances: 7,
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

// pub fn spectral() -> Vec<Color> {
// }
pub fn blues() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 3,
            rgba: [106, 172, 212, 255]
        }
    ]
}

pub fn reds() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 3,
            rgba: [128, 125, 186, 255]
        }
    ]
}

pub fn purples() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 3,
            rgba: [251, 146, 114, 255]
        }
    ]
}



pub fn blues1() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 250,
            rgba: [255, 0, 0, 255]
        }
    ]
}

pub fn blues2() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 100,
            rgba: [255, 0, 170, 255]
        }
    ]
}

pub fn blues3() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [170, 0, 255, 255]
        }
    ]
}

pub fn blues4() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [0, 0, 255, 255]
        }
    ]
}

pub fn blues5() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [0, 170, 255, 255]
        }
    ]
}

pub fn blues6() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [0, 255, 170, 0]
        }
    ]
}

pub fn blues7() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [0, 255, 0, 255]
        }
    ]
}

pub fn blues8() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [177, 255, 0, 255]
        }
    ]
}

pub fn blues9() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [255, 170, 0, 255]
        }
    ]
}

pub fn blues10() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [210, 177, 42, 255]
        }
    ]
}

pub fn blues11() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [156, 137, 60, 255]
        }
    ]
}

pub fn blues12() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [137, 126, 83, 255]
        }
    ]
}

pub fn blues13() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [111, 101, 60, 255]
        }
    ]
}

pub fn blues14() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [63, 58, 39, 255]
        }
    ]
}

pub fn blues15() -> Vec<Color> {
    vec! [
        Color {
            min_occurances: 0,
            rgba: [0, 0, 0, 0]
        },
        Color {
            min_occurances: 40,
            rgba: [27, 26, 23, 255]
        }
    ]
}
