use super::Result;
use super::models::Sample;
use std::fs::File;
use std::io::Write;

pub fn save(file: &mut File, rows: &[Sample], round: usize) -> Result<usize> {
    for r in rows {
        writeln!(
            file,
            "{},{},{},{},{},{},{},{:.6},{:.6},{},{},{},{},{},{},{},{},{:.9}",
            round,
            r.case,
            r.body_bytes,
            r.budget,
            r.concurrency,
            r.index,
            r.pages,
            r.latency_ms,
            r.first_page_ms,
            r.status,
            r.correct,
            r.error,
            r.response_bytes,
            r.returned_chars,
            r.conflicts,
            r.reasoning,
            r.http_version,
            r.phase_seconds
        )?;
    }
    file.flush()?;
    let mut latencies = rows
        .iter()
        .filter(|r| r.correct)
        .map(|r| r.latency_ms)
        .collect::<Vec<_>>();
    latencies.sort_by(f64::total_cmp);
    let p = |q: f64| {
        if latencies.is_empty() {
            f64::NAN
        } else {
            latencies[((latencies.len() as f64 * q).ceil() as usize).saturating_sub(1)]
        }
    };
    let errors = rows.len() - latencies.len();
    if let Some(first) = rows.first() {
        println!(
            "round={} case={} body={} budget={} c={} n={} errors={} p50={:.3}ms p95={:.3}ms p99={:.3}ms",
            round,
            first.case,
            first.body_bytes,
            first.budget,
            first.concurrency,
            rows.len(),
            errors,
            p(0.5),
            p(0.95),
            p(0.99)
        );
    }
    Ok(errors)
}
