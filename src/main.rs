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

use linked_hash_set::LinkedHashSet;
use priority_queue::PriorityQueue;
//use linked_hash_map::LinkedHashMap;
use indexmap::{IndexMap, IndexSet};

// Type inference lets us omit an explicit type signature (which
// would be `HashMap<String, String>` in this example).

use std::cmp::Ordering;

#[derive(Hash)]
struct Edge {
    id: i32,
    error: i32,
}

#[derive(Hash)]
struct MyEdge {
    a: i32,
}

impl Eq for MyEdge {

}


impl PartialEq for MyEdge {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a
    }
}

impl Eq for Edge {

}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.error == other.error {
            self.id.partial_cmp(&other.id).unwrap()
        }
        else {
            self.error.partial_cmp(&other.error).unwrap()
        }
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn main() {


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

    // let mut dic = HashMap::<i32, Vector3, _>::with_hasher(
    //     BuildHasherDefault::<SimpleHasher>::default()
    // );

    //let mut dic = HashMap::new();

    // for i in 0..50_000_000 {
    //     dic.insert(i, Vector3::new(0.,0.,0.));
    // }

    // let mut res = false;
    // for i in 0..50_000_000 {
    //     res = dic.contains_key(&i);
    // }

    // const s: usize = 500_000_000;
    // let mut array = vec![0.0; s];
    // for i in 0..s {
    //     array[i] = i as f64;
    // }
    // let now = Instant::now();
    // array.sort_by(|a, b| b.partial_cmp(a).unwrap());
    // println!("r={}, ms={}", array[10], now.elapsed().as_millis());

    // let now = Instant::now();
    // let mut btreemap = std::collections::BTreeMap::<Edge, i32>::new();
    // const s: i32 = 1_000_000;
    // const u: i32 = 1_000;
    // for i in 0..s {
    //     btreemap.insert(Edge { id: i, error: i % u }, i);
    // }
    // for i in 0..500_000 {
    //     match btreemap.remove(&Edge { id: i, error: i % u }) {
    //         None => panic!("shit"),
    //         _ => ()
    //     };
    //     match btreemap.remove(&Edge { id: i + 1, error: (i + 1) % u }) {
    //         None => panic!("removal failed"),
    //         _ => ()
    //     };
    //     btreemap.insert(Edge { id: i + 1, error: (i + 1) % u }, i);
    // }
    // println!("len={}", btreemap.len());
    // println!("ms={}", now.elapsed().as_millis());
    // println!("min={}", btreemap.iter().next().unwrap().0.error);
    // println!("max={}", btreemap.iter().next_back().unwrap().0.error);

    // let now = Instant::now();
    // let mut btreemap = linked_hash_set::LinkedHashSet::new();
    // const s: i32 = 1_000_000;
    // const u: i32 = 1_000;
    // for i in 0..s {
    //     btreemap.insert(Edge { id: i, error: i % u });
    // }
    // for i in 0..500_000 {
    //     if !btreemap.remove(&Edge { id: i, error: i % u }) {
    //         panic!("shit");
    //     }
    //     if !btreemap.remove(&Edge { id: i + 1, error: (i + 1) % u }) {
    //         panic!("shit");
    //     }
    //     btreemap.insert(Edge { id: i + 1, error: (i + 1) % u });
    // }
    // println!("len={}", btreemap.len());
    // println!("ms={}", now.elapsed().as_millis());
    // println!("min={}", btreemap.iter().next().unwrap().0.error);
    // println!("max={}", btreemap.iter().next_back().unwrap().0.error);

    // let now = Instant::now();
    // let mut btreemap = IndexSet::new();
    // const s: i32 = 1_000_000;
    // const u: i32 = 1_000;
    // for i in 0..s {
    //     btreemap.insert(Edge { id: i, error: i % u });
    // }
    // for i in 0..500_000 {
    //     if !btreemap.remove(&Edge { id: i, error: i % u }) {
    //         panic!("shit");
    //     }
    //     if !btreemap.remove(&Edge { id: i + 1, error: (i + 1) % u }) {
    //         panic!("shit");
    //     }
    //     btreemap.insert(Edge { id: i + 1, error: (i + 1) % u });
    //     btreemap.sort_by(|a, b| b.partial_cmp(a).unwrap());
    // }
    // println!("len={}", btreemap.len());
    // println!("ms={}", now.elapsed().as_millis());
    // println!("min={}", btreemap.first().unwrap().error);
    // println!("max={}", btreemap.last().unwrap().error);

    // let now = Instant::now();
    // let mut btreemap = PriorityQueue::new();
    // // let mut btreemap = PriorityQueue::with_hasher(
    // //     BuildHasherDefault::<SimpleHasher>::default()
    // // );
    // const s: i32 = 1_000_000;
    // const u: i32 = 1_000;
    // for i in 0..s {
    //     btreemap.push(MyEdge { a: i }, i);
    // }
    // for i in 0..500_000 {
    //     match btreemap.remove(&MyEdge { a: i }) {
    //         None => panic!("shit"),
    //         _ => ()
    //     };
    //     btreemap.push(MyEdge { a: i + 1 }, i - 124);
    // }
    // println!("len={}", btreemap.len());
    // println!("ms={}", now.elapsed().as_millis());
    // println!("min={}", btreemap.peek().unwrap().1);

    let mut mesh = ConnectedMesh::default();
    mesh.decimate();
}
