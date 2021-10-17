use crate::SymmetricMatrix;
use std::hash::Hash;
use priority_queue::PriorityQueue;
use hashbrown::HashSet;

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
            if self.nodes[i].is_removed {
                continue;
            }
            let edge = Edge::new(self.nodes[i] /* will copy */, self.nodes[self.nodes[i].relative as usize] /* will copy */); // Is it enough?
            queue.push(edge, Error(0.0));
            // let edge = Edge::new(self.nodes[self.nodes[i].relative as usize] /* will copy */, self.nodes[self.nodes[self.nodes[i].relative as usize].relative as usize] /* will copy */); // Is it enough?
            // queue.push(edge, Error(0.0));
            // Initialize quadrics per position
            if quadrics[self.nodes[i].position as usize].m[0] == -1.0 { // Todo: improve this (degeulassss)
                calculate_quadric(self, &mut quadrics, i as i32);
            }
        }
        println!("edges: {}", queue.len());

        // Initialize errors
        for x in &mut queue {
            if x.0.node_a.position == 40 || x.0.node_b.position == 40 {
                println!("edge in queue: {}", x.0);
            }
            calculate_error(self, &mut quadrics, x.0, x.1);
        }

        // Iterate
        while self.face_count > target_triangle_count {

            //println!("faces: {}", self.face_count);

            let mut edge_to_collapse = queue.pop().unwrap().0;

            //println!("collapse edge: {}", edge_to_collapse);

            // Repair
            loop_siblings!(edge_to_collapse.node_a.sibling, self.nodes, sibling, {
                let node = self.nodes[sibling as usize];
                if !node.is_removed {
                    edge_to_collapse.node_a = node;
                    break;
                }
            });

            loop_siblings!(edge_to_collapse.node_b.sibling, self.nodes, sibling, {
                let node = self.nodes[sibling as usize];
                if !node.is_removed {
                    edge_to_collapse.node_b = node;
                    break;
                }
            });

            //debug_assert!(self.print_siblings(edge_to_collapse.node_a.sibling));
            //debug_assert!(self.print_siblings(edge_to_collapse.node_b.sibling));

            loop_siblings!(edge_to_collapse.node_a.sibling, self.nodes, sibling, {
                let node = self.nodes[sibling as usize];
                let edge = Edge::new(node, self.nodes[node.relative as usize]);

                queue.remove(&edge);
                // match queue.remove(&edge) {
                //     None => println!("- already removed {}", edge),
                //     Some((item, prio)) => println!("- removed {}", edge)
                // }
            });

            loop_siblings!(edge_to_collapse.node_b.sibling, self.nodes, sibling, {
                let node = self.nodes[sibling as usize];
                let edge = Edge::new(node, self.nodes[node.relative as usize]);

                queue.remove(&edge);
                // match queue.remove(&edge) {
                //     None => println!("- already removed {}", edge),
                //     Some((item, prio)) => println!("- removed {}", edge)
                // }
            });
        
            //println!("face count {}", self.face_count);

            // Collapse edge
            //println!("start collapse");
            let valid_node_index = self.collapse_edge_to_a(edge_to_collapse.node_a.sibling, edge_to_collapse.node_b.sibling);
            //println!("end collapse");

            //println!("face count {}", self.face_count);

            // Recalculate quadric at A
            calculate_quadric(self, &mut quadrics, valid_node_index);

            loop_siblings!(valid_node_index, self.nodes, sibling, {
                let node = self.nodes[sibling as usize];
                let edge = &mut Edge::new(node, self.nodes[node.relative as usize]);

                // Recompute quadric
                calculate_quadric(self, &mut quadrics, edge.node_b.sibling);

                // Refresh edge in queue (new collapse target position)
                let error = &mut Error(0.);
                calculate_error(self, &mut quadrics, edge, error);

                // TODO: Avoid edge copy?
                queue.push(*edge, *error);
                // match queue.push(*edge, *error) { 
                //     None => println!("pushed {}", edge),
                //     Some(old) => println!("failed pushing {}", edge)
                // }
            });

            //panic!("end");
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