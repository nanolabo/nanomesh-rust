use crate::SymmetricMatrix;
use std::hash::Hash;
use priority_queue::PriorityQueue;
use hashbrown::HashSet;

include!("edge_hasher.rs");
include!("edge.rs");
include!("error.rs");

impl ConnectedMesh {    
    pub fn decimate_to_ratio(&mut self, target_triangle_ratio: f32) {
        self.decimate((target_triangle_ratio * self.face_count as f32) as u32);
    }

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
            let edge = Edge::new(self.nodes[i].position, self.nodes[self.nodes[i].relative as usize].position); // Is it enough?
            queue.push(edge, Error(0.0));
            // let edge = Edge::new(self.nodes[self.nodes[i].relative as usize] /* will copy */, self.nodes[self.nodes[self.nodes[i].relative as usize].relative as usize] /* will copy */); // Is it enough?
            // queue.push(edge, Error(0.0));
            // Initialize quadrics per position
            if quadrics[self.nodes[i].position as usize].m[0] == -1.0 { // Todo: improve this (degeulassss)
                calculate_quadric(self, &mut quadrics, i as i32);
            }
        }
        println!("edges: {}", queue.len());

        // Compute position to node mapping
        // let mut position_to_node = HashMap::<i32, i32, _>::with_hasher(
        //     BuildHasherDefault::<SimpleHasher>::default()
        // );
        let mut position_to_node = PosToNodeMap::new();
        for i in 0..self.nodes.len() {
            // TODO: Dont add when removed node
            position_to_node.insert(self.nodes[i as usize].position, i as i32);
        }

        // Initialize errors
        for x in &mut queue {
            calculate_error(self, &mut quadrics, x.0, x.1);
        }

        // Iterate
        while self.face_count > target_triangle_count {

            //println!("faces: {}", self.face_count);

            let edge_to_collapse = queue.pop().unwrap().0;

            //println!("collapse edge: {}", edge_to_collapse);

            //debug_assert!(self.print_siblings(edge_to_collapse.node_a.sibling));
            //debug_assert!(self.print_siblings(edge_to_collapse.node_b.sibling));

            let node_a = *position_to_node.get(&edge_to_collapse.pos_a).unwrap();
            loop_siblings!(node_a, self.nodes, sibling, {
                let node_a = self.nodes[sibling as usize];
                let node_c = self.nodes[node_a.relative as usize];
                let node_c2 = self.nodes[node_c.relative as usize];

                let edge = Edge::new(node_a.position, node_c.position);
                let edge2 = Edge::new(node_a.position, node_c2.position);

                queue.remove(&edge);
                queue.remove(&edge2);
                // match queue.remove(&edge) {
                //     None => println!("- already removed {}", edge),
                //     Some((item, prio)) => println!("- removed {}", edge)
                // }
            });

            let node_b = *position_to_node.get(&edge_to_collapse.pos_b).unwrap();
            loop_siblings!(node_b, self.nodes, sibling, {
                let node_b = self.nodes[sibling as usize];
                let node_c = self.nodes[node_b.relative as usize];
                let node_c2 = self.nodes[node_c.relative as usize];

                let edge = Edge::new(node_b.position, node_c.position);
                let edge2 = Edge::new(node_b.position, node_c2.position);

                queue.remove(&edge);
                queue.remove(&edge2);
                // match queue.remove(&edge) {
                //     None => println!("- already removed {}", edge),
                //     Some((item, prio)) => println!("- removed {}", edge)
                // }
            });
        
            //println!("face count {}", self.face_count);

            // Collapse edge
            //println!("start collapse");
            let valid_node_index = self.collapse_edge_to_a(*position_to_node.get(&edge_to_collapse.pos_a).unwrap(), *position_to_node.get(&edge_to_collapse.pos_b).unwrap(), &mut Some(&mut position_to_node));
            //println!("end collapse");

            if valid_node_index < 0
            {
                continue;
            }

            // Use optimal position
            self.positions[self.nodes[valid_node_index as usize].position as usize] = edge_to_collapse.collapse_to;

            //println!("face count {}", self.face_count);

            // Recalculate quadric at A
            calculate_quadric(self, &mut quadrics, valid_node_index);

            loop_siblings!(valid_node_index, self.nodes, sibling, {
                let node_a = self.nodes[sibling as usize];
                let node_c = self.nodes[node_a.relative as usize];
                let edge = &mut Edge::new(node_a.position, node_c.position);

                // Recompute quadric
                calculate_quadric(self, &mut quadrics, node_c.sibling);

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

            let pos_a = &connected_mesh.positions[edge.pos_a as usize];
            let pos_b = &connected_mesh.positions[edge.pos_b as usize];
            let pos_c = &(&(pos_a + pos_b) / 2.0);

            let matrix = &quadrics[edge.pos_a as usize] + &quadrics[edge.pos_b as usize];

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