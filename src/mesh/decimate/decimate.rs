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

        // Initialize
        for i in 0..self.nodes.len() {
            // Todo: Ignore removed nodes
            let edge = Edge { node_a: self.nodes[i] /* will copy */, node_b: self.nodes[self.nodes[i].relative as usize] /* will copy */ }; // Is it enough?
            queue.push(edge, Error(0.0));
            // Initialize quadrics per position
            if quadrics[self.nodes[i].position as usize].m[0] == -1.0 { // Todo: improve this (degeulassss)
                calculate_quadric(self, &mut quadrics, i as i32);
            }
        }
        println!("Edges: {}", queue.len());

        // Initialize errors
        for x in &mut queue {
            let error = calculate_error(self, &mut quadrics, &x.0);
            *x.1 = error;
        }

        // Iterate
        while self.face_count > target_triangle_count {
            let edge_to_collapse = queue.pop().unwrap().0;
            collapse_edge(&edge_to_collapse);
        }

        // match queue.get_mut(&item) {
        //     None => panic!(":cnon:"),
        //     Some((item, prio)) =>  {
        //         item.node_a.normal = 2;
        //     }
        // };

        fn collapse_edge(edge: &Edge) {

        }

        fn calculate_quadric(connected_mesh: &mut ConnectedMesh, quadrics: &mut Vec<SymmetricMatrix>, node_index: i32) {
            let mut matrix = SymmetricMatrix::default_zeroes();
            loop_siblings!(node_index, connected_mesh.nodes, sibling, {
                let face_normal = &connected_mesh.get_face_normal(sibling);
                let position = &connected_mesh.positions[connected_mesh.nodes[sibling as usize].position as usize];
                let dot = &-face_normal * position;
                matrix += SymmetricMatrix::from_normal(face_normal, &dot);
            });
            quadrics[connected_mesh.nodes[node_index as usize].position as usize] = matrix;
        }

        fn calculate_error(connected_mesh: &mut ConnectedMesh, quadrics: &mut Vec<SymmetricMatrix>, edge: &Edge) -> Error {

            let pos_a = &connected_mesh.positions[edge.node_a.position as usize];
            let pos_b = &connected_mesh.positions[edge.node_b.position as usize];
            let pos_c = &(pos_a + pos_b) / 2.0;

            let matrix = &quadrics[edge.node_a.position as usize] + &quadrics[edge.node_b.position as usize];

            let det = matrix.get_det_xyz();

            let mut error_o = f64::MAX;
            let mut pos_o = Vector3::default();

            if det > 0.001 || det < -0.001 {
                pos_o.x = -1.0 / det * matrix.get_det_x();
                pos_o.y =  1.0 / det * matrix.get_det_y();
                pos_o.z = -1.0 / det * matrix.get_det_z();
                error_o = matrix.quadric_distance_to_vertex(&pos_o);
            }

            let error_a = matrix.quadric_distance_to_vertex(&pos_a);
            let error_b = matrix.quadric_distance_to_vertex(&pos_b);
            let error_c = matrix.quadric_distance_to_vertex(&pos_c);

            macro_rules! min {
                ($error_a: expr, $pos_a: expr, $($error_b: expr, $pos_b: expr),+) => {
                    {
                        let mut error = error_a;
                        let mut pos = pos_a;
                        $({
                            if $error_b < error {
                                error = error_b;
                                pos = pos_b;
                            }
                        })*
                        (error, pos_b)
                    }
                }
            }

            let (error, pos) = min!(error_o, pos_o, error_a, pos_a, error_b, pos_b, error_c, pos_c);

            return Error(0.0);
        }
    }
}