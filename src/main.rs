
use std::error::Error;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
/* 
 * {
"error":false,
"apiversion":"2.1.0",
"year":2018,
"month":6,
"day":10,
"dayofweek":"Sunday",
"datechanged":false,
"state":"CA",
"city":"San Francisco",
"lon":-122.44,
"lat":37.76,
"county":"San Francisco",
"tz":-8,
"isdst":"yes",

"sundata":[
            {"phen":"BC", "time":"5:16 a.m. DT"},
            {"phen":"R", "time":"5:48 a.m. DT"},
            {"phen":"U", "time":"1:09 p.m. DT"},
            {"phen":"S", "time":"8:31 p.m. DT"},
            {"phen":"EC", "time":"9:02 p.m. DT"}],

"moondata":[
            {"phen":"R", "time":"3:44 a.m. DT"},
            {"phen":"U", "time":"10:22 a.m. DT"},
            {"phen":"S", "time":"5:08 p.m. DT"}],

"closestphase":{"phase":"New Moon","date":"June 13, 2018","time":"12:43 p.m. DT"},
"fracillum":"13%",
"curphase":"Waning Crescent"
* Connection #0 to host api.usno.navy.mil left intact
}
*/
#[derive(Serialize, Deserialize)]
struct PhenData {
    phen: String,
    time: String
}

#[derive(Serialize, Deserialize)]
struct PhaseData {
    phase: String,
    date: String,
    time: String
}

#[derive(Serialize, Deserialize)]
struct SunMoonResponse {
    error: bool,
    apiversion: String,
    year: u32,
    month: u32,
    day: u32,
    dayofweek: String,
    datechanged: bool,
    state: String,
    city: String,
    lon: f32,
    lat: f32,
    county: String,
    tz: i32,
    isdst: String,
    sundata: Vec<PhenData>,
    moondata: Vec<PhenData>,
    closestphase: PhaseData,
    fracillum: String,
    curphase: String
}

static NEW_MOON: &'static str = "\u{1F311}";
static WAXING_CRESCENT: &'static str = "\u{1F312}";
static FIRST_QUARTER: &'static str = "\u{1F313}";
static WAXING_GIBBOUS: &'static str = "\u{1F314}";
static FULL_MOON: &'static str = "\u{1F315}";
static WANING_GIBBOUS: &'static str = "\u{1F316}";
static LAST_QUARTER: &'static str = "\u{1F317}";
static WANING_CRESCENT: &'static str = "\u{1F318}";

static CRESCENT: &'static str = "\u{1F319}";
static NEW_MOON_FACE: &'static str = "\u{1F31A}";
static FIRST_QUARTER_FACE: &'static str = "\u{1F31B}";
static LAST_QUARTER_FACE: &'static str = "\u{1F31C}";
static FULL_MOON_FACE: &'static str = "\u{1F31D}";
static SUN_FACE: &'static str = "\u{1F31E}";
static GLOWING_STAR: &'static str = "\u{1F31F}";


static ONEDAY_URL_BASE: &'static str = "http://api.usno.navy.mil/rstt/oneday";

static CACHE_PATH: &'static str = "/tmp/moon";

fn get_today() -> String {
    let local: DateTime<Local> = Local::now();
    return local.format("%m/%d/%Y").to_string();
}

fn get_moonicode(phase: &str) -> Result<String, String> {
    match phase.as_ref() {
        "New Moon" => Ok(NEW_MOON_FACE.to_string()),
        "Waxing Crescent" => Ok(WAXING_CRESCENT.to_string()),
        "First Quarter" => Ok(FIRST_QUARTER.to_string()),
        "Waxing Gibbous" => Ok(WAXING_GIBBOUS.to_string()),
        "Full Moon" => Ok(FULL_MOON_FACE.to_string()),
        "Waning Gibbous" => Ok(WANING_GIBBOUS.to_string()),
        "Last Quarter" => Ok(LAST_QUARTER.to_string()),
        "Waning Crescent" => Ok(WANING_CRESCENT.to_string()),
        _ =>  Err("Unknown phase".to_string())
    }
}

fn read_cache() -> Result<String, String> {
    let cache_path = Path::new(CACHE_PATH);

    if !cache_path.exists() {
        return Err("No such file".to_owned());
    }

    let mod_sys_time : std::time::SystemTime = cache_path.metadata().map_err(|e| e.to_string())?
        .modified().map_err(|e| e.to_string())?;
    
    let mod_unix_secs : u64 =  mod_sys_time.duration_since(std::time::UNIX_EPOCH).map_err(|e| e.to_string())?
        .as_secs();

    let mod_date = Local.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp(mod_unix_secs as i64, 0)).date();

    if mod_date != Local::today() {
        return Err("Stale cache".to_owned());
    }

    // Read file
    let mut f = File::open(cache_path).expect("file not found"); 
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    Ok(contents)
}

fn write_cache(phase: &str) {
    let mut file = File::create("/tmp/moon").unwrap();
    file.write_all(phase.as_bytes()).unwrap();
}

fn try_main() -> Result<(), Box<Error>> {
    
    match read_cache() {
        Ok(text) => {
            println!("{}", text);
            return Ok(());
        },
        Err(e) => {
            // Do nothing. Maybe print something debug wise?
        }
    }


    let mut target = String::new();
    target.push_str(ONEDAY_URL_BASE);
    target.push_str("?date=");
    target.push_str(&get_today());
    target.push_str("&loc=San Francisco, CA");
    let body = reqwest::get(target.as_str())?.text()?;

    // TODO: If parsing fails, print the actual body
    //println!("{}", target);
    //println!("{}", body);
    let r: SunMoonResponse = serde_json::from_str(&body)?;

    let phase = get_moonicode(&r.curphase);

    match phase {
        Ok(phase) => {
            println!("{}", phase); 
            write_cache(&phase)
        },
        // TODO: Propagate this error...
        Err(message) => println!("Don't know phase: {}", r.curphase)
    }
    Ok(())
}

fn main() {

    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
