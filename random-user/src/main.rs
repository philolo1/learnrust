use reqwest;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    info: Info,
    results: Vec<ResultData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    page: u32,
    results: u32,
    seed: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultData {
    cell: String,
    dob: Dob,
    email: String,
    gender: String,
    location: Location,
    login: Login,
    name: Name,
    nat: String,
    phone: String,
    picture: Picture,
    registered: Registered,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dob {
    age: u32,
    date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Id {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    city: String,
    coordinates: Coordinates,
    country: String,
    postcode: String,
    state: String,
    street: Street,
    timezone: Timezone,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coordinates {
    latitude: String,
    longitude: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Street {
    name: String,
    number: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Timezone {
    description: String,
    offset: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Login {
    md5: String,
    password: String,
    salt: String,
    sha1: String,
    sha256: String,
    username: String,
    uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Name {
    first: String,
    last: String,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Picture {
    large: String,
    medium: String,
    thumbnail: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Registered {
    age: u32,
    date: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let str = reqwest::blocking::get("https://randomuser.me/api/")?.text()?;

    let res = match serde_json::from_str::<Response>(&str) {
        Ok(_) => println!("Ok"),
        Err(e) => {
            eprintln!("Error");
            println!("{}" , str);
            return Err(Box::new(e));
        }
    };

    println!("{:?}", res);
    Ok(())
}
