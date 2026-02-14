#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

extern crate rml_contracts;
use rml_contracts::*;

#[allow(
    non_snake_case,
    unused_mut,
    dead_code,
    unused_assignments,
    unused_variables
)]
fn __RUSTY_KEY_CTX_FN_NAME__(mut i: u32, mut b: bool) {
    {
        let x: &u32;
        x = &i;
        let j: u32 = i;
        let y: &u32 = &j;
        b = x == y;
        b
    };
}

#[spec { name = "my_contract",
    requires(0 <= a && 0 <= b && a <= 100 && b <= 200),
    ensures(result == a + b)
}]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg_attr(rml, allow(dead_code))]
pub fn sub(a: u32, b: u32) -> u32 {
    a - b
}
