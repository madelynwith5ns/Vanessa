/// Returns the current time in UNIX epoch milliseconds.
/// (milliseconds since January 1st, 1970: 00:00:00)
pub fn epoch_millis() -> u128 {
    let now = std::time::SystemTime::now();
    return match now.duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => duration.as_millis(),
        Err(_) => 0,
    };
}

/// Returns the number of days since the UNIX epoch.
pub fn epoch_days(point: u128) -> u128 {
    // you all know what that 86.4m is
    // right?
    return point / 86400000;
}

const MILLISECONDS_IN_MONTH: [u128; 12] = [
    2_678_400_000,
    2_419_200_000,
    2_678_400_000,
    2_592_000_000,
    2_678_400_000,
    2_592_000_000,
    2_678_400_000,
    2_678_400_000,
    2_592_000_000,
    2_678_400_000,
    2_592_000_000,
    2_678_400_000,
];
const MONTHS: [&'static str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
/// Returns the number of months that have passed since the UNIX epoch.
pub fn epoch_months(mut epoch: u128) -> u128 {
    let mut curmonth = 0u128;
    let mut months = 0u128;

    loop {
        let mut ms = MILLISECONDS_IN_MONTH[curmonth as usize];
        let full_months = 23640 + months;
        if curmonth == 1
            && (full_months % 48 == 0 && (full_months % 1200 != 0 || full_months % 4800 != 0))
        {
            ms += 86_400_000;
        }
        if epoch < ms {
            return months;
        }
        epoch -= ms;
        curmonth += 1;
        months += 1;
        if curmonth > 11 {
            curmonth = 0;
        }
    }
}

/// Returns the current month of the year.
pub fn current_month(mut epoch: u128) -> &'static str {
    let mut curmonth = 0u128;
    let mut months = 23639u128;

    loop {
        let mut ms = MILLISECONDS_IN_MONTH[curmonth as usize];
        if curmonth == 1 && (months % 48 == 0 && (months % 1200 != 0 || months % 4800 != 0)) {
            ms += 86_400_000;
        }
        if epoch < ms {
            return MONTHS[curmonth as usize];
        }
        epoch -= ms;
        curmonth += 1;
        months += 1;
        if curmonth > 11 {
            curmonth = 0;
        }
    }
}

/// Returns the current day of the month.
pub fn date(mut epoch: u128) -> u8 {
    let mut curmonth = 0u128;
    let mut months = 23639u128;

    loop {
        let mut ms = MILLISECONDS_IN_MONTH[curmonth as usize].clone();
        if curmonth == 1 && (months % 48 == 0 && (months % 1200 != 0 || months % 4800 == 0)) {
            ms += 86_400_000;
        }
        if epoch < ms {
            return (epoch / 86_400_000 + 1) as u8;
        }
        epoch -= ms;
        curmonth += 1;
        months += 1;
        if curmonth > 11 {
            curmonth = 0;
        }
    }
}

/// Returns the number of years since the UNIX epoch.
pub fn epoch_years(epoch: u128) -> u128 {
    return epoch / 31556925975;
}

/// Returns the current year.
pub fn current_year(epoch: u128) -> u128 {
    return epoch_years(epoch) + 1970;
}

/// Returns the current hour of the day.
pub fn hour(epoch: u128) -> u8 {
    let hours = epoch / 1000 / 60 / 60;
    return (hours % 24) as u8;
}

/// Returns the current minute of the hour.
pub fn minute(epoch: u128) -> u8 {
    let minutes = epoch / 1000 / 60;
    return (minutes % 60) as u8;
}

/// Returns the current second of the minute.
pub fn second(epoch: u128) -> u8 {
    let seconds = epoch / 1000;
    return (seconds % 60) as u8;
}

/// Creates the string timestamp of the provided UNIX timestamp.
pub fn timestamp(epoch: u128) -> String {
    let mut timestamp = String::new();
    timestamp.push_str(&format!("{}", current_year(epoch)));
    timestamp.push('-');
    timestamp.push_str(&current_month(epoch)[0..3]);
    timestamp.push('-');
    timestamp.push_str(&format!("{}", date(epoch)));
    timestamp.push('-');
    timestamp.push_str(&format!("{:02}", hour(epoch)));
    timestamp.push(':');
    timestamp.push_str(&format!("{:02}", minute(epoch)));
    timestamp.push(':');
    timestamp.push_str(&format!("{:02}", second(epoch)));
    return timestamp;
}

/// Returns the string timestamp of the current time.
pub fn timestamp_now() -> String {
    return timestamp(epoch_millis());
}
