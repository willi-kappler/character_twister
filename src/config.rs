use clap::{Arg, App};

#[derive(Clone,Debug)]
pub struct Configuration {
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
            .index(1))
        .arg(Arg::with_name("version")
            .short("v")
            .long("version")
            .help("Shows version number"))
        .arg(Arg::with_name("font_range")
            .short("s")
            .long("size")
            .help("Specify the range of font sizes to try, separated with a colon ':'. ex.: 8:20"))
        .arg(Arg::with_name("font_path")
            .short("f")
            .long("font")
            .help("Specify the font path"))
        .after_help(
            "Examples:\n\
             # uses default values for font range, name and input path:\n\
             # - font rage: 8:16, try font sizes between 8 and 16\n\
             # - font path: /usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf\n\
             # - input path: current folder './'\n\
             character_twister\n\n\
             character_twister -s 12:14  # set font size rage from 12 to 14\n\
            "
        )
        .get_matches();

        // Default values:
        let mut result = Configuration {
            input_path: "./".to_string(),
            font_size_min: 8,
            font_size_max: 16,
            font_path: "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf".to_string()
        };

        if let Some(config_file) = matches.value_of("config") {
            println!("Loading configuration file: {}", config_file);
        }

        // Command line parameter can overwrite configuration file settings

        if let Some(input_path) = matches.value_of("input") {
            result.input_path = input_path.to_string();
        }

        if let Some(font_path) = matches.value_of("font_path") {
            result.font_path = font_path.to_string();
        }

        if let Some(font_range) = matches.value_of("font_range") {
            //result.font_size_min = ;
            //result.font_size_max = ;
        }
/*
        if let Some() = matches.value_of("") {
            result. = ;
        }
*/
        result
}
