use clap::{Arg, App};
// use std::env;
pub struct Configuration {
    pub diameter: i32,
    pub radius: i32,
    pub image_width: u32,
    pub image_height: u32,
    pub file_name: String,
    pub color: String,
    pub stamp: String,
    pub location_source: String,
    pub automated: bool
}

pub fn get_configuration() -> Configuration {
    let matches = App::new("Molten")
      .version("1.0")
      .author("Joel Olsson. <joel_1st@hotmail.com>")
      .about("Generates a heatmap")
      .arg(Arg::with_name("diameter")
          .long("diameter")
          .short("d")
          .takes_value(true)
          .help("Sets the diameter of the heatmap stamp"))
      .arg(Arg::with_name("image_width")
          .long("image_width")
          .short("w")
          .takes_value(true)
          .help("Sets the image width"))
      .arg(Arg::with_name("image_height")
          .long("image_height")
          .short("h")
          .takes_value(true)
          .help("Sets the image height"))
      .arg(Arg::with_name("file_output")
          .long("file_output")
          .short("o")
          .takes_value(true)
          .help("name of the file"))
      .arg(Arg::with_name("automated")
          .long("automated")
          .short("a")
          .takes_value(true)
          .help("automatically choose threshold levels for each color"))
      .arg(Arg::with_name("color")
          .long("color")
          .short("c")
          .takes_value(true)
          .help("name of the colour theme"))
      .arg(Arg::with_name("stamp")
          .long("stamp")
          .short("s")
          .takes_value(true)
          .help("stamp to use for the heat points"))
      .arg(Arg::with_name("location_source")
          .long("location_source")
          .short("l")
          .takes_value(true)
          .help("source for retrieving the heatmap points"))
      .get_matches();

    let diameter: i32 = value_t!(matches.value_of("diameter"), i32).unwrap_or_else(|_| 20);
    let radius: i32 = diameter / 2;
    let image_width: u32 = value_t!(matches.value_of("image_width"), u32).unwrap_or_else(|_| 2000);
    let image_height: u32 = value_t!(matches.value_of("image_height"), u32).unwrap_or_else(|_| 2000);
    let file_name: String = value_t!(matches.value_of("file_output"), String).unwrap_or_else(|_| String::from("test.png"));
    let color: String = value_t!(matches.value_of("color"), String).unwrap_or_else(|_| String::from("blue"));
    let stamp: String = value_t!(matches.value_of("stamp"), String).unwrap_or_else(|_| String::from("circle_discrete"));
    let location_source: String = value_t!(matches.value_of("location_source"), String).unwrap_or_else(|_| String::from("mongo"));
    let automated: bool = value_t!(matches.value_of("automated"), bool).unwrap_or_else(|_| false);

    Configuration {
        diameter: diameter,
        radius: radius,
        image_width: image_width,
        image_height: image_height,
        file_name: file_name,
        color: color,
        stamp: stamp,
        location_source: location_source,
        automated: automated,
    }
}
