use crate::SymmetricMatrix;

use std::hash::Hash;
use priority_queue::PriorityQueue;
use hashbrown::HashSet;

include!("edge_hasher.rs");
include!("edge.rs");
include!("error.rs");

struct DecimationContext<'a> {
    mesh: &'a mut ConnectedMesh,
    edge_buffer: HashSet::<i32, BuildHasherDefault<SimpleHasher>>,
    queue: PriorityQueue::<Edge, Error, BuildHasherDefault<EdgeHasher>>,
    quadrics: &'a mut Vec<SymmetricMatrix>,
}

impl ConnectedMesh {    
    pub fn decimate_to_ratio(&mut self, target_triangle_ratio: f32) {
        self.decimate((target_triangle_ratio * self.face_count as f32) as u32);
    }

    pub fn decimate(&mut self, target_triangle_count: u32) {

        let mut context = DecimationContext {
            mesh: self,
            edge_buffer: HashSet::<i32, _>::with_hasher(BuildHasherDefault::<SimpleHasher>::default()),
            queue: PriorityQueue::<Edge, Error, _>::with_hasher(BuildHasherDefault::<EdgeHasher>::default()),
            quadrics: &mut vec![SymmetricMatrix::default_uninitalized(); /*self.positions.len()*/ 10],
        };

        // Initialize
        for i in 0..context.mesh.nodes.len() {
            if context.mesh.nodes[i].is_removed {
                continue;
            }
            let edge = Edge::new(context.mesh.nodes[i].position, context.mesh.nodes[context.mesh.nodes[i].relative as usize].position); // Is it enough?
            context.queue.push(edge, Error(0.0));
            // let edge = Edge::new(self.nodes[self.nodes[i].relative as usize] /* will copy */, self.nodes[self.nodes[self.nodes[i].relative as usize].relative as usize] /* will copy */); // Is it enough?
            // queue.push(edge, Error(0.0));
            // Initialize quadrics per position
            if context.quadrics[context.mesh.nodes[i].position as usize].m[0] == -1.0 { // Todo: improve this (degeulassss)
                calculate_quadric(&mut context, i as i32);
            }
        }

        // Compute position to node mapping
        let mut position_to_node = HashMap::<i32, i32, _>::with_hasher(
            BuildHasherDefault::<SimpleHasher>::default()
        );

        for i in 0..context.mesh.nodes.len() {
            // TODO: Dont add when removed node
            position_to_node.insert(context.mesh.nodes[i as usize].position, i as i32);
        }

        // Initialize errors
        let mut queueeee = context.queue;
        for x in &mut queueeee {
            calculate_error(&context, x.0, x.1);
        }

        // Iterate
        while context.mesh.face_count > target_triangle_count {

            let edge_to_collapse = queueeee.pop().unwrap().0;

            // There are many different approaches:
            // A: If edge A or B position doesn't exist anymore, pass ton chemin (it gets popped anyway by queue.pop())
            //    If edge is valid, update target collapse point and error 
            // B: ?

            match position_to_node.get(&edge_to_collapse.pos_a) {
                Some(_) => (),
                None => continue
            };

            match position_to_node.get(&edge_to_collapse.pos_b) {
                Some(_) => (),
                None => continue
            };
        
            // Collapse edge
            let valid_node_index = context.mesh.collapse_edge_to_a(*position_to_node.get(&edge_to_collapse.pos_a).unwrap(), *position_to_node.get(&edge_to_collapse.pos_b).unwrap(), &mut Some(&mut position_to_node));

            if valid_node_index < 0 {
                continue;
            }

            // Use optimal position
            context.mesh.positions[context.mesh.nodes[valid_node_index as usize].position as usize] = edge_to_collapse.collapse_to;

            // Recalculate quadric at A
            calculate_quadric(&mut context, valid_node_index);

            loop_siblings!(valid_node_index, context.mesh.nodes, sibling, {
                let node_a = context.mesh.nodes[sibling as usize];
                let node_c = context.mesh.nodes[node_a.relative as usize];
                let edge = &mut Edge::new(node_a.position, node_c.position);

                // Recompute quadric
                calculate_quadric(&mut context, node_c.sibling);

                // Refresh edge in queue (new collapse target position)
                let error = &mut Error(0.);
                calculate_error(&context, edge, error);

                match queueeee.get_mut(edge) {
                    Some((item, _)) => {
                        item.collapse_to = edge.collapse_to;
                    },
                    None => ()
                }

                queueeee.change_priority(edge, *error);
            });
        }

        // macro_rules! loop_edges {
        //     ($node_index:expr, $nodes:expr, $relative:ident, $exec:expr) => {{
        //         adjacent_edges_buffer.clear();
        //         loop_siblings!($node_index, $nodes, sibling, {
        //             let mut $relative: i32 = sibling;
        //             loop {
        //                 $relative = $nodes[$relative as usize].relative;
        //                 if $relative == $node_index {
        //                     break;
        //                 }
        //                 if adjacent_edges_buffer.insert($nodes[$relative as usize].position) {
        //                     $exec
        //                 }
        //             }
        //         });
        //     }};
        // }

        fn calculate_quadric(context: &mut DecimationContext, node_index: i32) {
            let mut matrix = SymmetricMatrix::default_zeroes();
            loop_siblings!(node_index, context.mesh.nodes, sibling, {
                let face_normal = &context.mesh.get_face_normal(sibling);
                let position = &context.mesh.positions[context.mesh.nodes[sibling as usize].position as usize];
                let dot = &-face_normal * position;
                matrix += SymmetricMatrix::from_normal(face_normal, &dot);
            });
            //context.quadrics[context.mesh.nodes[node_index as usize].position as usize] = matrix;
        }

        fn calculate_error(context: &DecimationContext, edge: &mut Edge, error: &mut Error) {

            let pos_a = &context.mesh.positions[edge.pos_a as usize];
            let pos_b = &context.mesh.positions[edge.pos_b as usize];
            let pos_c = &(&(pos_a + pos_b) / 2.0);

            // let node_a = *position_to_node.get(&edge.pos_a).unwrap();
            // let node_b = *position_to_node.get(&edge.pos_b).unwrap();

            let matrix = &context.quadrics[edge.pos_a as usize] + &context.quadrics[edge.pos_b as usize];

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

            // We multiply by edge length to be agnotics with quadrics error.
            // Otherwise it becomes too scale dependent
            let length = (pos_b - pos_a).magnitude();

            // println!("edges");
            // loop_edges!(node_a, context.connected_mesh.nodes, relative, {
            //     println!("edge end: {}", context.connected_mesh.nodes[relative as usize].position);
            // });

            let (xerror, xpos) = min!(*error_o, pos_o, error_a, pos_a, error_b, pos_b, error_c, pos_c);

            error.0 = -xerror; // Negative is a small hack because PriorityQueue is max based, but we want min
            edge.collapse_to = *xpos;
        }
    }
}