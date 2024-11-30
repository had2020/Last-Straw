//!  comments item containing the comment
pub use glfw;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// the three is in markdown
// cargo test
// doc gen cargo doc --open
/// simple hello world print
///
///# Examples
///```rust
///let arg = 2;
///let result = laststraw::yol_t(arg);
///assert_eq!(3, result);
///```
pub fn yol_t(x: f32) -> f32{
    let b = x + 1.0_f32;
    b
}