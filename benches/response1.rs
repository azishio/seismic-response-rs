use criterion::{criterion_group, criterion_main, Criterion};
use csv::Reader;

use seismic_response::{ResponseAccAnalyzer, ResponseAccAnalyzerParams};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut csv = Reader::from_path("benches/seismic_acc_waveform.csv").unwrap();
    let data = csv
        .deserialize::<f64>()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let params = ResponseAccAnalyzerParams {
        natural_period_ms: 500,
        dt_ms: 10,
        damping_h: 0.05,
        beta: 0.25,
        init_x: 0.0,
        init_v: 0.0,
        init_a: 0.0,
        init_xg: 0.0,
    };

    let analyzer = ResponseAccAnalyzer::from_params(params);

    c.bench_function("response1", |b| b.iter(|| analyzer.analyze(&data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
