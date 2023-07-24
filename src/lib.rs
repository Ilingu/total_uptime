use std::{fmt::Display, time::Duration};

use colored::Colorize;

pub struct TotalUptime(Duration);

/// returns in the total uptime in seconds
pub fn compute_total_uptime(input: &str) -> TotalUptime {
    let dur_as_sec = input
        .lines()
        .filter_map(|l| l.split_whitespace().last())
        .filter_map(|time| {
            let time_data = time
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(':')
                .collect::<Vec<_>>();
            if time_data.len() != 2 {
                return None;
            }

            if let (Ok(hours), Ok(min)) = (time_data[0].parse::<u64>(), time_data[1].parse::<u64>())
            {
                Some(hours * 3600 + min * 60)
            } else {
                None
            }
        })
        .sum::<u64>();
    TotalUptime(Duration::from_secs(dur_as_sec))
}

pub trait ColoredUnwrap<T, E> {
    fn colored_unwrap(self, why: &str) -> T;
}

impl<T, E> ColoredUnwrap<T, E> for Result<T, E> {
    fn colored_unwrap(self, why: &str) -> T {
        match self {
            Ok(t) => t,
            Err(_) => {
                eprintln!("{}", why.red().bold());
                panic!();
            }
        }
    }
}

impl Display for TotalUptime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time_elapsed = self.0.as_secs();

        let (days, hours_remaining) = (
            time_elapsed / (3600 * 24),
            time_elapsed.rem_euclid(3600 * 24),
        );
        let (hours, min_remaining) = (hours_remaining / 3600, hours_remaining.rem_euclid(3600));
        let (min, sec_remaining) = (min_remaining / 60, min_remaining.rem_euclid(60));

        writeln!(
            f,
            "{} {}",
            "The total uptime of your machine is".bright_purple(),
            format!("{days:0>2}d {hours:0>2}h {min:0>2}m {sec_remaining:0>2}s")
                .green()
                .bold()
        )?;
        writeln!(
            f,
            "{}",
            "(This exclude the current uptime session)"
                .italic()
                .truecolor(149, 165, 166)
        )
    }
}
