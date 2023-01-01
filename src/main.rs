use std::path::PathBuf;
use std::time::Duration;

use anyhow::{anyhow, ensure, Result};
use clap::Parser;
use csv::{ReaderBuilder, Trim};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {
    max_hr: f64,

    file: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let max_hr = cli.max_hr;
    let path = cli.file.as_path();

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .trim(Trim::All)
        .from_path(path)?;

    assert!(reader.has_headers());

    let headers = reader.headers()?;

    dbg!(headers);

    let time_idx = headers
        .iter()
        .position(|x| x == "Time")
        .ok_or(anyhow!("No \"Time\" column"))?;

    let avg_hr_idx = headers
        .iter()
        .position(|x| x == "Avg HR")
        .ok_or(anyhow!("No \"Avg HR\" column"))?;

    let date_idx = headers
        .iter()
        .position(|x| x == "Date")
        .ok_or(anyhow!("No \"Date\" column"))?;

    dbg!(time_idx);
    dbg!(avg_hr_idx);
    dbg!(date_idx);

    for record in reader.records() {
        let record = record?;

        let time_str = record.get(time_idx).unwrap();
        let avg_hr_str = record.get(avg_hr_idx).unwrap();
        let date_str = record.get(date_idx).unwrap();

        let avg_hr = avg_hr_str.parse::<f64>()?;
        let time = parse_time(time_str)?;

        dbg!(date_str, max_hr, avg_hr, time);

        let tss = calculate_tss_per_activity(max_hr, avg_hr, time);

        dbg!(tss);

    }

    Ok(())
}

fn parse_time(time_str: &str) -> Result<Duration> {
    let temp = time_str.split(':').collect::<Vec<_>>();

    ensure!(
        temp.len() == 3,
        format!("Invalid data in \"Time\" column: {}", time_str)
    );

    let hours = temp[0].parse::<f64>()?;
    let minutes = temp[1].parse::<f64>()?;
    let seconds = temp[2].parse::<f64>()?;

    Ok(Duration::from_secs_f64(
        60.0 * 60.0 * hours + 60.0 * minutes + seconds,
    ))
}

fn calculate_tss_per_activity(max_hr: f64, avg_hr: f64, time: Duration) -> f64 {
    let if_ = avg_hr / max_hr;

    dbg!(if_);

    let hours = time.as_secs_f64() / 3600.0;

    let tss = if_ * if_ * 100.0 * hours;

    tss
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tss() {
        dbg!(calculate_tss_per_activity(
            180.0,
            88.0,
            Duration::from_secs_f64(40.283333333333333333333333)
        ));
    }
}
