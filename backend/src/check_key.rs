use chrono::prelude::*;

pub fn validate_key(key: &str) -> bool {
    // The key is a combination of a date and 6 alphanumeric characters with the time at the end (e.g. 12.12.2020:asdh9f:12.00.00)
    // We allow a 10 second window for the time to account for latency
    let key_parts: Vec<&str> = key.split(':').collect();
    if key_parts.len() != 3 {
        println!(
            "Key is not valid because it does not contain 3 parts: {}",
            key
        );
        return false;
    }
    let date = key_parts[0];
    let random = key_parts[1];
    let time = key_parts[2];

    let date_parts: Vec<&str> = date.split('.').collect();
    if date_parts.len() != 3 {
        println!("Key is not valid because the date is not valid: {}", date);
        return false;
    }
    // Check if the date is valid
    let year = date_parts[2].parse::<i32>().unwrap();
    let month = date_parts[1].parse::<u32>().unwrap();
    let day = date_parts[0].parse::<u32>().unwrap();
    let date = NaiveDate::from_ymd_opt(year, month, day);
    if date.is_none() {
        println!(
            "Key is not valid because the date could not be parsed: {:?}",
            date
        );
        return false;
    }
    // Convert the date to %d.%m.%Y format
    let date = date.unwrap().format("%d.%m.%Y").to_string();

    // Get current date, format it and compare it to the key
    let current_date = Utc::now().format("%d.%m.%Y").to_string();
    if current_date != date.to_string() {
        println!(
            "Key is not valid because the date isn't today: (constructed: {} current: {})",
            date.to_string(),
            current_date
        );
        return false;
    }
    // Check if the time is valid
    let time_parts: Vec<&str> = time.split('.').collect();
    if time_parts.len() != 3 {
        println!(
            "Key is not valid because the time could not be parsed: {}",
            time
        );
        return false;
    }
    let hour = time_parts[0].parse::<u32>().unwrap();
    let minute = time_parts[1].parse::<u32>().unwrap();
    let second = time_parts[2].parse::<u32>().unwrap();
    // UTC+1
    let time = Utc::now().naive_utc() + chrono::Duration::hours(1);

    if hour != time.hour() || minute != time.minute() {
        println!(
            "Key is not valid because the hour or minute is not valid: {}",
            time.format("%H.%M.%S").to_string()
        );
        return false;
    }
    // Make a 10 second window for the time, only for the future
    if second > time.second() && second - time.second() >= 10 {
        println!(
            "Key is not valid because the second is not valid: {}",
            time.format("%H.%M.%S").to_string()
        );
        return false;
    }

    // Check if the random string is valid
    if random.len() != 6 {
        println!(
            "Key is not valid because the random string is not valid: {}",
            random
        );
        return false;
    }
    // Check if the random string is alphanumeric
    let alphanumeric = random.chars().all(|c| c.is_alphanumeric());
    if !alphanumeric {
        println!(
            "Key is not valid because the random string is not valid: {}",
            random
        );
        return false;
    }

    true
}
