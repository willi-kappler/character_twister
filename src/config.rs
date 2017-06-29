// External modules
use clap::{Arg, App};
use toml;

// System modules
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Read;
use std::cmp;

#[derive(Clone, Debug, PartialEq)]
pub struct Configuration {
    pub input_path: String,
    pub font_size_min: i8,
    pub font_size_max: i8,
    pub font_name: String,

}

#[derive(Deserialize)]
struct TOMLConfig {
    font: Option<TOMLFont>,
    input_path: Option<String>
}

#[derive(Deserialize)]
struct TOMLFont {
    name: String,
    size_min: i8,
    size_max: i8
}

fn default_config() -> Configuration {
    Configuration {
        input_path: "./".to_string(),
        font_size_min: 8,
        font_size_max: 16,
        font_name: "FreeMono".to_string()
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
    let mut result = default_config();

    if let Some(config_file) = matches.value_of("config") {
        result = load_config(config_file);
    }

    // Command line parameter can overwrite configuration file settings
    if let Some(input_path) = matches.value_of("input") {
        result.input_path = input_path.to_string();
        info!("input path: {}", input_path);
    }

    if let Some(font_name) = matches.value_of("font_name") {
        result.font_name = font_name.to_string();
        info!("using font: {}", font_name);
    }

    if let Some(font_range) = matches.value_of("font_range") {
        let values: Vec<&str> = font_range.split(":").collect();

        let num_of_values = values.len();

        info!("num_of_values: {}", num_of_values);

        match num_of_values {
            1 => {
                warn!("wrong format for font range: '{}', use colon ':' to separate values, ex.: '12:16'.", font_range);
            },
            2 => {
                let left_value = values[0].parse::<i8>();
                let right_value = values[1].parse::<i8>();

                match (left_value, right_value) {
                    (Ok(left_value), Ok(right_value)) => {
                        if left_value < 4 || right_value < 4 {
                            warn!("values for font size should be at least 4: '{}'", font_range);
                        } else {
                            result.font_size_min = cmp::min(left_value, right_value);
                            result.font_size_max = cmp::max(left_value, right_value);
                            info!("font range values: {} - {}", result.font_size_min, result.font_size_max);
                        }
                    },
                    _ => {
                        warn!("wrong format for font range: '{}', two integer values are needed, ex.: '12:16'.", font_range);
                    }
                }
            },
            _ => {
                warn!("wrong format for font range: '{}', only one colon ':' is  allowed.", font_range);
            }
        }
    }
    result
}

fn load_config(filename: &str) -> Configuration {
    info!("Loading configuration file: {}", filename);

    if let Ok(file) = OpenOptions::new().read(true).open(filename) {
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();

        if let Ok(_) = buf_reader.read_to_string(&mut content) {
            return parse_config_file(&content);
        } else {
            // Content of configuration file could not be read, use default settings instead.
            warn!("Could not read configuration file '{}', using default settings", filename);
        }
    } else {
        // Configuration file could not be opened, use default settings instead.
        warn!("Could not open configuration file '{}', using default settings", filename);
    }
    default_config()
}

fn parse_config_file(content: &str) -> Configuration {
    let mut result = default_config();

    match toml::from_str::<TOMLConfig>(content) {
        Ok(config) => {
            if let Some(input_path) = config.input_path {
                result.input_path = input_path;
            }

            if let Some(font) = config.font {
                result.font_name = font.name;
                result.font_size_min = font.size_min;
                result.font_size_max = font.size_max;
            }
        },
        Err(e) => {
            warn!("TOML parse error: {}", e);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::{parse_config_file, default_config, Configuration};

    use logger::create_logger;

    #[test]
    fn parse_config_file1() {
        create_logger();

        let input = "";

        let expected_output = default_config();

        assert_eq!(parse_config_file(input), expected_output);
    }

    #[test]
    fn parse_config_file2() {
        create_logger();

        let input =
r#"
input_path = "scans"

[font]
name = "SomeFont"
size_min = 11
size_max = 19
"#;

        let expected_output = Configuration{
            input_path: "scans".to_string(),
            font_size_min: 11,
            font_size_max: 19,
            font_name: "SomeFont".to_string()
        };

        assert_eq!(parse_config_file(input), expected_output);
    }

    #[test]
    fn parse_config_file3() {
        create_logger();

        let input =
r#"
input_path = "scans"
"#;

        let expected_output = Configuration{
            input_path: "scans".to_string(),
            font_size_min: 8,
            font_size_max: 16,
            font_name: "FreeMono".to_string()
        };

        assert_eq!(parse_config_file(input), expected_output);
    }

    #[test]
    fn parse_config_file4() {
        create_logger();

        let input =
r#"
[font]
name = "SomeFont"
size_min = 11
size_max = 19
"#;

        let expected_output = Configuration{
            input_path: "./".to_string(),
            font_size_min: 11,
            font_size_max: 19,
            font_name: "SomeFont".to_string()
        };

        assert_eq!(parse_config_file(input), expected_output);
    }
}
