use crate::ataxx::position::{Outcome, Position, Side};
use std::time::Instant;

const INFINITY: f32 = 1_000_000.0;
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
    fn new(self, position: Position) -> Self {
        const NODEPOOL_SIZE: usize = NODEPOOL_MAX_MEM / std::mem::size_of::<Node>();
        let mut v = Vec::with_capacity(NODEPOOL_SIZE);

        let root = Node {
            idx: 0,
            parent: None,
            children: Vec::new(),
            visits: 0,
            total_value: 0.0,
            position,
        };

        v.push(root);
        Tree { nodes: v }
    }

    fn select_expand_simulate(&mut self) {
        let root = self.nodes[0];
        let time = Instant::now();

        // Each move is given 5 seconds
        while time.elapsed().as_millis() > 5000 {
            let mut node = root;

            // Find best terminal node
            while node.children.len() > 0 {
                let child_idx = node.select_child(self);
                node = self.nodes[child_idx];
            }

            let mut value = node.rollout();

            // Backpropagation
            while node.parent.is_some() {
                node.visits += 1;
                node.total_value += value as f32;
                value = -value;
                node = self.nodes[node.parent.unwrap()];
            }
        }
    }
}

impl Node {
    fn ucb1(self, tree: Tree) -> f32 {
        if self.visits == 0 {
            return INFINITY;
        }

        let exploitation = self.total_value / self.visits as f32;
        let exploration = C
            * ((2.0 * (tree.nodes[self.parent.unwrap()].visits as f32).ln()) / self.visits as f32)
                .sqrt();
        let v = exploitation + exploration;

        v
    }

    fn select_child(&mut self, tree: &mut Tree) -> usize {
        if self.children.len() == 0 {
            self.expand(tree);
            return self.children[fastrand::usize(..self.children.len())];
        }

        let mut best_value = 0.0;
        let mut best_child = None;

        for child_idx in self.children {
            let child = tree.nodes[child_idx];
            let child_value = child.ucb1(*tree);

            if child_value > best_value {
                best_value = child_value;
                best_child = Some(child_idx);
            }
        }

        best_child.unwrap()
    }

    fn rollout(self) -> i32 {
        let mut position: Position = self.position.clone();
        let s2m = position.turn;

        while !position.game_over() {
            let moves = position.generate_moves();
            let random_move = moves.data[fastrand::usize(..moves.len())];
            position.make_move(random_move);
        }

        let outcome_score = match position.winner() {
            Some(Outcome::WhiteWin) => 1,
            Some(Outcome::BlackWin) => -1,
            Some(Outcome::Draw) => 0,
            None => panic!("Game not over"),
        };

        if s2m == Side::White {
            return outcome_score;
        } else {
            return -outcome_score;
        }
    }

    fn expand(&mut self, tree: &mut Tree) {
        let moves = self.position.generate_moves();

        for m in moves.as_slice() {
            let mut new_position = self.position.clone();
            new_position.make_move(*m);

            let new_node = Node {
                idx: tree.nodes.len(),
                parent: Some(self.idx),
                children: Vec::new(),
                visits: 0,
                total_value: 0.0,
                position: new_position,
            };

            tree.nodes.push(new_node);
            self.children.push(new_node.idx);
        }
    }
}
