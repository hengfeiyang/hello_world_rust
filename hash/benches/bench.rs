use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hash::*;
use pprof::criterion::{Output, PProfProfiler};
use std::hash::Hasher;
use std::sync::Arc;
use std::time::Duration;

struct Label {
    name: &'static str,
    value: &'static str,
}

type Labels = Vec<Arc<Label>>;

pub fn ben_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("idgen");
    group.measurement_time(Duration::from_secs(8));
    // let key = "hello";
    let labels: Labels = vec![
        Arc::new(Label {
            name: "path",
            value: "/api/service-49",
        }),
        Arc::new(Label {
            name: "method",
            value: "GET",
        }),
        Arc::new(Label {
            name: "le",
            value: "0.33252567300796504",
        }),
    ];
    for alias in [
        //"fnv",
        "ahash",
        "defaultHash",
        //"xxhash",
        //"murmur3",
        "cityhash",
        "gxhash",
    ] {
        let mut h: Box<dyn Hasher> = match alias {
            // "fnv" => Box::new(fnv::new_hasher()),
            "ahash" => Box::new(ahash::new_hasher()),
            "defaultHash" => Box::new(default_hasher::new_hasher()),
            // "xxhash" => Box::new(xxhash::new()),
            // "murmur3" => Box::new(murmur3::new()),
            "cityhash" => Box::new(cityhash::new_hasher()),
            "gxhash" => Box::new(gxhash::new_hasher()),
            _ => panic!("not support version"),
        };
        group.bench_function(BenchmarkId::from_parameter(format!("{alias}-sum64")), |b| {
            b.iter(|| {
                labels.iter().for_each(|item| {
                    h.write(item.name.as_bytes());
                    h.write(item.value.as_bytes());
                });
                h.finish()
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
