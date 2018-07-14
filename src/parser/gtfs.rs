extern crate csv;

use std::io;
use std::fs;

mod parser::gtfs {
    let supported_files : <Vec<String>> =
        vec!["agency", "stops", "calendar_dates",
            "calendar", "trips", "routes", "shapes",
            "transfers", "stop_times"];

    pub fn check_files_from(path: String) -> <Vec<String>> {
        let list = fs::read_dir(path).unwrap_or([]);

        for item in &list {
            println(item.path().unwrap().display());
        }
    }
}




#[cfg(test)]
mod tests {

    #[test]
    fn test_check_files_from() {
        assert_eq!(check_files_from("./"), "");
    }
}
