use crate::SymmetricMatrix;
use std::hash::Hash;
use priority_queue::PriorityQueue;

include!("edge_hasher.rs");
include!("edge.rs");
include!("error.rs");

impl ConnectedMesh {    
    pub fn decimate(&mut self, target_triangle_count: u32) {

        let mut queue = PriorityQueue::<Edge, Error, _>::with_hasher(
            BuildHasherDefault::<EdgeHasher>::default()
        );
        let mut quadrics = vec![SymmetricMatrix::default_uninitalized(); self.positions.len()];

        // match queue.get_mut(&item) {
        //     None => panic!(":cnon:"),
        //     Some((item, prio)) =>  {
        //         item.node_a.normal = 2;
        //     }
        // };

        fn collapse_edge(edge: &Edge) {

        }

        let calculate_quadric = |node_index: i32| {
            let mut matrix =  SymmetricMatrix::default_zeroes();
            loop_siblings!(node_index, self.nodes, sibling, {
                // todo
            });
        };

        let calculate_error = |edge: &Edge| {
            return Error(0.0);
        };

        // Initialize
        for i in 0..self.nodes.len() {
            // Todo: Ignore removed nodes
            let edge = Edge { node_a: self.nodes[i] /* will copy */, node_b: self.nodes[self.nodes[i].relative as usize] /* will copy */ }; // Is it enough?
            queue.push(edge, Error(0.0));
            // Initialize quadrics per position
            if quadrics[self.nodes[i].position as usize].m[0] == -1.0 { // Todo: improve this (degeulassss)
                calculate_quadric(i as i32);
            }
        }
        println!("Edges: {}", queue.len());

        // Initialize errors
        for x in &mut queue {
            let error = calculate_error(&x.0);
            *x.1 = error;
        }

        // Iterate
        while self.face_count > target_triangle_count {
            let edge_to_collapse = queue.pop().unwrap().0;
            collapse_edge(&edge_to_collapse);
        }
    }
}