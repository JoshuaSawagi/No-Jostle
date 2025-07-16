#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_unsafe)]
#![feature(const_trait_impl)]
#![allow(warnings)]
#![feature(concat_idents)]

pub mod common;
mod fighters;

#[skyline::main(name = "NX")]
pub fn main() {
    //common::install();
    fighters::install();
}