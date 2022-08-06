use std::collections::HashMap;

#[derive(Debug)]
pub struct Trie {
    pub nodes: Vec<Node>,
    pub root: i32,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            nodes: vec![Node::new(0)],
            root: 0,
        }
    }

    pub fn insert(&mut self, string: &str, word_id: i32) {
        let mut node_id = 0_i32;
        for c in string.chars() {
            let next_node_id = self.nodes[node_id as usize].next_node_id.get(&c);
            let next_node_id = if let Some(next_node_id) = next_node_id {
                *next_node_id
            } else {
                let next_node_id = self.nodes.len() as i32;
                self.nodes.push(Node::new(next_node_id));
                next_node_id
            };

            self.nodes[node_id as usize].number_of_common_string += 1;
            self.nodes[node_id as usize]
                .next_node_id
                .insert(c, next_node_id);
            node_id = next_node_id;
        }
        self.nodes[node_id as usize].number_of_common_string += 1;
        self.nodes[node_id as usize].end_string_id.push(word_id);
    }

    pub fn search(&self, string: &str, prefix: bool) -> bool {
        let mut node_id = 0_i32;
        for c in string.chars() {
            let next_id = self.nodes[node_id as usize].next_node_id.get(&c);
            if let Some(next_id) = next_id {
                node_id = *next_id;
            } else {
                return false;
            }
        }
        !self.nodes[node_id as usize].end_string_id.is_empty()
    }
}

#[derive(Debug)]
pub struct Node {
    pub next_node_id: HashMap<char, i32>,
    pub end_string_id: Vec<i32>,
    pub distance_from_root: i32,
    pub number_of_common_string: i32,
}

impl Node {
    pub fn new(distace: i32) -> Self {
        Node {
            next_node_id: HashMap::new(),
            end_string_id: vec![],
            distance_from_root: distace,
            number_of_common_string: 0,
        }
    }
}
