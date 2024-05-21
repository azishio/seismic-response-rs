use std::f64::consts::PI;

#[derive(Debug, Copy, Clone)]
pub struct ResponseAnalyzerParams {
    natural_period_ms: u32,
    dt_ms: u32,
    mass: f64,
    damping: f64,
    beta: f64,
    init_x: f64,
    init_v: f64,
    init_a: f64,
    init_xg: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct ResponseAnalyzer {
    /// 時間分解能 [s]
    dt: f64,

    /// 剛性
    hardness: f64,

    /// 質量 [kg]
    mass: f64,

    /// 減衰定数
    damping: f64,

    /// ニューマークβ法のβ
    beta: f64,

    /// 初期条件
    init_x: f64,
    init_v: f64,
    init_a: f64,
    init_xg: f64,
}


/// 具体的な応答計算を行う
///
/// x: 変位
/// v: 速度
/// a: 加速度
/// x_1: 次の変位
/// v_1: 次の速度
/// a_1: 次の加速度
impl ResponseAnalyzer {
    pub fn from_params(params: ResponseAnalyzerParams) -> Self {
        Self {
            dt: params.dt_ms as f64 / 1000.,
            hardness: 4. * PI.powf(2.) * params.mass / (params.natural_period_ms as f64 / 1000.).powf(2.),
            mass: params.mass,
            damping: params.damping,
            beta: params.beta,
            init_x: params.init_x,
            init_v: params.init_v,
            init_a: params.init_a,
            init_xg: params.init_xg,
        }
    }

    fn a_1(&self, xg: f64, a: f64, v: f64, x: f64) -> f64 {
        let p_1 = -(xg * self.mass);

        (p_1 - self.damping * (v + self.dt / 2. * a) - self.hardness * (x + self.dt + (1. / 2. - self.beta) * self.dt.powf(2.) * a)) / (self.mass + self.dt * self.damping / 2. + self.beta * self.dt.powf(2.) * self.hardness)
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

    pub fn analyze(&mut self, mut xg: Vec<f64>) -> Vec<f64> {
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

        result.into_iter().zip(xg.iter()).map(|((_x, _v, a), xg)| {
            Self::abs_response_acc(a, *xg)
        }).collect()
    }
}