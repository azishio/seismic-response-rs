use std::f64::consts::PI;

/// Response analysis parameters
///
/// 応答解析のパラメータ
#[derive(Debug, Copy, Clone)]
pub struct ResponseAccAnalyzerParams {
    /// /// Natural period [ms]
    ///
    /// 固有周期 [ms]
    pub natural_period_ms: u32,

    /// Input data time resolution [ms]
    ///
    /// 入力データの時間分解能 [ms]
    pub dt_ms: u32,

    /// Mass [kg]
    ///
    /// 質量 [kg]
    pub mass: f64,

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


// 具体的な応答計算を行う
//
// x: 変位
// v: 速度
// a: 加速度
// x_1: 次の変位
// v_1: 次の速度
// a_1: 次の加速度
impl ResponseAccAnalyzer {
    /// Generate a response analyzer from parameters
    ///
    /// パラメータをもとに応答解析器を生成する
    pub fn from_params(params: ResponseAccAnalyzerParams) -> Self {
        let hardness = 4. * PI.powf(2.) * params.mass / (params.natural_period_ms as f64 / 1000.).powf(2.);
        let damping_c = params.damping_h * 2. * (params.mass * hardness).sqrt();
        Self {
            dt: params.dt_ms as f64 / 1000.,
            hardness,
            mass: params.mass,
            damping_c,
            beta: params.beta,
            init_x: params.init_x,
            init_v: params.init_v,
            init_a: params.init_a,
            init_xg: params.init_xg,
        }
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
    pub fn analyze(&self, mut xg: Vec<f64>) -> Vec<f64> {
        // 初期地震動を挿入
        xg.insert(0, self.init_xg);

        let mut result = Vec::<(f64, f64, f64)>::with_capacity(xg.len());

        // 初期条件を挿入
        result.push((self.init_x, self.init_v, self.init_a));

        (0..xg.len()).for_each(|i| {
            let (x, v, a) = result[i];
            let xg = xg[i];

            let a_1 = self.a_1(xg, a, v, x);
            let v_1 = self.v_1(a, a_1, v);
            let x_1 = self.x_1(a, a_1, v, x);

            result.push((x_1, v_1, a_1));
        });

        result.into_iter().zip(xg).map(|((_x, _v, a), xg)| {
            Self::abs_response_acc(a, xg)
        }).collect()
    }
}

#[cfg(test)]
mod test {
    use csv::Reader;

    #[test]
    fn test() {
        let mut csv = Reader::from_path("benches/211.csv").unwrap();
        let data = csv.records().map(|x| {
            let a = x.unwrap();
            println!("{:?}", "demo");
            println!("{:?}", "demo2".to_string());
            println!("{:?}", a);
        }).collect::<Vec<_>>();
        println!("hoge");
        panic!();
    }
}
