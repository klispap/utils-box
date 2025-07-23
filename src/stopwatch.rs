//! # Stopwatch utilities
//! A toolbox of small time keeping utilities.
//! Useful for measuring and comparing execution times.

use std::{
    cmp::Ordering,
    collections::HashMap,
    time::{Duration, Instant},
};

pub struct StopWatch {
    timer: Instant,
    last_lap: Duration,
}

impl StopWatch {
    pub fn start() -> Self {
        StopWatch {
            timer: std::time::Instant::now(),
            last_lap: Duration::new(0, 0),
        }
    }

    pub fn lap_time(&mut self, lap: &str) -> Duration {
        let elapsed = self.timer.elapsed() - self.last_lap;

        let (time, unit) = format_duration(&elapsed);

        log_info!("[StopWatch] Lap [{}]: {} {}", lap, time, unit);
        self.last_lap = self.timer.elapsed();

        elapsed
    }

    pub fn check_time(&self, lap: &str) -> Duration {
        let elapsed = self.timer.elapsed();

        let (time, unit) = format_duration(&elapsed);

        log_info!("[StopWatch] Up to [{}]: {} {}", lap, time, unit);
        elapsed
    }
}

fn format_duration(duration: &Duration) -> (f64, &str) {
    let micros = duration.as_micros();
    if micros < 1000 {
        (micros as f64, "us")
    } else if micros < 1000000 {
        ((micros / 1000) as f64, "ms")
    } else {
        (micros as f64 / 1.0e6, "s")
    }
}

pub struct StopWatchStats {
    lap_totals: HashMap<String, (u8, Duration)>,
    max_id: u8,
}

impl StopWatchStats {
    pub fn init() -> Self {
        StopWatchStats {
            lap_totals: HashMap::new(),
            max_id: 0,
        }
    }

    pub fn store_lap(&mut self, lap: &str, time: Duration) -> Duration {
        match self.lap_totals.get(lap) {
            Some(&(id, total)) => {
                self.lap_totals.insert(lap.to_string(), (id, time + total));
                time + total
            }
            None => {
                self.max_id += 1;
                self.lap_totals.insert(lap.to_string(), (self.max_id, time));
                time
            }
        }
    }

    pub fn report(&self) {
        if self.lap_totals.is_empty() {
            let report_width = 20;
            log_info!("+{:->report_width$}+", "");
            log_info!("|{:^report_width$}|", "StopWatch Report");
            log_info!("+{:->report_width$}+", "");
            log_info!("| No laps recorded!  |");
            log_info!("+{:->report_width$}+", "");
            return;
        }

        let max = self
            .lap_totals
            .iter()
            .max_by(|a, b| {
                if a.0.len() > b.0.len() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .unwrap()
            .0
            .len();

        let mut laps: Vec<(&String, &(u8, Duration))> = self.lap_totals.iter().collect();
        laps.sort_by(|a, b| {
            if a.1 .0 > b.1 .0 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        let report_width = max + 14;
        log_info!("+{:->report_width$}+", "");
        log_info!("|{:^report_width$}|", "StopWatch Report");
        log_info!("+{:->report_width$}+", "");

        laps.iter().for_each(|(lap, (_id, time))| {
            let (time, unit) = format_duration(time);
            log_info!("|{:>max$} | {:7.3} {:3}|", lap, time, unit);
        });

        log_info!("+{:->report_width$}+", "");
    }
}

pub struct TimeKeeper {
    stop_watch: StopWatch,
    stats: StopWatchStats,
}

impl TimeKeeper {
    pub fn init() -> Self {
        TimeKeeper {
            stop_watch: StopWatch::start(),
            stats: StopWatchStats::init(),
        }
    }

    pub fn lap(&mut self, lap: &str) -> Duration {
        self.stats.store_lap(lap, self.stop_watch.lap_time(lap))
    }

    pub fn lap_totals(&self, lap: &str) -> Duration {
        let total = *self.stats.lap_totals.get(lap).unwrap();
        let (time, unit) = format_duration(&total.1);
        log_info!("[TimeKeeper] Lap [{}] Total Time: {} {}", lap, time, unit);

        total.1
    }

    pub fn totals(&self) {
        self.stats.report()
    }

    pub fn merge(&mut self, time_keeper: Self) {
        time_keeper
            .stats
            .lap_totals
            .iter()
            .for_each(|(lap, (id, time))| {
                let new_time = match self.stats.lap_totals.get(lap) {
                    Some((d, t)) => (*d.min(id), *t + *time),
                    None => (*id, *time),
                };
                self.stats.lap_totals.insert(lap.to_string(), new_time);
            });
    }
}

#[cfg(test)]
mod tests {
    use crate::stopwatch::*;

    #[test]
    fn stopwatch_test() {
        let mut s = StopWatch::start();
        let mut stats = StopWatchStats::init();

        stats.store_lap("aaaaaaaaaaaa", s.lap_time("a"));

        for _ in 0..5 {
            std::thread::sleep(Duration::from_millis(5));
            stats.store_lap("b", s.lap_time("b"));
        }

        stats.store_lap("aaaaaaaaaaaa", s.lap_time("a"));

        stats.report();
    }

    #[test]
    fn timekeeper_test() {
        let mut s = TimeKeeper::init();
        let mut t = TimeKeeper::init();

        s.totals();

        s.lap("arni");

        for _ in 0..5 {
            std::thread::sleep(Duration::from_millis(5));
            s.lap("rifi");
            t.lap("rifi");
        }
        s.lap_totals("rifi");
        std::thread::sleep(Duration::from_millis(1234));
        s.lap("arni");

        s.totals();
        t.totals();

        s.merge(t);

        s.totals();
    }
}
