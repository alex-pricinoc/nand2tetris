pub mod compilation;
pub mod compiler;
pub mod parse;
pub mod tokenize;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        eprintln!("compiler [{}:{}] {}", file!(), line!(), format_args!( $( $t )* ))
    }
}
