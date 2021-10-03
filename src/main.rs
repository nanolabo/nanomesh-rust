#![allow(dead_code)]

mod base;
mod utils;
mod mesh;

use crate::base::*;
use crate::mesh::*;
use crate::utils::*;

use std::hash::BuildHasherDefault;
use std::time::Instant;

//use std::collections::HashMap;
use hashbrown::HashMap;

// Type inference lets us omit an explicit type signature (which
// would be `HashMap<String, String>` in this example).

fn main() {

    let now = Instant::now();
    // let mut r = 0.0;
    // let mut v = 0.0;
    // for _i in 0..100_000_000 {
    //     v += 3.0;
    //     let a = Vector3::new(v,0.,v);
    //     v += 7.0;
    //     let b = Vector3::new(v,v,0.);
    //     let c = (&a ^ &b).normalized();
    //     r += c.magnitude();
    // }

    let mut dic = HashMap::<i32, Vector3, _>::with_hasher(
        BuildHasherDefault::<SimpleHasher>::default()
    );

    //let mut dic = HashMap::new();

    for i in 0..50_000_000 {
        dic.insert(i, Vector3::new(0.,0.,0.));
    }

    let mut res = false;
    for i in 0..50_000_000 {
        res = dic.contains_key(&i);
    }

    println!("r={}, ms={}", res, now.elapsed().as_millis());
}
