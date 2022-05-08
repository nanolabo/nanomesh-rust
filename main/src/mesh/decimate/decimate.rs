use super::base::SymmetricMatrix;

use std::hash::Hash;
use priority_queue::PriorityQueue;
use hashbrown::HashSet;
use pool::Pool;

include!("edge.rs");
include!("collapse_context.rs");

impl ConnectedMesh {    
    pub fn decimate_to_ratio(&mut self, target_triangle_ratio: f32) {
        self.decimate((target_triangle_ratio * self.face_count as f32) as u32);
    }

    pub fn decimate(&mut self, target_triangle_count: u32) {

        macro_rules! loop_edges {
            ($node_index:expr, $edge_buffer:expr,$nodes:expr, $relative:ident, $exec:expr) => {{
                $edge_buffer.clear();
                loop_siblings!($node_index, $nodes, sibling, {
                    let mut $relative: u32 = sibling;
                    loop {
                        $relative = $nodes[$relative as usize].relative;
                        if $relative == sibling {
                            break;
                        }
                        if $edge_buffer.insert($nodes[$relative as usize].position) {
                            $exec
                        }
                    }
                });
            }};
        }

        let mut queue = PriorityQueue::<Edge, CollapseContext, _>::with_hasher(BuildHasherDefault::<SimpleHasher>::default());
        let mut position_to_node = U32Map::with_hasher(BuildHasherDefault::<SimpleHasher>::default());
        let mut quadrics = vec![SymmetricMatrix::default_uninitalized(); self.positions.len()];

        let mut pool = Pool::with_capacity(20, 0, || U32Set::with_hasher(BuildHasherDefault::<SimpleHasher>::default()) /* SetU32::new() */);

        for i in 0..self.nodes.len() {
            if self.nodes[i].is_removed {
                continue;
            }
            // TODO: Check if enough? Maybe there is a better loop for this purpose
            let edge = Edge::new(self.nodes[i].position, self.nodes[self.nodes[i].relative as usize].position);
            queue.push(edge, CollapseContext::default());
            position_to_node.insert(self.nodes[i as usize].position, i as u32);
        }

        // Initialize quadrics
        for pos_to_node in &position_to_node {
            calculate_quadric(self, &mut quadrics, *pos_to_node.1);
        }

        // Initialize weights
        for x in &mut queue {
            calculate_weight(self, &position_to_node, x.0, x.1);
        }

        // Initialize errors
        {
            let mut collapse_contexts = Vec::<CollapseContext>::with_capacity(queue.len());
        
            for x in &queue {
                let mut collapse_context = CollapseContext::default();
                calculate_error(self, &mut quadrics, &queue, &position_to_node, &mut pool.checkout().unwrap(), x.0, &mut collapse_context);
                collapse_contexts.push(collapse_context);
            }
    
            let mut i: usize = 0;
            for x in &mut queue {
                x.1.error = collapse_contexts[i].error;
                x.1.collapse_to = collapse_contexts[i].collapse_to;
                i += 1;
            }
        }

        // Iterate
        while self.face_count > target_triangle_count {

            let pair_to_collapse = queue.pop().unwrap();
            let edge_to_collapse = pair_to_collapse.0;
            let collapse_context = pair_to_collapse.1;

            match position_to_node.get(&edge_to_collapse.pos_a) {
                Some(_) => (),
                None => continue
            };

            match position_to_node.get(&edge_to_collapse.pos_b) {
                Some(_) => (),
                None => continue
            };
        
            // Collapse edge
            let valid_node_index_o = self.collapse_edge_to_a(*position_to_node.get(&edge_to_collapse.pos_a).unwrap(), *position_to_node.get(&edge_to_collapse.pos_b).unwrap(), &mut Some(&mut position_to_node));

            if valid_node_index_o.is_none() {
                continue;
            }

            let valid_node_index = valid_node_index_o.unwrap();

            // Use optimal position
            self.positions[self.nodes[valid_node_index as usize].position as usize] = collapse_context.collapse_to;

            // Recalculate quadric at A
            calculate_quadric(self, &mut quadrics, valid_node_index);

            let node_a = self.nodes[valid_node_index as usize];

            let mut positions = pool.checkout().unwrap();

            loop_edges!(valid_node_index, positions, self.nodes, relative, {
                let node_c = self.nodes[relative as usize];
                let edge = &Edge::new(node_a.position, node_c.position);
                // Recompute quadric
                calculate_quadric(self, &mut quadrics, node_c.sibling);
                // Refresh edge in queue (new collapse target position)
                let mut collapse_context = CollapseContext::default();
                queue.push(*edge, collapse_context);
                calculate_weight(self, &position_to_node, edge, &mut collapse_context);
            });

            for position in positions.iter() {
                debug_assert!(node_a.position != *position);
                let edge = &Edge::new(node_a.position, *position);
                // Refresh edge in queue (new collapse target position)
                let mut collapse_context = *queue.get(&edge).unwrap().1;
                calculate_error(self, &mut quadrics, &queue, &position_to_node, &mut pool.checkout().unwrap(), edge, &mut collapse_context);
                queue.change_priority(edge, collapse_context);
            }
        }

        fn calculate_quadric(connected_mesh: &mut ConnectedMesh, quadrics: &mut Vec<SymmetricMatrix>, node_index: u32)
        {
            let mut matrix = SymmetricMatrix::default_zeroes();

            loop_siblings!(node_index, connected_mesh.nodes, sibling, {
                let face_normal = &connected_mesh.get_face_normal(sibling);
                let position = &connected_mesh.positions[connected_mesh.nodes[sibling as usize].position as usize];
                let dot = &-face_normal.dot(position);
                matrix += SymmetricMatrix::from_normal(face_normal, &dot);
            });
            quadrics[connected_mesh.nodes[node_index as usize].position as usize] = matrix;

            // TODO: Take surface area into consideration
            // TODO: "For each face adjacent to a given boundary edge, we compute a plane through the edge that is perpendicular to the face"
        }

        fn calculate_weight(connected_mesh: &ConnectedMesh, position_to_node: &U32Map, edge: &Edge, collapse_context: &mut CollapseContext)
        {
            let node_a = *position_to_node.get(&edge.pos_a).unwrap();
            let node_b = *position_to_node.get(&edge.pos_b).unwrap();

            collapse_context.weight = connected_mesh.get_edge_topo(node_a, node_b);
        }

        fn calculate_error(connected_mesh: &mut ConnectedMesh, quadrics: &mut Vec<SymmetricMatrix>, queue: &PriorityQueue::<Edge, CollapseContext, BuildHasherDefault<SimpleHasher>>, position_to_node: &U32Map, edge_buffer: &mut U32Set, edge: &Edge, collapse_context: &mut CollapseContext)
        {
            let pos_a = &connected_mesh.positions[edge.pos_a as usize];
            let pos_b = &connected_mesh.positions[edge.pos_b as usize];
            let pos_c = &(&(pos_a + pos_b) / 2.0);

            //let node_a = *position_to_node.get(&edge.pos_a).unwrap();
            //let node_b = *position_to_node.get(&edge.pos_b).unwrap();

            let matrix = &quadrics[edge.pos_a as usize] + &quadrics[edge.pos_b as usize];

            let det = matrix.get_det_xyz();

            let (error_o, pos_o) = &
            if det > 0.001 || det < -0.001 {
                let pos = DVec3::new(
                    -1.0 / det * matrix.get_det_x(),
                     1.0 / det * matrix.get_det_y(),
                    -1.0 / det * matrix.get_det_z());
                (matrix.quadric_distance_to_vertex(&pos), pos)
            } else {
                (f64::MAX, DVec3::default())
            };

            let mut error_a = matrix.quadric_distance_to_vertex(&pos_a);
            let mut error_b = matrix.quadric_distance_to_vertex(&pos_b);
            let mut error_c = matrix.quadric_distance_to_vertex(&pos_c);

            // We multiply by edge length to be agnotics with quadrics error.
            // Otherwise it becomes too scale dependent
            //let length = (pos_b - pos_a).magnitude();

            // loop_edges!(node_a, edge_buffer, &connected_mesh.nodes, relative, {
            //     let pos_d_index = connected_mesh.nodes[relative as usize].position;
            //     let pos_d = &connected_mesh.positions[pos_d_index as usize];
            //     let weight = queue.get(&Edge::new(edge.pos_a, pos_d_index)).unwrap().1.weight;
            //     error_b += weight * length * pos_a.distance_to_line(pos_b, pos_d);
            //     error_c += weight * length * pos_a.distance_to_line(pos_c, pos_d);
            // });

            // loop_edges!(node_b, edge_buffer, &connected_mesh.nodes, relative, {
            //     let pos_d_index = connected_mesh.nodes[relative as usize].position;
            //     let pos_d = &connected_mesh.positions[pos_d_index as usize];
            //     let weight = queue.get(&Edge::new(edge.pos_b, pos_d_index)).unwrap().1.weight;
            //     error_a += weight * length * pos_b.distance_to_line(pos_a, pos_d);
            //     error_c += weight * length * pos_b.distance_to_line(pos_c, pos_d);
            // });

            error_c *= 0.4716252;

            let (xerror, xpos) = min!(*error_o, pos_o, error_a, pos_a, error_b, pos_b, error_c, pos_c);

            collapse_context.error = -xerror; // Negative is a small hack because PriorityQueue is max based, but we want min
            collapse_context.collapse_to = *xpos;
        }
    }
}