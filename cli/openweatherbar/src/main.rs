extern crate clap;
extern crate reqwest;
extern crate serde_json;

use std::fmt;
use std::env::{ var };
use std::path::{ Path };
use std::fs::{ read_to_string, write };
use std::process::{ exit };
use std::convert::{ TryFrom };
use std::time::{ Duration };
use std::thread::{ sleep };

use serde_json::{ Value };

static API: &'static str = "https://api.openweathermap.org/data/2.5";

enum Units {
    Metric,
    Imperial,
}

impl From<&str> for Units {
    fn from(v: &str) -> Self {
        // Note -- Assumes that `v` has already been validated by clap.
        let lower = v.to_lowercase();
        match lower.as_str() {
            "metric"   => Units::Metric,
            "imperial" => Units::Imperial,
            _ => panic!(),
        }
    }
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Units::Metric   => "Metric",
            Units::Imperial => "Imperial",
        })
    }
}

#[derive(Debug)]
struct BasicInfo {
    pub temp: f64,
    pub feels_like: f64,
    pub description: String,
    pub icon: String,
}

impl TryFrom<Value> for BasicInfo {
    type Error = ();
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let mut to_return = BasicInfo {
            temp: 0.0,
            feels_like: 0.0,
            description: String::new(),
            icon: String::new(),
        };
        match v {
            Value::Object(v) => {
                match v.get("main") {
                    Some(main) => match main {
                        Value::Object(main) => {
                            match main.get("temp") {
                                Some(temp) => match temp {
                                    Value::Number(temp) => to_return.temp = temp.as_f64().unwrap(),
                                    _ => return Err(()),
                                }
                                None => return Err(()),
                            }
                            match main.get("feels_like") {
                                Some(feels_like) => match feels_like { 
                                    Value::Number(feels_like) => to_return.feels_like = feels_like.as_f64().unwrap(),
                                    _ => return Err(()),
                                }
                                None => return Err(()),
                            }
                        }
                        _ => return Err(()),
                    }
                    None => return Err(()),
                }
                match v.get("weather") {
                    Some(weather) => match weather {
                        Value::Array(weather) => match weather.get(0) {
                            Some(weather) => match weather {
                                Value::Object(weather) => {
                                    match weather.get("icon") {
                                        Some(icon) => match icon {
                                            Value::String(icon) => to_return.icon = icon.clone(),
                                            _ => return Err(()),
                                        }
                                        None => return Err(()),
                                    }
                                    match weather.get("description") {
                                        Some(description) => match description {
                                            Value::String(description) => to_return.description = description.clone(),
                                            _ => return Err(()),
                                        }
                                        None => return Err(()),
                                    }
                                }
                                _ => return Err(()),
                            }
                            None => return Err(()),
                        }
                        _ => return Err(()),
                    }
                    None => return Err(()),
                }
            }
            _ => return Err(()),
        }
        Ok(to_return)
    }
}

fn get_icon(i: &str) -> &'static str {
    match i {
        "01d" => "󰖙",
        "01n" => "",
        "02d" => "",
        "02n" => "",
        "03d"|"03n" => "",
        "04d"|"04n" => "",
        "09d" => "",
        "09n" => "",
        "10d" => "",
        "10n" => "",
        "11d" => "",
        "11n" => "",
        "13d" => "",
        "13n" => "",
        "50d" => "",
        "50n" => "",
        _     =>"",
    }
}

fn error<E: fmt::Display>(e: E) -> ! {
    match write("/tmp/openweatherbar-error", format!("{}", e)) {
        Ok(_) => (),
        Err(_) => (),
    }
    print!("󰀦");
    exit(1);
}

fn main() {
    // Power arg parsing and whatnot with clap!
    let matches = clap::App::new("OpenWeatherBar")
        .version("v0.1.0")
        .author("Jack Johannesen <jack@insertdomain.name>")
        .about("Provides the same output as openweathermap-fullfeatured, with extra customizability and speed.")
        .arg(clap::Arg::with_name("key")
             .short("k")
             .long("key")
             .value_name("API_KEY")
             .help("Sets the API key to use when accessing the OpenWeather API. If not set, it will try and fallback to API key from ~/.config/api-keys/openweather.")
             .takes_value(true))
        .arg(clap::Arg::with_name("city")
            .short("c")
            .long("city")
            .value_name("CITY")
            .help("The city to use when accessing the OpenWeather API. If not set, it will try and fallback to ~/.config/location.")
            .takes_value(true))
        .arg(clap::Arg::with_name("units")
            .short("u")
            .long("units")
            .value_name("UNITS")
            .help("The type of units to use. Accepts either \"Metric\" or \"Imperial\".")
            .takes_value(true)
            .validator(|v| -> Result<(), String> {
                let lower = v.to_lowercase();
                if lower == "metric"
                || lower == "imperial" {
                    Ok(())
                } else {
                    Err(format!("Expected either \"Metric\" or \"Imperial\", got \"{}\"", v))
                }
            }))
        .arg(clap::Arg::with_name("symbol")
            .short("s")
            .long("symbol")
            .value_name("SYMBOL")
            .help("The symbol to use as a suffix after the temperature values. Ex: \"°\".")
            .takes_value(true))
        .arg(clap::Arg::with_name("tail")
            .short("-t")
            .long("tail")
            .help("Start the tool in tail mode. This also enables waiting for network related daemons to start.")
            .takes_value(false))
        .get_matches();

    // Get values from clap.

    let home = Path::new("/home/")
        .join(var("USER").unwrap_or(String::new()));
    
    let symbol = match matches.value_of("symbol") {
        Some(v) => String::from(v),
        None => String::from("°"),
    };

    let key = match matches.value_of("key") {
        Some(v) => String::from(v),
        None    => match read_to_string(home.join(".config/api-keys/openweather")) {
            Ok(v)  => v,
            Err(e) => error(e),
        },
    };

    let city = match matches.value_of("city") {
        Some(v) => String::from(v),
        None    => match read_to_string(home.join(".config/location")) {
            Ok(v)  => v,
            Err(e) => error(e),
        }
    };

    let units = match matches.value_of("units") {
        Some(v) => Units::from(v),
        None    => Units::Metric,
    };

    let tail = matches.is_present("tail");

    if !tail {
        update(&symbol, &key, &city, &units);
    } else {
        // Wait for networking daemons to start.
        loop {
            match reqwest::blocking::get("http://www.example.com") {
                Ok(_) => break,
                Err(_) => sleep(Duration::from_millis(1000 * 20)),
            }
        }
        loop {
            update(&symbol, &key, &city, &units);
            sleep(Duration::from_millis(1000 * 60 * 10));
        }
    }
}

fn update(symbol: &str, key: &str, city: &str, units: &Units) {
    let none_err = "Unwrapped None value.";

    // Make API requests.

    let current = {
        let try_current = reqwest::blocking::get(&format!("{api}/weather?appid={key}&q={city}&units={units}",
            api   = API,
            key   = key,
            city  = city,
            units = units));
        match try_current {
            Ok(current) => {
                match current.json::<Value>() {
                    Ok(json) => match BasicInfo::try_from(json) {
                        Ok(current) => current,
                        Err(_) => error("Deserialization error."),
                    }
                    Err(e) => error(Box::new(e)),
                }
            }
            Err(e) => error(Box::new(e)),
        }
    };

    let forecast = {
        let try_forecast = reqwest::blocking::get(&format!("{api}/forecast?appid={key}&q={city}&units={units}&cnt=1",
            api   = API,
            key   = key,
            city  = city,
            units = units));
        match try_forecast {
            Ok(current) => {
                match current.json::<Value>() {
                    Ok(json) => match json {
                        Value::Object(obj) => match obj.get("list") {
                            Some(list) => match list {
                                Value::Array(list) => match list.get(0) {
                                    Some(obj) => match BasicInfo::try_from(obj.clone()) {
                                        Ok(forecast) => forecast,
                                        Err(_) => error("Deserialization error."),
                                    }
                                    None => error(none_err),
                                }
                                _ => error("Expected Value::Array."),
                            }
                            None => error(none_err),
                        },
                        _ => error("Expected Value::Object."),
                    },
                    Err(e)   => error(e),
                }
            }
            Err(e) => error(e),
        }
    };

    let trend =
        if current.temp.round() > forecast.temp.round() { "" }
        else if current.temp.round() < forecast.temp.round() { "" }
        else { "" };

    // Print out the fetched info.
    println!("{} {}{} {} {} {}{}",
        get_icon(&current.icon),
        current.temp.round() as i32,
        symbol,
        trend,
        get_icon(&forecast.icon),
        forecast.temp.round() as i32,
        symbol);

    // Write to a file so other utils can read the output.
    match write("/tmp/openweatherbar",
        &format!(r#"{{
"current": {{ "temp": {c_temp}, "icon": "{c_icon}", "feels_like": {c_feels_like}, "description": "{c_description}" }},
"forecast": {{ "temp": {f_temp}, "icon": "{f_icon}", "feels_like": {f_feels_like}, "description": "{f_description}" }}
}}"#,
        c_temp = current.temp,
        c_icon = current.icon,
        c_feels_like = current.feels_like,
        c_description = current.description,
        f_temp = forecast.temp,
        f_icon = forecast.icon,
        f_feels_like = forecast.feels_like,
        f_description = forecast.description)) {
        Ok(_) => (),
        Err(_) => (),
    }
}
