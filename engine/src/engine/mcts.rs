use crate::ataxx::position::Position;
use std::f32::ln;

const INFINITY: f32 = 1_000_000;
const C: f32 = 1.41421356237; // 2 / sqrt(2)
const NODEPOOL_MAX_MEM: usize = 2 * 1024 * 1024 * 1024; // 2GB

struct Node {
    idx: usize,
    parent: Option<usize>,
    children: Vec<usize>,
    visits: usize,
    total_value: f32,
    position: Position,
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new(self) -> Self {
        const NODEPOOL_SIZE: usize = NODEPOOL_MAX_MEM / std::mem::size_of::<Node>();
        let mut v = Vec::with_capacity(NODEPOOL_SIZE);

        let root = Node {
            idx: 0,
            parent: None,
            children: Vec::new(),
        };

        v.push(root);
    }
}

impl Node {
    fn ucb1(self) -> f32 {
        if self.visits == 0 {
            return INFINITY;
        }

        let exploitation = self.total_value / self.visits as f32;
        let exploration = C * sqrt((2 * self.total_visits().ln()) / self.visits as f32);
        let v = exploitation + exploration;

        return v;
    }

    fn total_visits(self) -> f32 {
        // Recursively travel up to the root and return it's visits
        match self.parent {
            Some(parent_idx) => {
                let parent = self.nodes[parent_idx];
                return parent.total_visits();
            }
            None => return self.visits as f32,
        }
    }

    fn select_child(self) -> Node {
        if self.children.len() == 0 {
            self.expand();
            return self.select_child();
        }

        let mut best_value = 0.0;
        let mut best_child = None;

        // ...
    }

    fn rollout(self) -> i32 {
        let mut position: Position = self.position.clone();

        while !position.game_over() {
            let moves = position.legal_moves();
            let random_move = moves[fastrand::usize(..moves.len())];
            position.make_move(random_move);
        }

        return position.winner();
    }

    fn expand(self) {
        panic!("Not implemented")
    }
}
