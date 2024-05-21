[English](README.md) | 日本語

# seismic-response

Newmarkのβ法により1質点系の地震応答解析を行います。
地震の加速度波形から、絶対応答加速度を求められます。

## 使い方

```rust
use csv::Reader;
use crate::{ResponseAccAnalyzer, ResponseAccAnalyzerParams};

fn example() {
    let mut csv = Reader::from_path("benches/seismic_acc_waveform.csv").unwrap();
    let data = csv.deserialize::<f64>().map(|x| x.unwrap()).collect::<Vec<_>>();

    let params = ResponseAccAnalyzerParams {
        // 固有周期 [ms]
        natural_period_ms: 500,
        // 入力加速度波形の時間刻み [ms]
        dt_ms: 10,
        // 質量 [kg]
        mass: 100.,
        // 減衰定数
        damping_h: 0.05,
        // Newmarkのβ法のβ
        beta: 0.25,
        // 初期応答変位 [m]
        init_x: 0.0,
        // 初期応答速度 [m/s]
        init_v: 0.0,
        // 初期応答加速度 [gal]
        init_a: 0.0,
        // 初期応答入力加速度 [gal]
        init_xg: 0.0,
    };

    let analyzer = ResponseAccAnalyzer::from_params(params);

    let result: Vec<f64> = analyzer.analyze(data);
}
```

## ライセンス

以下のいずれかの下でライセンスされています。

+ Apache License, Version 2.0, (LICENSE-APACHE または http://www.apache.org/licenses/LICENSE-2.0)
+ MITライセンス(LICENSE-MITまたは http://opensource.org/licenses/MIT)

(ドキュメンテーションコメント及びREADMEファイルの英語はDeepLとChatGPTにより翻訳されています。)
