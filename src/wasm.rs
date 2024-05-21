use js_sys::Float64Array;
use wasm_bindgen::prelude::*;

use crate::{ResponseAccAnalyzer, ResponseAccAnalyzerParams};

#[wasm_bindgen]
pub fn calc_response_acc(data: Vec<f64>, params: ResponseAccAnalyzerParams) -> Float64Array {
    let analyzer = ResponseAccAnalyzer::from_params(params);
    Float64Array::from(analyzer.analyze(data).as_slice())
}
