#![feature(use_extern_macros)]

use serde_derive::*;

#[derive(Debug,Deserialize)]
crate struct Agency {
    #[serde(rename="agency_id")] id: i32,
    #[serde(rename="agency_name")] name: String,
    #[serde(rename="agency_url")] url: String,
    #[serde(rename="agency_timezone")] timezone: String,
    #[serde(rename="agency_lang")] language: String,
    #[serde(rename="agency_phone")] phone: String
}

#[derive(Debug,Deserialize)]
crate struct Calendar {

}

#[derive(Debug,Deserialize)]
crate struct CalendarDate {

}

#[derive(Debug,Deserialize)]
crate struct Transfer {

}

#[derive(Debug,Deserialize)]
crate struct Stop {
    #[serde(rename="stop_id")] id: String,
    #[serde(rename="stop_code")] code: String,
    #[serde(rename="stop_name")] name: String,
    #[serde(rename="stop_desc")] desc: String,
    #[serde(rename="stop_lat")] lat: String,
    #[serde(rename="stop_lon")] lng: String,
    #[serde(rename="location_type")] location_type: String,
    #[serde(rename="parent_station")] parent_station: String,
}

#[derive(Debug,Deserialize)]
crate struct Route {
    id: i32,
    agency: Agency,
    short_name: String,
    long_name: String,
    color: String,
    text_color: String,
}

#[derive(Debug,Deserialize)]
crate struct Trip {
    route: Route,
}
