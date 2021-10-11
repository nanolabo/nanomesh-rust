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
            let edge = Edge::new(self.nodes[i] /* will copy */, self.nodes[self.nodes[i].relative as usize] /* will copy */); // Is it enough?
            queue.push(edge, Error(0.0));
            // Initialize quadrics per position
            if quadrics[self.nodes[i].position as usize].m[0] == -1.0 { // Todo: improve this (degeulassss)
                calculate_quadric(self, &mut quadrics, i as i32);
            }
        }
        println!("Edges: {}", queue.len());

        // Initialize errors
        for x in &mut queue {
            calculate_error(self, &mut quadrics, x.0, x.1);
        }

        // Iterate
        while self.face_count > target_triangle_count {
            let edge_to_collapse = queue.pop().unwrap().0;
            
            let mut nodes_indices = Vec::<i32>::with_capacity(20); // Later change for stack-allocated array
            loop_siblings!(edge_to_collapse.node_a.sibling, self.nodes, sibling, {
                nodes_indices.push(sibling);
            });
            loop_siblings!(edge_to_collapse.node_b.sibling, self.nodes, sibling, {
                nodes_indices.push(sibling);
            });
            // Collapse edge
            self.collapse_edge(edge_to_collapse.node_a.sibling, edge_to_collapse.node_b.sibling);
            // Update edges and errors
            for node_index in nodes_indices {
                let node = self.nodes[node_index as usize];
                let edge = &mut Edge::new(node, self.nodes[node.relative as usize]);
                if node.is_removed {
                    queue.remove(edge);
                } else {
                    // Recompute quadric
                    calculate_quadric(self, &mut quadrics, node.relative);
                    // Refresh edge in queue (new collapse target position)
                    let error = &mut Error(0.);
                    match queue.get_mut(&edge) {
                        None => panic!("Should not happen!"),
                        Some((edge_in_place, _)) =>  {
                            calculate_error(self, &mut quadrics, edge_in_place, error);
                            edge_in_place.node_a = edge.node_a;
                            edge_in_place.node_b = edge.node_b;
                        }
                    };
                    // Refresh error in queue (priority)
                    queue.change_priority(edge, *error);
                }
            }
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

        fn calculate_error(connected_mesh: &mut ConnectedMesh, quadrics: &mut Vec<SymmetricMatrix>, edge: &mut Edge, error: &mut Error) {

            let pos_a = &connected_mesh.positions[edge.node_a.position as usize];
            let pos_b = &connected_mesh.positions[edge.node_b.position as usize];
            let pos_c = &(&(pos_a + pos_b) / 2.0);

            let matrix = &quadrics[edge.node_a.position as usize] + &quadrics[edge.node_b.position as usize];

            let det = matrix.get_det_xyz();

            let (error_o, pos_o) = &
            if det > 0.001 || det < -0.001 {
                let pos = Vector3::new(
                    -1.0 / det * matrix.get_det_x(),
                     1.0 / det * matrix.get_det_y(),
                    -1.0 / det * matrix.get_det_z());
                (matrix.quadric_distance_to_vertex(&pos), pos)
            } else {
                (f64::MAX, Vector3::default())
            };

            let error_a = matrix.quadric_distance_to_vertex(&pos_a);
            let error_b = matrix.quadric_distance_to_vertex(&pos_b);
            let error_c = matrix.quadric_distance_to_vertex(&pos_c);

            macro_rules! min {
                ($lerror_a: expr, $lpos_a: expr, $($lerror_b: expr, $lpos_b: expr),+) => {
                    {
                        let mut lerror = $lerror_a;
                        let mut lpos = $lpos_a;
                        $({
                            if $lerror_b < lerror {
                                lerror = $lerror_b;
                                lpos = $lpos_b;
                            }
                        })*
                        (lerror, lpos)
                    }
                }
            }

            let (xerror, xpos) = min!(*error_o, pos_o, error_a, pos_a, error_b, pos_b, error_c, pos_c);

            error.0 = -xerror; // Negative is a small hack because PriorityQueue is max based, but we want min
            edge.collapse_to = *xpos;
        }
    }
}