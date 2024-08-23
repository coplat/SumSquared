use wasm_bindgen::prelude::*;

// exposing `summing_up_squares` function to JavaScript
#[wasm_bindgen]
pub fn summing_up_squares(n: u64) -> u64 {
    (1..=n).map(|x| x * x).sum()
}

#[wasm_bindgen]
pub fn test_summing_up_squares() -> bool {
    summing_up_squares(7) == 140
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summing_up_squares() {
        assert_eq!(summing_up_squares(7), 140);
    }
}
