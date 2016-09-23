#[macro_use] extern crate quick_error;
extern crate darwin_rs;
extern crate clap;
extern crate yaml_rust;

mod config;

use darwin_rs::{Individual, SimulationBuilder, Population, PopulationBuilder, SimError};

use config::{Configuration, create_config};

fn main() {
    let config = create_config();



}
