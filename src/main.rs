use chrono::{DateTime, Local, TimeDelta};

const SLEEP_CYCLE_MINS: i64 = 90;

fn cycle_count_to_bedtime(
    local_time: DateTime<Local>,
    cycle_count: i32,
) -> Option<DateTime<Local>> {
    return local_time.checked_add_signed(TimeDelta::minutes(SLEEP_CYCLE_MINS) * cycle_count);
}

fn main() {
    let sys_time = Local::now();
    let recommended_cycles = [4, 5, 6];

    let bedtimes = recommended_cycles.map(|c| cycle_count_to_bedtime(sys_time, c));

    println!("Set your alarm at...");
    for bt in &bedtimes {
        println!("{time}", time = bt.unwrap().format("%H:%M %p"))
    }
}
