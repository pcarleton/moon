extern crate chrono;

use chrono::prelude::*;

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

// This accounts for leap years
const DAYS_PER_YEAR: f64 = 365.25;

// This is the mean time between New Moons.  It varies between
// 29.26 and 29.80 due to the perturbing effects of the sun.
const MEAN_LUNATION_PERIOD_DAYS: f64 = 29.530588;

const LUNATIONS_PER_YEAR: f64 = DAYS_PER_YEAR / MEAN_LUNATION_PERIOD_DAYS;

enum Phase {
    NewMoon,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    FullMoon,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
}

fn get_moonicode(phase: &Phase) -> String {
    match phase {
        Phase::NewMoon => NEW_MOON_FACE.to_string(),
        Phase::WaxingCrescent => WAXING_CRESCENT.to_string(),
        Phase::FirstQuarter => FIRST_QUARTER.to_string(),
        Phase::WaxingGibbous => WAXING_GIBBOUS.to_string(),
        Phase::FullMoon => FULL_MOON_FACE.to_string(),
        Phase::WaningGibbous => WANING_GIBBOUS.to_string(),
        Phase::LastQuarter => LAST_QUARTER.to_string(),
        Phase::WaningCrescent => WANING_CRESCENT.to_string(),
    }
}

fn fractional_year(dt: DateTime<Local>) -> f64 {
    let day_component = dt.date().ordinal0() as f64 / DAYS_PER_YEAR;
    dt.year() as f64 + day_component
}

fn simple_phase_decimal(dt: DateTime<Local>) -> f64 {
    // 2000/01/06 18:14 UTC was a new moon, so it serves as the reference year
    // This could be any decimal year that had a new moon
    let lun0_utc = Utc.ymd(2000, 1, 6).and_hms(18, 14, 0);
    let lun0_local: DateTime<Local>  = DateTime::from(lun0_utc);

    let base_fy = fractional_year(lun0_local);
    let target_fy = fractional_year(dt);

    let lun_number = (target_fy - base_fy)*LUNATIONS_PER_YEAR;

    // Get just the decimal part since that indicates how
    // far we are in the current lunation
    let lun_decimal = lun_number - lun_number.floor();

    lun_decimal
}

fn calculate_moon_phase(dt: DateTime<Local>) -> Phase {
    let phase_decimal = simple_phase_decimal(dt);

    // Consider a "primary" phase (i.e. not waxing/waning) to
    // be in effect if we are within 12 hours.
    let buffer =
        12.0 / 
        (MEAN_LUNATION_PERIOD_DAYS * 24.0) // Length of a lunation in hours
    ;

    if phase_decimal < (0.0 + buffer) {
        Phase::NewMoon
    } else if phase_decimal < (0.25 - buffer) {
        Phase::WaxingCrescent
    } else if phase_decimal < (0.25 + buffer) {
        Phase::FirstQuarter
    } else if phase_decimal < (0.5 - buffer) {
        Phase::WaxingGibbous
    } else if phase_decimal < (0.5 + buffer) {
        Phase::FullMoon
    } else if phase_decimal < (0.75 - buffer) {
        Phase::WaningGibbous
    } else if phase_decimal < (0.75 + buffer) {
        Phase::LastQuarter
    } else if phase_decimal < (1.0 - buffer) {
        Phase::WaningCrescent
    } else {
        Phase::NewMoon
    }
}

fn print_phase_example() {
    for d in 1..30 { 
        let dt: DateTime<Local> = Local.ymd(2019, 11, d).and_hms(0, 0, 0);
        let phase = get_moonicode(&calculate_moon_phase(dt));
        println!("{} {}", dt.format("%Y/%m/%d"), phase)
    }
}


fn main() {
    let phase = get_moonicode(&calculate_moon_phase(Local::now()));
    println!("{}", phase)
}

