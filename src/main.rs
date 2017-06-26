extern crate darwin_rs;
extern crate clap;

use darwin_rs::{Individual, SimulationBuilder, Population, PopulationBuilder};

mod config;

use config::{create_config};


fn main() {
    let config = create_config();

}
