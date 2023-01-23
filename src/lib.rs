#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros,
    clippy::borrow_interior_mutable_const
)]

mod sheik;

#[skyline::main(name = "secret_sheik")]
pub fn main() {
    sheik::install();
}