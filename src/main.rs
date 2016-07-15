extern crate chrono;

use chrono::duration::Duration;
use chrono::offset::local::Local;

fn main() {

    // TODO: clap-rs
    // TODO: -d/--delay=14 (mins)
    // TODO: -c/--cycle=90 (mins)
    // TODO: wakeup at 8:30pm (needs chomp parsers)

    let sleep_delay = 14;
    let sleep_cycle = 90;

    println!("Assume it takes {} to fall asleep.\n",
        Timerange::new(Duration::minutes(sleep_delay).num_seconds() as u64).print(10));

    println!("Wake up at:");

    // 6 sleep cycles
    for cycle in 1..7 {

        let sleep_length = sleep_delay + sleep_cycle * cycle;
        let wake_up = Local::now().naive_local() + Duration::minutes(sleep_length);
        let range = Timerange::new(Duration::minutes(sleep_length).num_seconds() as u64);

        println!("{} ({})", wake_up.format("%-l:%M %p"), range.print(10));

    }

}

struct Timerange {
    range: u64
}

impl Timerange {

    fn new(range: u64) -> Timerange {
        Timerange {
            range: range
        }
    }

    fn floor_time_unit(&self) -> (u64, u64, String) {

        let sec_per_minute: f64 = 60f64;
        let sec_per_hour: f64 = sec_per_minute * 60f64;
        let sec_per_day: f64 = sec_per_hour * 24f64;
        let sec_per_month: f64 = sec_per_day * 30f64;
        let sec_per_year: f64 = sec_per_day * 365f64;

        let mut elapsed = self.range as f64;
        let mut remainder: f64 = 0f64;
        let unit;

        if elapsed < sec_per_minute {
            unit = "second";
        } else if elapsed < sec_per_hour {
            remainder = elapsed % sec_per_minute;
            elapsed = (elapsed / sec_per_minute).floor();
            unit = "minute"
        } else if elapsed < sec_per_day {
            remainder = elapsed % sec_per_hour;
            elapsed = (elapsed / sec_per_hour).floor();
            unit = "hour"
        } else if elapsed < sec_per_month {
            remainder = elapsed % sec_per_day;
            elapsed = (elapsed / sec_per_day).floor();
            unit = "day"
        } else if elapsed < sec_per_year {
            remainder = elapsed % sec_per_month;
            elapsed = (elapsed / sec_per_month).floor();
            unit = "month"
        } else {
            remainder = elapsed % sec_per_year;
            elapsed = (elapsed / sec_per_year).floor();
            unit = "year"
        }

        // pluralize
        let unit = if elapsed <= 1f64 {
            unit.to_owned()
        } else {
            format!("{}s", unit)
        };

        let elapsed = elapsed as u64;
        let remainder = remainder as u64;

        return (elapsed, remainder, unit);
    }

    fn print(&self, depth: u32) -> String {

        let (elapsed, remainder, unit) = self.floor_time_unit();

        if remainder <= 0 || depth <= 1 {
            return format!("{} {}", elapsed, unit);
        }

        let pretty_remainder = Timerange::new(remainder).print(depth - 1);

        if remainder < 60 || depth <= 2 {
            return format!("{} {} and {}", elapsed, unit, pretty_remainder);
        }


        return format!("{} {} {}", elapsed, unit, pretty_remainder);

    }
}
