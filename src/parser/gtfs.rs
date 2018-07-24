use std::fs;
use types::gtfs::*;
use std::error::Error;
use std::fs::DirEntry;
use types::gtfs::*;
use std::prelude::v1::None;
use std::result::Result;
extern crate csv;


/// Returns true if the parse was successful
pub fn parse(path: &str)  {
    load_files(path);
}
/*
 * Check if the files from given path are
 * included on the valid GTFS file list.
**/
fn load_files(path: &str) {
    let list = fs::read_dir(path).unwrap();

    for file in list {
        let file = file.unwrap();
        let file_name = &file.file_name();
        let file_name = file_name.to_str();

        if is_supported_file(file_name.unwrap()) {
            info!("GTFS found: {}", file_name.unwrap());
            transform_to_native_type(file);
        } else {
            warn!("Invalid GTFS file: {}", file_name.unwrap())
        }
    }
}

#[derive(Debug)]
enum GTFS {
    Agencies(Agency),
    Stops(Stop),
    Nothing(())
}

fn transform_to_native_type(file: DirEntry) -> Result<(), Box<Error>> {
    let file = file;
    // let file_name = &;
    let mut reader = csv::ReaderBuilder::new()
    .flexible(true).has_headers(true).from_path(&file.path())?;

    for result in reader.deserialize() {
        let record : Stop = result?;
        // let record = enum_for_supported(&file.file_name().to_str().unwrap(), result);

        // info!("{:?}", record);
    }

    Ok(())
}

// fn enum_for_supported(file_name: &str, result: Result<csv::Result, csv::Error>) -> Result<GTFS, csv::Error> {
//     let record = match file_name {
//         "stops.txt" => GTFS::Stops(Stop::from(result?)),
//         "agency.txt" => GTFS::Agencies(Agency::from(result?)),
//         _ => GTFS::Nothing(())
//     };
//     return Ok(record)
// }

fn is_supported_file(file_name: &str) -> bool {
    match file_name {
        "agency.txt" | "calendar.txt" | "calendar_dates.txt" | "routes.txt" |
        "shapes.txt" | "stops.txt" | "stop_times.txt" | "transfers.txt" |
        "trips.txt" => return true,
        _  => return false
    }
}

#[cfg(test)]
mod tests {
    use parser::gtfs::*;

    #[test]
    fn test_load_files() {
        assert_eq!((), load_files("./"))
    }

    #[test]
    fn test_parse() {
        assert_eq!((), parse("./"))
    }
}
