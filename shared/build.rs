#[cfg(feature = "std")]
extern crate lalrpop;

fn main() {
    #[cfg(feature = "std")]
    lalrpop::process_root().unwrap();
}