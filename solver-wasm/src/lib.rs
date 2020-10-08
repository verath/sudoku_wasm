use serde::{Deserialize, Serialize};
use solver;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct SolveResult {
    pub error: Option<String>,
    pub solution: Option<String>,
}

impl SolveResult {
    fn success(solved: &str) -> SolveResult {
        SolveResult {
            error: None,
            solution: Some(solved.to_owned()),
        }
    }
    fn fail(err: &str) -> SolveResult {
        SolveResult {
            error: Some(err.to_owned()),
            solution: None,
        }
    }
}

#[wasm_bindgen]
pub fn solve(input: &str) -> JsValue {
    let result = match solver::solve(input) {
        Ok(solved) => SolveResult::success(&solved),
        Err(err) => SolveResult::fail(err),
    };
    JsValue::from_serde(&result).unwrap()
}
