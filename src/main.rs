use serde_json::{Result, Value};
use reqwest;

use std::env;
use std::fs;

// to get every line from file for various variables
fn get_from_file(filename: String) -> Vec<String>{
    let content = fs::read_to_string(filename).unwrap();

    let mut current_name = String::new();
    let mut api_names: Vec<String> = Vec::new();

    for c in content.chars(){
        if c == '\n' { 
            api_names.push(current_name);
            current_name = String::new();
        }
        else { current_name.push(c); }
    }

    api_names
}

// opens file current where remembered id of api is stored
fn init_current_api() -> i32{
    let proto_res: String = fs::read_to_string("current").unwrap();
    let res = proto_res.parse::<i32>().unwrap();

    res

}

// when changing api, checks if it exists in apiNames file
fn check_api_exists(name: &String, names: &Vec<String>) -> i32{
    let mut to_api: i32 = -1;
    let mut exists = false;

    for current_name in names{
        to_api += 1;
        if *name == *current_name{
            exists = true;
            break;
        }
    }

    if exists { to_api }
    else { -1 }
}

// changing and saving new apiId
fn change_api(name: &String, names: Vec<String>){
    let new_api: i32 = check_api_exists(name, &names);
    if new_api != -1{
        save_new_api(new_api, names);
    }
}

fn save_new_api(id: i32, names: Vec<String>){
    let res = fs::write("current", id.to_string());
}

// connecting, getting and parsing resulting json
fn connect_to_api(city: &String, id: usize, key: &String, format: &String){

    let mut res_format: String = String::new();
    let mut proc: bool = false;

    for c in format.chars(){
        if proc { 
            if c == 'C' {res_format.push_str(city); }
            else if c == 'K' {res_format.push_str(key);} 
            proc = false;
        }
        else if c == '{' { proc = true; }
        else if c != '}' { res_format.push(c); }
    }

    //println!("{:?}", resFormat);

    let resp = reqwest::blocking::get(&res_format).unwrap().text().unwrap();
    let v: serde_json::Value = serde_json::from_str(&resp).unwrap();

    struct CurrentWeather{
        temp: String,
        weather: String,
    }
    impl Default for CurrentWeather{
        fn default() -> CurrentWeather{
            CurrentWeather{temp: "".to_owned(), 
            weather: "".to_owned()}
        }
    }
    let mut cw = CurrentWeather::default();

    // when adding new provider this match id
    // is the only thing that is nessecery 
    // to edit since json's that you recive
    // are different for every provider
    match id{
        0 => {
            cw.temp = ((v["main"]["temp"].as_f64().unwrap() as i64) - 273).to_string();
            cw.weather = v["weather"][0]["description"].to_string();
        },
        1 => {
            cw.temp = v["current"]["temp_c"].to_string();
            cw.weather = v["current"]["condition"]["text"].to_string();
        },
        _ => {println!("Unexpected Error");},
    }
    println!("In {} it's {} and {} degrees Celcius", city.to_owned(), 
    cw.weather, cw.temp);
}

fn parse_args() -> Vec<String>{
    let mut arg_vec: Vec<String> = Vec::new();

    for arg in env::args(){
        arg_vec.push(arg);
    }

    arg_vec
}

// result of list command
fn give_all_names(api_names: &Vec<String>){
    for name in api_names{
        println!("{}", name);
    }
}

fn give_all_cmd(){
    println!(
        "list - lists all avalible api\nconfigure <apiName> - change to different api\nget <cityname>      - get current weather in the city <cityname>\n"
    )
}

fn main() {
    let arg_vec: Vec<String> = parse_args();

    let api_names:   Vec<String> = get_from_file("apiNames".to_string());
    if arg_vec.len() == 2{
        if      &arg_vec[1] == "list" { give_all_names(&api_names); }
        else if &arg_vec[1] == "help" { give_all_cmd(); }
    }
    else if arg_vec.len() == 3 {
        let task = &arg_vec[1];
        let val =  &arg_vec[2];

        let api_keys:    Vec<String> = get_from_file("keys".to_string());
        let api_formats: Vec<String> = get_from_file("format".to_string());
        let current_api = init_current_api() as usize;

        let current_key = &api_keys[current_api];
        let current_format = &api_formats[current_api];

        if *task == "configure" { change_api(val, api_names); }
        else if *task == "get"{ connect_to_api(val, current_api, current_key, current_format); }
        else{ println!("Invalid arguments"); }
    }
    else { println!("Invalid arguments") };
}

