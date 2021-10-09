pub struct ConnectedMesh {
    nodes: Vec<Node>,
    face_count: u32,

    positions: Vec<Vector3>,
    normals: Vec<Vector3>,
    // uv0: Vec<Vector3>,
    // colors: Vec<Vector3>,
}

impl Default for ConnectedMesh {
    fn default() -> ConnectedMesh {
        ConnectedMesh { 
            positions: Vec::new(),
            normals: Vec::new(),
            nodes: Vec::new(),
            face_count: 0 }
    }
}

macro_rules! loop_relatives {
    ($node_index:expr, $nodes:expr, $relative:ident, $exec:expr) => {{
        let mut $relative: i32 = $node_index;
        loop {
            $exec
            $relative = $nodes[$relative as usize].relative;
            if $relative == $node_index {
                break;
            }
        }
    }};
}

macro_rules! loop_siblings {
    ($node_index:expr, $nodes:expr, $sibling:ident, $exec:expr) => {{
        let mut $sibling: i32 = $node_index;
        loop {
            $exec
            $sibling = $nodes[$sibling as usize].sibling;
            if $sibling == $node_index {
                break;
            }
        }
    }};
}

use std::hash::Hash;
use std::time::Instant;
use priority_queue::PriorityQueue;

impl ConnectedMesh {    
    pub fn decimate(&mut self) {

        use std::hash::Hasher;
        pub struct EdgeHasher {
            hash: u64,
        }

        impl Default for EdgeHasher {
            fn default() -> EdgeHasher {
                EdgeHasher { hash: 0 }
            }
        }

        impl Hasher for EdgeHasher {
            #[inline]
            fn finish(&self) -> u64 {
                self.hash
            }
            #[inline]
            fn write(&mut self, _bytes: &[u8]) { 
            }
            #[inline]
            fn write_i32(&mut self, i: i32) {
                self.hash = self.hash + i as u64;
            }
        }

        pub struct Edge {
            node_a: Node,
            node_b: Node,
        }

        impl Eq for Edge {

        }
        
        impl PartialEq for Edge {
            fn eq(&self, other: &Self) -> bool {
                self.node_a.position == other.node_a.position
             && self.node_b.position == other.node_b.position
            }
        }

        impl Hash for Edge {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.node_a.position.hash(state);
                self.node_b.position.hash(state);
            }
        }

        let now = Instant::now();
        //let mut btreemap = PriorityQueue::new();
        let mut queue = PriorityQueue::with_hasher(
            BuildHasherDefault::<EdgeHasher>::default()
        );
        const s: i32 = 1000000;
        for i in 0..s {
            queue.push(Edge {
                node_a: Node { position: i, normal: 0, relative: 0, sibling: 0, is_removed: false },
                node_b: Node { position: i + 10000000, normal: 0, relative: 0, sibling: 0, is_removed: false } }, i);
        }
        for i in 0..1000000 {
            // Pop highest priority
            //queue.pop(); 
            // Update some

            let item = Edge {
                node_a: Node { position: i, normal: 1, relative: 0, sibling: 0, is_removed: false },
                node_b: Node { position: i + 10000000, normal: 1, relative: 0, sibling: 0, is_removed: false } };

            match queue.get_mut(&item) {
                None => panic!(":cnon:"),
                Some((item, prio)) =>  {
                    item.node_a.normal = 2;
                }
            };
            queue.change_priority(&item, i + 1);
        }
        println!("len={}", queue.len());
        println!("ms={}", now.elapsed().as_millis());
        println!("max={}", queue.peek().unwrap().1);
        println!("normal={}", queue.peek().unwrap().0.node_a.normal);
    }

    fn collapse_edge(&mut self, node_index_a: i32, node_index_b: i32) {

        let pos_a = self.nodes[node_index_a as usize].position;
        let pos_b = self.nodes[node_index_b as usize].position;

        debug_assert!(pos_a != pos_a);

        loop_siblings!(node_index_a, self.nodes, sibling_of_a, {

            let mut is_face_touched = false;
            let mut face_edge_count = 0;
            let mut node_index_c = -1;

            loop_relatives!(sibling_of_a, self.nodes, relative_of_a, {
                let pos_c = self.nodes[relative_of_a as usize].position;
                if pos_b == pos_c {
                    is_face_touched = true;
                } else if pos_a != pos_c {
                    node_index_c = relative_of_a;
                }
                face_edge_count = face_edge_count + 1;
            });

            debug_assert!(face_edge_count == 3);

            if is_face_touched {
                loop_relatives!(sibling_of_a, self.nodes, relative_of_a, {
                    self.nodes[relative_of_a as usize].is_removed = true;
                });
                let valid_node_at_c = self.reconnect_sibling(node_index_c);
                //let pos_c = self.nodes[node_index_c as usize].position;
                //update position_to_nodes
                self.face_count = self.face_count - 1;
            }
        });

        let valid_node_at_a = self.reconnect_siblings(node_index_a, node_index_b, pos_a);
        //update position_to_nodes
    }

    fn reconnect_siblings(&mut self, node_index_a: i32, node_index_b: i32, position: i32) -> i32 {
        let mut last_valid = -1;
        let mut first_valid = -1;

        loop_siblings!(node_index_a, self.nodes, sibling, {

            if self.nodes[sibling as usize].is_removed {
                continue;
            }
            if first_valid == -1 {
                first_valid = sibling;
            }
            if last_valid != -1 {
                self.nodes[sibling as usize].sibling = sibling;
                self.nodes[sibling as usize].position = position;
            }
            last_valid = sibling;
        });

        loop_siblings!(node_index_b, self.nodes, sibling, {

            if self.nodes[sibling as usize].is_removed {
                continue;
            }
            if first_valid == -1 {
                first_valid = sibling;
            }
            if last_valid != -1 {
                self.nodes[sibling as usize].sibling = sibling;
                self.nodes[sibling as usize].position = position;
            }
            last_valid = sibling;
        });

        if last_valid == -1 {
            return -1; // All siblings were removed
        }

        // Close the cloop
        self.nodes[last_valid as usize].sibling = first_valid;
        self.nodes[last_valid as usize].position = position;

        return first_valid;
    }

    fn reconnect_sibling(&mut self, node_index: i32) -> i32 {
        let mut last_valid = -1;
        let mut first_valid = -1;
        let mut position = -1;

        loop_siblings!(node_index, self.nodes, sibling, {

            if self.nodes[sibling as usize].is_removed {
                continue;
            }
            if first_valid == -1 {
                first_valid = sibling;
                position = self.nodes[sibling as usize].position;
            }
            if last_valid != -1 {
                self.nodes[sibling as usize].sibling = sibling;
                self.nodes[sibling as usize].position = position;
            }
            last_valid = sibling;
        });

        if last_valid == -1 {
            return -1; // All siblings were removed
        }

        // Close the cloop
        self.nodes[last_valid as usize].sibling = first_valid;
        self.nodes[last_valid as usize].position = position;

        return first_valid;
    }

    fn get_edge_topo(&mut self, node_index_a: i32, node_index_b: i32) -> f64 {
        let pos_b = self.nodes[node_index_b as usize].position;
        let mut faces_attached = 0;
        let mut attribute_at_a: i32 = -1;
        let mut attribute_at_b: i32 = -1;
        let mut edge_weight = 0.0;
        
        loop_siblings!(node_index_a, self.nodes, sibling_of_a, {
            if self.nodes[sibling_of_a as usize].is_removed {
                continue;
            }
            loop_relatives!(sibling_of_a, self.nodes, relative_of_a, {
                let pos_c = self.nodes[relative_of_a as usize].position;
                if pos_c == pos_b {
                    faces_attached = faces_attached + 1;

                    if self.normals.len() > 0 {
                        if attribute_at_b != -1 && self.normals[attribute_at_b as usize] == self.normals[self.nodes[relative_of_a as usize].normal as usize] {
                            edge_weight = edge_weight + 10.0
                        }
                        if attribute_at_a != -1 && self.normals[attribute_at_a as usize] == self.normals[self.nodes[sibling_of_a as usize].normal as usize] {
                            edge_weight = edge_weight + 10.0
                        }
                    }

                    attribute_at_b = self.nodes[relative_of_a as usize].normal;
                    attribute_at_a = self.nodes[sibling_of_a as usize].normal;
                }
            });
        });

        // Check if border
        if faces_attached < 2 {
            edge_weight = edge_weight + 100.0;
        }

        return edge_weight;
    }
}

pub struct Group {
    first_index: i32,
    index_count: i32,
}

#[derive(Clone)]
pub struct Node {
    sibling: i32,
    relative: i32,

    position: i32,
    normal: i32,
    // uv0: i32,
    // color: i32,

    is_removed: bool,
}

impl Default for Node {
    fn default() -> Self {
        Node { position: 0, normal: 0, relative: 0, sibling: 0, is_removed: false }
    }
}