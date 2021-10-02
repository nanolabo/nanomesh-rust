#![allow(dead_code)]

mod base;
mod utils;
mod mesh;

use crate::base::*;
use crate::mesh::*;

use std::time::Instant;

fn main() {

    let now = Instant::now();
    let mut r = 0.0;
    let mut v = 0.0;
    for _i in 0..100_000_000 {
        v += 3.0;
        let a = Vector3::new(v,0.,v);
        v += 7.0;
        let b = Vector3::new(v,v,0.);
        let c = (&a ^ &b).normalized();
        r += c.magnitude();
    }
    println!("r={}, ms={}", r, now.elapsed().as_millis());
}