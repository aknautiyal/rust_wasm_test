use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn count_vowels(input: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let count = input.chars().filter(|c| vowels.contains(c)).count();
    count.to_string()
}
