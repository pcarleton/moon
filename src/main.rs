
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate astro;

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
    fracillum: Option<String>,
    curphase: Option<String>
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

    let phase_str = match r.curphase { 
        Some(cur_phase) => cur_phase,
        None => r.closestphase.phase
    };


    match get_moonicode(&phase_str) {
        Ok(phase) => {
            println!("{}", phase); 
            write_cache(&phase)
        },
        // TODO: Propagate this error...
        Err(message) => println!("Don't know phase: {}", phase_str)
    }
    Ok(())
}

fn dt_to_julian(dt: DateTime<Local>) -> astro::time::Date {
    let now: astro::time::Date = astro::time::Date{
        year: dt.year() as i16,
        month: dt.month() as u8,
        decimal_day: dt.day() as f64,
        cal_type: astro::time::CalType::Gregorian,
    };

    return now;
}

fn dt_for_phase<'a>(dt: DateTime<Local>, phase: &astro::lunar::Phase) -> Result<DateTime<Local>, &'a str> {
    let dtj = dt_to_julian(dt);
    let phase_jd = astro::lunar::time_of_phase(&dtj, &phase);
    match astro::time::date_frm_julian_day(phase_jd) {
        Err(why) => {
            Err(why)
        }
        Ok((year, month, day)) => {
            let secs_from_midnight = (day - day.floor())*(24.0 * 60.0 * 60.0);
            let time = NaiveTime::from_num_seconds_from_midnight(secs_from_midnight as u32, 0);
            let utc = Utc.ymd(year.into(), month.into(), day as u32).and_time(time).unwrap();
            let local: DateTime<Local> = DateTime::from(utc);
            Ok(local)
        }
    }
}

fn phase_to_str(phase: &astro::lunar::Phase) -> &str {
    match phase {
        astro::lunar::Phase::New => "New Moon",
        astro::lunar::Phase::First=> "First Quarter",
        astro::lunar::Phase::Full=> "Full Moon",
        astro::lunar::Phase::Last=> "Last Quarter",
    }
}

fn print_phase(dt: DateTime<Local>, phase: &astro::lunar::Phase) {
    match dt_for_phase(dt, phase) {
        Err(why) => println!("Trouble with phase: {:?}", why),
        Ok(phase_dt) => {
            println!("{} {}", phase_to_str(phase), phase_dt.format("%Y/%m/%d").to_string());
        }
    }
}

fn next_phase(phase: &astro::lunar::Phase) -> &str {
    match phase {
        astro::lunar::Phase::New => "Waxing Crescent",
        astro::lunar::Phase::First=> "Waxing Gibbous",
        astro::lunar::Phase::Full=> "Waning Gibbous",
        astro::lunar::Phase::Last=> "Waning Crescent",
    }
}

fn prev_phase(phase: &astro::lunar::Phase) -> &str {
    match phase {
        astro::lunar::Phase::First=> "Waxing Crescent",
        astro::lunar::Phase::Full => "Waxing Gibbous",
        astro::lunar::Phase::Last => "Waning Gibbous",
        astro::lunar::Phase::New => "Waning Crescent",
    }
}

// TODO: find the nearest phase
// Get the list of phases (maybe jump ahead a couple days from the "Last" to get the next New Moon
// With that last, start from the New moon
// if its equal on date, then use it, otherwise, continue
// if the next date is greater, then use "Waxing Crescent"
// So I need a map of "primary" phase to the "following phase" so I can do a lookup.




fn near_phases(local: DateTime<Local>) {
    let phases: Vec<&astro::lunar::Phase> = vec![
        &astro::lunar::Phase::New,
        &astro::lunar::Phase::First,
        &astro::lunar::Phase::Full,
        &astro::lunar::Phase::Last,
    ];
    let phase_dates: Vec<(&astro::lunar::Phase, DateTime<Local>)> = phases.iter()
        .map(|&p| (p, dt_for_phase(local, p).unwrap())).collect();

    for (p, d)  in phase_dates {
        println!("{} {}", phase_to_str(p), d.format("%Y/%m/%d").to_string());
    }
}

fn simple_phase_angle(dt: DateTime<Local>) -> f64 {
    let year_component = dt.year() - 1900;
    let day_component = dt.date().ordinal0() as f64 / 365.0;
    let year_fraction = year_component as f64 + day_component;
    let k = year_fraction*12.3685;

    k - k.floor()
}

fn simple_phase(dt: DateTime<Local>) -> &'static str {
    let phase_angle = simple_phase_angle(dt);

    let buffer = 0.01;

    if phase_angle < (0.0 + buffer) {
        "New Moon"
    } else if phase_angle < (0.25 - buffer) {
        "Waxing Crescent"
    } else if phase_angle < (0.25 + buffer) {
        "First Quarter"
    } else if phase_angle < (0.5 - buffer) {
        "Waxing Gibbous"
    } else if phase_angle < (0.5 + buffer) {
        "Full Moon"
    } else if phase_angle < (0.75 - buffer) {
        "Waning Gibbous"
    } else if phase_angle < (0.75 + buffer) {
        "Last Quarter"
    } else if phase_angle < (1.0 - buffer) {
        "Waning Crescent"
    } else {
        "New Moon"
    }
}

fn nearest_phase(dt: DateTime<Local>) -> &'static str {
    let phases: Vec<&astro::lunar::Phase> = vec![
        &astro::lunar::Phase::New,
        &astro::lunar::Phase::First,
        &astro::lunar::Phase::Full,
        &astro::lunar::Phase::Last,
    ];
    let phase_dates: Vec<(&astro::lunar::Phase, Date<Local>)> = phases.iter()
        .map(|&p| (p, dt_for_phase(dt, p).unwrap().date())).collect();

    let cur_day = dt.date();
    for (p, d)  in phase_dates {
        if d > cur_day {
            //println!("d ({}) > cur_day ({})", d, cur_day);
            return prev_phase(p);
        }
        if d == cur_day {
            //println!("d ({}) == cur_day ({})", d, cur_day);
            return phase_to_str(p);
        }
    }
    next_phase(&astro::lunar::Phase::Last)
}

fn print_phase_comparison(dt: DateTime<Local>) {
    let p1 = get_moonicode(nearest_phase(dt)).unwrap();
    let p2 = get_moonicode(simple_phase(dt)).unwrap();
    let pa = simple_phase_angle(dt);
    //near_phases(dt);
    println!("{} {} {} {:.3}", dt.format("%Y/%m/%d"), p1, p2, pa)
}


fn main() {
    for d in 1..30 { 
        let local: DateTime<Local> = Local.ymd(2019, 11, d).and_hms(0, 0, 0);
        print_phase_comparison(local);
    }

    near_phases(Local.ymd(2019, 11, 20).and_hms(0, 0, 0));
    //let local: DateTime<Local> = Local.ymd(2019, 11, 21).and_hms(0, 0, 0);
    //let local: DateTime<Local> = Local::now();
}

