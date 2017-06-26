extern crate darwin_rs;
extern crate clap;
extern crate font_loader;
extern crate yaml_rust;

// use darwin_rs::{Individual, SimulationBuilder, Population, PopulationBuilder};

mod config;

use config::{create_config};
use font_loader::system_fonts;

fn main() {
    let config = create_config();

/*
    let sysfonts = system_fonts::query_all();

    for font in &sysfonts {
        println!("font: {}", font);
    }
*/

    let font_property = system_fonts::FontPropertyBuilder::new().
        family(&config.font_name).build();

    let font = system_fonts::get(&font_property);

    if let Some((font_data, res)) = font {
        println!("Font found");
        println!("Font size: {}", font_data.len());
        println!("res: {}", res);
    } else {
        println!("Could find / load font: {}", &config.font_name);
    }
}
