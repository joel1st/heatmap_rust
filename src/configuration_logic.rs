use std::env;
pub struct Configuration {
    pub diameter: i32,
    pub radius: i32,
    pub image_width: u32,
    pub image_height: u32,
    pub file_name: String,
    pub color: String,
    pub stamp: String,
    pub location_source: String
}

pub fn get_configuration() -> Configuration {
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
