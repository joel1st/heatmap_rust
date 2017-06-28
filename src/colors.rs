pub struct Color {
    pub min_occurances: u16,
    pub rgba: [u8; 4]
}

pub fn get_color(color: &String) -> Vec<Color> {
    match color.as_ref() {
        "blue" => blue(),
        "red" => red(),
        _ => red()
    }
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
            min_occurances: 10,
            rgba: [200, 240, 100, 200]
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
