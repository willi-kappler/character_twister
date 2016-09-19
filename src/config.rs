extern crate clap;
use clap::{Arg, App};

#[derive(Clone,Debug)]
struct Configuration {
    input_path: String,
    font_size_min: u8,
    font_size_max: u8,
    font_path: String,

}

pub fn create_config() -> Configuration {
    let version = "0.1";

    let matches = App::new("Character Twister")
        .version(version).author("Willi Kappler <grandor@gmx.de>").about("OCR software written in Rust")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a configuration file")
            .takes_value(true))
        .arg(Arg::with_name("input")
            .help("Sets the input file or folder")
            .required(true)
            .index(1))
        .arg(Arg::with_name("version")
            .short("v")
            .long("version")
            .help("Shows version number")
        .arg(Arg::with_name("font_size")
            .short("s")
            .long("size")
            .help("Specify the range of font sizes")
        .arg(Arg::with_name("font_path")
            .short("f")
            .long("font")
            .help("Specify the font path")
        .get_matches();


}
