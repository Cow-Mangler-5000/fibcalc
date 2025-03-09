use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn fibonacci(num: u128) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    if num == 0 {
        return 0;
    }

    for _ in 1..num {
        let temp = a;
        a = b;
        b = temp + b;
    }

    b
}

