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

## WebAssembly

このプログラムは[npmパッケージ](https://www.npmjs.com/package/seismic-response)としてnpmに公開されています。
Rustのクレートと同様に使用できます。

## 数式

このプログラムは、次の数式に基づいて実装されています。

### 剛性係数

質量 $ m $ とミリ秒単位の固有周期 $ T_{\text{ms}} $ に基づいて剛性係数 $ k $ を計算します：

$$
k = \frac{4 \pi^2 m}{\left(\frac{T_{\text{ms}}}{1000}\right)^2}
$$

### 減衰係数

減衰定数 $ h $、質量 $ m $、および剛性係数 $ k $ に基づいて減衰係数 $ c $ を計算します：

$$
c = 2h\sqrt{km}
$$

### ステップごとの計算

#### 応答加速度

次のステップの加速度 $ a_{n+1} $ を計算します：

$$
a_{n+1} = \frac{p_{n+1} - c\left(v_n + \frac{\Delta t}{2}a_n\right) - k\left(x_n + \Delta t v_n + \left(\frac{1}{2} -
\beta\right)\Delta t^2 a_n\right)}{m + \frac{\Delta t}{2}c + \beta \Delta t^2 k}
$$

ここで、外力 $ p_{n+1} $ は次のように与えられます：

$$
p_{n+1} = -xg_{n+1} m
$$

#### 応答速度

次のステップの速度 $ v_{n+1} $ を計算します：

$$
v_{n+1} = v_n + \frac{\Delta t}{2}(a_n + a_{n+1})
$$

#### 応答変位

次のステップの変位 $ x_{n+1} $ を計算します：

$$
x_{n+1} = x_n + \Delta t v_n + \left(\frac{1}{2} - \beta\right) \Delta t^2 a_n + \beta \Delta t^2 a_{n+1}
$$

### 絶対応答加速度

最終的な絶対応答加速度 $ a_{\text{abs}} $ を計算します：

$$
a_{\text{abs}} = a + xg
$$

以上が、プログラム内で実装されている主要な計算式です。

> [!NOTE]
> ここの数式では質量 $ m $を変数として扱っていますが、実際のプログラムでは質量1として計算しています。
> これは、質量が絶対応答加速度に影響を与えないためです。
> このことは、ドキュメント内のテストコードで確認できます。

## ライセンス

以下のいずれかの下でライセンスされています。

+ Apache License, Version 2.0, (LICENSE-APACHE または http://www.apache.org/licenses/LICENSE-2.0)
+ MITライセンス(LICENSE-MITまたは http://opensource.org/licenses/MIT)

(ドキュメンテーションコメント及びREADMEファイルの英語はDeepLとChatGPTにより翻訳されています。)
