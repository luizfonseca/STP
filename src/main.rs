#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate flexi_logger;

/// Usage:
/// ./stp {--agency=796} --gtfs_path=/path/gtfs/bvg --osm=berlin.osm --osm-url=apix
// extern crate geo;
//
// use geo::algorithm::haversine_distance::HaversineDistance;
// use geo::{Point};
mod parser;
mod types;
use parser::gtfs as gtfs_parser;
use flexi_logger::Logger;

fn main() {

    Logger::with_str("info,warn,error,trace")
        .print_message()
        .start().unwrap();

    let path = "/Users/luizfonseca/Downloads/bvg";
    gtfs_parser::parse(path);
}
