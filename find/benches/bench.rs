use ::std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::criterion::{Output, PProfProfiler};

use find::{simd, std};

pub fn ben_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("find");
    group.measurement_time(Duration::from_secs(8));
    let body = "{\"time_local\": \"07/Feb/2025:03:41:26 +0000\", \"remote_addr\": \"\", \"x_forward_for\": \"52.76.38.220\", \"request_id\": \"5cb629bec66ddec929dd06b640e17435\", \"remote_user\": \"ernest@atomi.cloud\", \"bytes_sent\": 588, \"request_time\": 0.000, \"status\": \"429\", \"vhost\": \"api.openobserve.ai\", \"request_proto\": \"HTTP/2.0\", \"path\": \"/api/atomicloud_MwvsSHPiOT9uFdn/v1/metrics\", \"request_query\": \"\", \"request_length\": 10, \"duration\": 0.000,\"method\": \"POST\", \"http_referrer\": \"\", \"http_user_agent\": \"OpenTelemetry Collector Contrib/0.85.0 (linux/amd64)\" }";
    let keyword = "request_id";
    for alias in ["std", "simd"] {
        let h = match alias {
            "std" => std::find,
            "simd" => simd::find,
            _ => panic!("not support version"),
        };
        group.bench_function(BenchmarkId::from_parameter(format!("{alias}-find")), |b| {
            b.iter(|| {
                let _ = h(black_box(&body), black_box(&keyword));
            })
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = ben_benchmark
}

criterion_main!(benches);
