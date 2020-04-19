#[macro_export]
macro_rules! log {
    ( $( $arg:tt )* ) => {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!( $( $arg )* ).into());
        #[cfg(not(target_arch = "wasm32"))]
        println!("{}", &format!( $( $arg )* ));
    }
}
