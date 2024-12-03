use microbench::statistics::Model;
use microbench::time::{Nanoseconds, Stopwatch};
use microbench::{measure, Analysis, Options, Sample};
use std::time::Duration;

pub fn options(secs: u64) -> Options {
    Options::default().time(Duration::from_secs(secs))
}

/// Benchmarks the supplied function and prints the results.
pub fn bench<T>(options: &Options, name: &str, f: impl FnMut() -> T) {
    bench_impl(name, move || measure(options, f));
}

/// Returns a new analysis for the supplied samples.
fn analysis(samples: &[Sample]) -> Analysis {
    let Model { alpha, beta, r2 } = samples
        .iter()
        .map(|m| (m.iterations as f64, m.elapsed.0 as f64))
        .collect::<Model>();

    Analysis {
        alpha: Nanoseconds(alpha),
        beta: Nanoseconds(beta),
        r2,
    }
}

/// Prints an analysis of the samples produced by the supplied function.
fn bench_impl(name: &str, f: impl FnOnce() -> Vec<Sample>) {
    let stopwatch = Stopwatch::default();
    let samples = f();
    let elapsed = stopwatch.elapsed();
    let analysis = analysis(&samples);

    let prefix = format!("{} ({}) ...", name, elapsed);
    if samples.len() < 2 || analysis.beta.0 < 0.0 {
        println!("{:<32} {:>15}", prefix, "           not enough samples");
    } else {
        let beta = Duration::from_nanos(analysis.beta.0 as _);
        println!(
            "{:<32} {:>15.3?}/iter ({:.3} R²)",
            prefix, beta, analysis.r2
        );
    }
}
