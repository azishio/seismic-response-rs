English | [日本語](README.ja.md)

# seismic-response

We conduct seismic response analysis of a single-degree-of-freedom system using Newmark's β method.
Absolute response acceleration can be obtained from the acceleration waveform of the earthquake.

## Usage

```rust
use csv::Reader;
use crate::{ResponseAccAnalyzer, ResponseAccAnalyzerParams};

fn example() {
    let mut csv = Reader::from_path("benches/seismic_acc_waveform.csv").unwrap();
    let data = csv.deserialize::<f64>().map(|x| x.unwrap()).collect::<Vec<_>>();

    let params = ResponseAccAnalyzerParams {
        // Natural period [ms]
        natural_period_ms: 500,
        // Time step of input acceleration waveform [ms]
        dt_ms: 10,
        // Mass [kg]
        mass: 100.,
        // Damping constant
        damping_h: 0.05,
        // β of Newmark's β method
        beta: 0.25,
        // Initial response displacement [m]
        init_x: 0.0,
        // Initial response velocity [m/s]
        init_v: 0.0,
        // Initial response acceleration [gal]
        init_a: 0.0,
        // Initial response input acceleration [gal]
        init_xg: 0.0,
    };

    let analyzer = ResponseAccAnalyzer::from_params(params);

    let result: Vec<f64> = analyzer.analyze(data);
}
```

## License

Licensed under either of:

+ Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)

(Documentation comments and README file in English are translated by DeepL and ChatGPT.)
