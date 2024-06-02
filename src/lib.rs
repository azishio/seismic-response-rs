use std::f64::consts::PI;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

/// Response analysis parameters
///
/// 応答解析のパラメータ
#[derive(Debug, Clone, Copy, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ResponseAccAnalyzerParams {
    /// Natural period [ms]
    ///
    /// 固有周期 [ms]
    pub natural_period_ms: u32,

    /// Input data time resolution [ms]
    ///
    /// 入力データの時間分解能 [ms]
    pub dt_ms: u32,

    /// Damping constant
    ///
    /// 減衰定数
    pub damping_h: f64,

    /// β of Newmark-β method
    ///
    /// ニューマークβ法のβ
    pub beta: f64,

    /// Initial response displacement [m]
    ///
    /// 初期応答変位 [m]
    pub init_x: f64,

    /// Initial response velocity [m/s]
    ///
    /// 初期応答速度 [m/s]
    pub init_v: f64,

    /// Initial response acceleration [gal]
    ///
    /// 初期応答加速度 [gal]
    pub init_a: f64,

    /// Initial ground acceleration [gal]
    ///
    /// 初期地震動 [gal]
    pub init_xg: f64,
}


/// Seismic response analyser for one mass point systems.
///
/// 1質点系の地震応答解析器
#[derive(Debug, Copy, Clone)]
#[wasm_bindgen]
pub struct ResponseAccAnalyzer {
    dt: f64,
    hardness: f64,
    mass: f64,
    // 減衰係数
    damping_c: f64,
    beta: f64,
    init_x: f64,
    init_v: f64,
    init_a: f64,
    init_xg: f64,
}

/// Result of response analysis
///
/// 応答解析の結果
#[derive(Debug)]
#[wasm_bindgen(getter_with_clone)]
pub struct Result {
    /// Response displacement [m]
    ///
    /// 応答変位 [m]
    pub x: Vec<f64>,

    /// Response velocity [m/s]
    ///
    /// 応答速度 [m/s]
    pub v: Vec<f64>,

    /// Response acceleration [gal]
    ///
    /// 応答加速度 [gal]
    pub a: Vec<f64>,

    /// Absolute response acceleration [gal]
    ///
    /// 絶対応答加速度 [gal]
    pub abs_acc: Vec<f64>,
}

// 具体的な応答計算を行う
//
// x: 変位
// v: 速度
// a: 加速度
// x_1: 次の変位
// v_1: 次の速度
// a_1: 次の加速度
#[wasm_bindgen]
impl ResponseAccAnalyzer {
    /// Generate a response analyzer from parameters
    ///
    /// パラメータをもとに応答解析器を生成する
    ///
    ///g
    #[wasm_bindgen]
    pub fn from_params(params: ResponseAccAnalyzerParams) -> Self {
        // 結果に質量は影響しないので1としている
        let mass = 100.;

        let hardness = Self::calc_hardness(mass, params.natural_period_ms);
        let damping_c = Self::calc_damping_c(params.damping_h, mass, hardness);
        Self {
            dt: params.dt_ms as f64 / 1000.,
            hardness,
            mass,
            damping_c,
            beta: params.beta,
            init_x: params.init_x,
            init_v: params.init_v,
            init_a: params.init_a,
            init_xg: params.init_xg,
        }
    }

    fn calc_hardness(mass: f64, natural_period_ms: u32) -> f64 {
        4. * PI.powf(2.) * mass / (natural_period_ms as f64 / 1000.).powf(2.)
    }

    fn calc_damping_c(damping_h: f64, mass: f64, hardness: f64) -> f64 {
        damping_h * 2. * (mass * hardness).sqrt()
    }


    /// This function does not affect the result. (Strictly speaking, it may have an effect due to floating point number errors.)
    /// This function can be used to change parameters related to mass.
    ///
    /// この関数は結果に影響を与えません。（厳密には浮動小数点数の誤差があるため、影響があるかもしれません）
    /// この関数を使用すると質量に関するパラメータを変更できます。
    ///
    /// [ResponseAccAnalyzerParams]には`mass`が含まれていませんが、それはmassが結果に影響を与えないためです。
    /// この関数を使用することで、massを変更したときの影響を確認できます。
    ///
    /// [ResponseAccAnalyzerParams] does not include `mass` because mass does not affect the result.
    /// You can use this function to check the effect of changing the mass.
    ///
    /// # Example
    ///
    /// ```
    /// use close_to::assert_close_to;
    /// use csv::Reader;
    /// use seismic_response::{ResponseAccAnalyzer, ResponseAccAnalyzerParams};
    ///
    /// let mut csv = Reader::from_path("benches/seismic_acc_waveform.csv").unwrap();
    /// let data = csv.deserialize::<f64>().map(|x| x.unwrap()).collect::<Vec<_>>();
    ///
    /// let natural_period_ms = 500;
    /// let damping_h = 0.05;
    ///
    /// let params = ResponseAccAnalyzerParams {
    /// natural_period_ms,
    /// damping_h,
    /// dt_ms: 10,
    /// beta: 0.25,
    /// init_x: 0.0,
    /// init_v: 0.0,
    /// init_a: 0.0,
    /// init_xg: 0.0,
    /// };
    ///
    /// let mut analyzer = ResponseAccAnalyzer::from_params(params);
    /// let result1 = analyzer.analyze(data.clone());
    ///
    /// let result2 = analyzer.set_mass(10., natural_period_ms, damping_h).analyze(data);
    ///
    /// result1.abs_acc.into_iter().zip(result2.abs_acc).for_each(|(r1, r2)| {
    /// // Allow an error margin of 10^-10
    /// // 10^-10の誤差を許容する
    /// assert_close_to(r1, r2, 10);
    /// });
    /// ```
    ///
    /// このドキュメントは正しくビルドされたため、質量を変更しても結果に影響がないことがわかります。
    /// You can see that this document was built correctly, so changing the mass will not affect the result.
    pub fn set_mass(&mut self, mass: f64, natural_period_ms: u32, damping_h: f64) -> Self {
        self.mass = mass;
        self.hardness = Self::calc_hardness(mass, natural_period_ms);
        self.damping_c = Self::calc_damping_c(damping_h, mass, self.hardness);
        *self
    }

    fn a_1(&self, xg: f64, a: f64, v: f64, x: f64) -> f64 {
        let p_1 = -(xg * self.mass);

        (p_1 - self.damping_c * (v + self.dt / 2. * a) - self.hardness * (x + self.dt * v + (1. / 2. - self.beta) * self.dt.powf(2.) * a)) / (self.mass + self.dt * self.damping_c / 2. + self.beta * self.dt.powf(2.) * self.hardness)
    }

    fn v_1(&self, a: f64, a_1: f64, v: f64) -> f64 {
        v + (a_1 + a) * self.dt / 2.
    }

    fn x_1(&self, a: f64, a_1: f64, v: f64, x: f64) -> f64 {
        x + v * self.dt + (1. / 2. - self.beta) * a * self.dt.powf(2.) + self.beta * a_1 * self.dt.powf(2.)
    }

    // 絶対応答加速度
    fn abs_response_acc(a: f64, xg: f64) -> f64 {
        a + xg
    }

    /// Calculate absolute response acceleration.
    /// xg: Earthquake acceleration waveform [gal]
    ///
    /// 絶対応答加速度を計算する。
    /// xg: 地震の加速度波形 [gal]
    #[wasm_bindgen]
    pub fn analyze(&self, mut xg: Vec<f64>) -> Result {
        // 初期地震動を挿入
        xg.insert(0, self.init_xg);

        let mut result = Result {
            x: Vec::with_capacity(xg.len()),
            v: Vec::with_capacity(xg.len()),
            a: Vec::with_capacity(xg.len()),
            abs_acc: Vec::with_capacity(xg.len()),
        };

        result.x.push(self.init_x);
        result.v.push(self.init_v);
        result.a.push(self.init_a);

        (0..xg.len()).for_each(|i| {
            let x = result.x[i];
            let v = result.v[i];
            let a = result.a[i];

            let xg = xg[i];

            let a_1 = self.a_1(xg, a, v, x);
            let v_1 = self.v_1(a, a_1, v);
            let x_1 = self.x_1(a, a_1, v, x);

            result.x.push(x_1);
            result.v.push(v_1);
            result.a.push(a_1);
            result.abs_acc.push(Self::abs_response_acc(a_1, xg));
        });

        result
    }
}

#[cfg(test)]
mod test {
    use close_to::assert_close_to;
    use csv::Reader;

    use crate::{ResponseAccAnalyzer, ResponseAccAnalyzerParams};

    #[test]
    fn test() {
        let mut csv = Reader::from_path("benches/seismic_acc_waveform.csv").unwrap();
        let data = csv.deserialize::<f64>().map(|x| x.unwrap()).collect::<Vec<_>>();

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
        analyzer.analyze(data);
    }

    #[test]
    fn same_result_change_mass() {
        let mut csv = Reader::from_path("benches/seismic_acc_waveform.csv").unwrap();
        let data = csv.deserialize::<f64>().map(|x| x.unwrap()).collect::<Vec<_>>();

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

        let mut analyzer = ResponseAccAnalyzer::from_params(params);
        let result1 = analyzer.analyze(data.clone());

        let result2 = analyzer.set_mass(10., 500, 0.05).analyze(data);

        result1.abs_acc.into_iter().zip(result2.abs_acc).for_each(|(r1, r2)| {
            assert_close_to(r1, r2, 5);
        });
    }
}
