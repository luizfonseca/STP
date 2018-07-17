extern crate csv;
use std::fs;
use types::gtfs::*;
use std::error::Error;

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
        let fullpath = &file.path();
        let file_name = &file.file_name();
        let file_name = file_name.to_str();

        if is_supported_file(file_name.unwrap()) {
            info!("GTFS found: {}", file_name.unwrap());
            transform_to_native_type(fullpath.to_str())
        } else {
            warn!("Invalid GTFS file: {}", file_name.unwrap())
        }
    }
}

fn transform_to_native_type(path: Option<&str>)  {
    let reader = csv::Reader::from_path(path.unwrap());
    for result in reader.iter() {
        info!("Transform: {:?}", result)
    }
}

fn transform_agency(csv: csv::ReaderBuilder) -> Result<(), Box<Error>> {
    return Ok(());
}


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
