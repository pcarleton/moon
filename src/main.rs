
use std::error::Error;
use std::process;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

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

fn get_moonicode(phase: &str) -> Result<String, String> {
    match phase.as_ref() {
        "Waxing Crescent" => Ok(WAXING_CRESCENT.to_string()),
        _ =>  Err("Unknown phase".to_string())

    }
}

fn try_main() -> Result<(), Box<Error>> {
    let mut target = String::new();
    target.push_str(ONEDAY_URL_BASE);
    target.push_str("?date=6/17/2018&loc=San Francisco, CA");
    let body = reqwest::get(target.as_str())?.text()?;

    let r: SunMoonResponse = serde_json::from_str(&body)?;

    match get_moonicode(&r.curphase) {
        Ok(phase) => println!("{}", phase),
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
