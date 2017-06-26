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

#[derive(Clone, Debug)]
pub struct Configuration {
    pub input_path: String,
    pub font_size_min: u8,
    pub font_size_max: u8,
    pub font_name: String,

}

fn default_config() -> Configuration {
    Configuration {
        input_path: String::from("./"),
        font_size_min: 8,
        font_size_max: 16,
        font_name: String::from("FreeMono")
    }
}

pub fn create_config() -> Configuration {
    let version = "0.1";

    let matches = App::new("Character Twister")
        .version(version).author("Willi Kappler <grandor@gmx.de>").about("OCR software written in Rust")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a configuration file, command line arguments can overwrite values from the config file")
            .takes_value(true))
        .arg(Arg::with_name("input")
            .help("Sets the input file or folder, default is current folder './'")
            .index(1))
        .arg(Arg::with_name("version")
            .short("v")
            .long("version")
            .help("Shows version number"))
        .arg(Arg::with_name("font_range")
            .short("s")
            .long("size")
            .help("Specify the range of font sizes to try, separated with a colon ':'. ex.: 8:16 which is the default")
            .takes_value(true))
        .arg(Arg::with_name("font_name")
            .short("f")
            .long("font")
            .help("Specify the font to use")
            .takes_value(true))
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
    let mut result = default_config().clone();

    if let Some(config_file) = matches.value_of("config") {
        result = load_config(config_file);
    }

    // Command line parameter can overwrite configuration file settings
    if let Some(input_path) = matches.value_of("input") {
        result.input_path = input_path.to_string();
        println!("input path: {}", input_path);
    }

    if let Some(font_name) = matches.value_of("font_name") {
        result.font_name = font_name.to_string();
        println!("using font: {}", font_name);
    }

    if let Some(font_range) = matches.value_of("font_range") {
        let values: Vec<&str> = font_range.split(":").collect();

        let num_of_values = values.len();

        println!("num_of_values: {}", num_of_values);

        match num_of_values {
            1 => {
                println!("wrong format for font range: '{}', use colon ':' to separate values, ex.: '12:16'.", font_range);
            },
            2 => {
                let min_value = values[0].parse::<u8>();
                let max_value = values[1].parse::<u8>();

                match (min_value, max_value) {
                    (Ok(min_value), Ok(max_value)) => {
                        println!("font range values: {} - {}", min_value, max_value);
                        result.font_size_min = min_value;
                        result.font_size_max = max_value;
                    },
                    _ => {
                        println!("wrong format for font range: '{}', two integer values are needed, ex.: '12:16'.", font_range);
                    }
                }
            },
            _ => {
                println!("wrong format for font range: '{}', only two values allowed.", font_range);
            }
        }
    }
    result
}

fn load_config(file_name: &str) -> Configuration {
    println!("Loading configuration file: {}", file_name);

    let mut result = default_config().clone();

    result
}
