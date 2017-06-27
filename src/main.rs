extern crate darwin_rs;
extern crate clap;
extern crate font_loader;
extern crate yaml_rust;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate chrono;

// use darwin_rs::{Individual, SimulationBuilder, Population, PopulationBuilder};

// External modules
use font_loader::system_fonts;
use simplelog::{Config, TermLogger, WriteLogger, LogLevelFilter};
use log::{LogLevel};
use chrono::Local;

// System modules
use std::fs::OpenOptions;

// Internal modules
mod config;
use config::{create_config};


fn main() {
    // Init logger
    let dt = Local::now();
    let log_filename = dt.format("character_twister_%Y_%m_%d.log").to_string();

    let log_config = Config{
        time: Some(LogLevel::Warn),
        level: Some(LogLevel::Warn),
        target: Some(LogLevel::Warn),
        location: Some(LogLevel::Warn)
    };

    if let Ok(file) = OpenOptions::new().append(true).create(true).open(&log_filename) {
        let _ = WriteLogger::init(LogLevelFilter::Info, log_config, file);
        info!("Log file '{}' created succesfully", &log_filename);
    } else {
        // Log file could not be created, use stdout instead
        let _ = TermLogger::init(LogLevelFilter::Info, log_config);
        warn!("Could not open log fle: '{}', using sdtout instead!", &log_filename);
    }

    let config = create_config();


    let sysfonts = system_fonts::query_all();

    for font in &sysfonts {
        println!("font: {}", font);
    }


/*
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
*/
}
