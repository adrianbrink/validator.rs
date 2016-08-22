#![feature(custom_attribute, custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate curl;
#[macro_use]
extern crate diesel;
extern crate serde;
extern crate serde_json;
extern crate validator;
extern crate schema;

// use self::validator::models::*;
use curl::easy::Easy;
use diesel::pg::PgConnection;
use self::diesel::prelude::*;
use self::validator::schema::*;
use std::collections::HashMap;
use std::str;

// This should reside in models.rs but it causes an error with custom_derive for serde.
#[derive(Serialize, Deserialize, Qeuryable, Debug, PartialEq)]
#[insertable_into(summoners)]
pub struct Summoner {
    pub id: u64,
    pub name: String,
    pub profileIconId: u64,
    pub summonerLevel: u64,
    pub revisionDate: u64,
}

pub fn create_summoner(conn: &PgConnection, summoner: Summoner) {
    use schema::summoners;

    diesel::insert(&summoner).into(summoners::table)
        .get_result(conn)
        .expect("Error saving new summoner.")
}

fn main() {
    println!("### It works ###");

    let mut handle = Easy::new();
    handle.url("https://na.api.pvp.net/api/lol/na/v1.4/summoner/by-name/RiotSchmick?api_key=RGAPI-11577715-A924-4825-A831-FF7038985625").unwrap();
    handle.write_function(|data| {
        let json_string = str::from_utf8(data).unwrap();
        let deserialized_summoner: HashMap<String, Summoner> = serde_json::from_str(json_string).unwrap();
        println!("{:?}", deserialized_summoner);
        Ok(data.len())
    }).unwrap();
    handle.perform().unwrap();
}

/*
let mut data = Vec::new();
let mut handle = Easy::new();
handle.url("https://na.api.pvp.net/api/lol/na/v1.4/summoner/by-name/RiotSchmick?api_key=RGAPI-11577715-A924-4825-A831-FF7038985625").unwrap();
{
    let mut transfer = handle.transfer();
    transfer.write_function(|new_data| {
        data.extend_from_slice(new_data);
        Ok(new_data.len())
    }).unwrap();
    transfer.perform().unwrap();
}
println!("{:?}", data);
*/
