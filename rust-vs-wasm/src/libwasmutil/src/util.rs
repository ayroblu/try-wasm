#[macro_export]
macro_rules! log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (console_log(&format_args!($($t)*).to_string()))
}

#[cfg(not(debug_assertions))]
pub fn console_log(_s: &str) {}

#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
pub fn console_log(s: &str) {
    println!("{}", s)
}

#[cfg(all(debug_assertions, target_arch = "wasm32"))]
pub fn console_log(s: &str) {
    web_sys::console::log_1(&s.into())
}
