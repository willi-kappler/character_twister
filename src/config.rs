use clap::{Arg, App};
use yaml_rust::YamlLoader;

quick_error! {
    #[derive(Debug)]
    pub enum ConfigurationError {
        InvalidFontRange {}
        FontPathDoesNotExist {}
        InputPathDoesNotExist {}
        ErrorInConfigFile {}
    }
}

#[derive(Clone,Debug)]
pub struct Configuration {
    input_path: String,
    font_size_min: u8,
    font_size_max: u8,
    font_path: String,

}

fn load_config_file(file_name: String) -> Result<Configuration, ConfigurationError> {
    Ok(default_configuration())
}

fn default_configuration() -> Configuration {
    Configuration {
        input_path: "./".to_string(),
        font_size_min: 10,
        font_size_max: 10,
        font_path: "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf".to_string()
    }
}

fn parse_font_size(font_size: &str) -> Result<(u8, u8), ConfigurationError> {
    let font_size_min = 10;
    let font_size_max = 10;

    // Valid arguments:
    // 10
    // 10,15
    // 10-15
    // 10;15

    Ok((font_size_min, font_size_max))
}

pub fn create_config() -> Result<Configuration, ConfigurationError> {
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
            .help("Shows version number"))
        .arg(Arg::with_name("font_size")
            .short("s")
            .long("size")
            .help("Specify the range of font sizes"))
        .arg(Arg::with_name("font_path")
            .short("f")
            .long("font")
            .help("Specify the font path"))
        .get_matches();

    let mut configuration = default_configuration();

    if let Some(config_file_name) = matches.value_of("config") {
        println!("Loading configuration file: {}", config_file_name);
        configuration = try!(load_config_file(config_file_name.to_string()));
    }

    // Command line parameter can overwrite configuration file settings

    // Save to call unwrap() here since "input" is mandatory
    configuration.input_path = matches.value_of("input").unwrap().to_string();

    if let Some(font_size) = matches.value_of("font_size") {
        let (font_size_min, font_size_max) = try!(parse_font_size(font_size));
        configuration.font_size_min = font_size_min;
        configuration.font_size_max = font_size_max;
    }

    if let Some(font_path) = matches.value_of("font_path") {
        configuration.font_path = font_path.to_string();
    }

    Ok(configuration)
}
