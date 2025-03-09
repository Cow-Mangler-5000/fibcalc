use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn fibonacci(num: i32) -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 1;
    
    if num < 0 {
        return -1;
    }
    if num == 0 {
        return 0;
    }
    else {
        for _ in 1..num {
            let temp: i32 = a;
            a = b;
            b = temp + b;
        }
        return b;
    }
}
