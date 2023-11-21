use super::moves::Move;
use crate::ataxx::position::{Outcome, Position, Side};
use std::{cell::RefCell, f32::consts::SQRT_2, rc::Rc, time::Instant};

const INFINITY: f32 = 1_000_000.0;
const C: f32 = SQRT_2; // sqrt(2)
const NODEPOOL_MAX_MEM: usize = 2 * 1024 * 1024 * 1024; // 2GB

#[derive(Clone, Debug)]
struct Node {
    idx: usize,
    parent: Option<usize>,
    children: Rc<RefCell<Vec<usize>>>,
    visits: usize,
    total_value: f32,
    position: Position,
    from_action: Move,
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn new(position: Position) -> Self {
        const NODEPOOL_SIZE: usize = NODEPOOL_MAX_MEM / std::mem::size_of::<Node>();
        let mut v = Vec::with_capacity(NODEPOOL_SIZE);

        let root = Node {
            idx: 0,
            parent: None,
            children: Rc::new(RefCell::new(Vec::new())),
            visits: 0,
            total_value: 0.0,
            position,
            from_action: Move::null(),
        };

        v.push(root);
        Tree { nodes: v }
    }

    pub fn select_expand_simulate(&mut self) {
        let root_idx = 0;
        let time = Instant::now();

        // Each move is given 5 seconds
        while time.elapsed().as_millis() < 5000 {
            let mut node_idx = root_idx;

            // Find best terminal node
            loop {
                let mut node = self.nodes[node_idx].clone();
                let len = (*node.children).borrow().len();

                if node.position.game_over() {
                    break;
                }

                node_idx = if let Some(idx) = node.select_child(self) {
                    idx
                } else {
                    node.expand(self);
                    let len = (*node.children).borrow().len();
                    let children = (*node.children).borrow_mut();
                    children[fastrand::usize(..len)]
                };

                // Found a rollout node
                if len == 0 && node.visits == 0 {
                    break;
                }
            }

            let mut node = &mut self.nodes[node_idx];
            let mut value = node.rollout();

            while node.parent.is_some() {
                node.visits += 1;
                node.total_value += value as f32;
                value = -value;
                let idx = node.parent.unwrap();
                node = &mut self.nodes[idx];
            }
        }
    }

    pub fn best_move(&self) -> Move {
        assert!(!self.nodes.is_empty());
        let root = &self.nodes[0];
        let mut best_value = 0.0;
        let mut best_move = Move::null();

        let children = (*root.children).borrow();

        for child_idx in children.iter() {
            let child = &self.nodes[*child_idx];
            let avg_val = child.total_value / child.visits as f32;

            if avg_val > best_value {
                best_value = avg_val;
                best_move = child.from_action;
            }
        }

        assert_ne!(best_move, Move::null());
        best_move
    }
}

impl Node {
    fn ucb1(&self, tree: &Tree) -> f32 {
        if self.visits == 0 {
            return INFINITY;
        }

        let exploitation = self.total_value / self.visits as f32;
        let exploration = C
            * ((2.0 * (tree.nodes[self.parent.unwrap()].visits as f32).ln()) / self.visits as f32)
                .sqrt();

        exploitation + exploration
    }

    fn select_child(&self, tree: &Tree) -> Option<usize> {
        if self.children.borrow().len() == 0 && self.visits > 0 {
            return None;
        }

        let mut best_value = 0.0;
        let mut best_child = None;

        let children = (*self.children).borrow();

        for child_idx in children.iter() {
            let child = &tree.nodes[*child_idx];
            let child_value = child.ucb1(tree);

            if child_value > best_value {
                best_value = child_value;
                best_child = Some(child_idx);
            }
        }

        best_child.copied()
    }

    fn rollout(&self) -> i32 {
        let mut position: Position = self.position;
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
            outcome_score
        } else {
            -outcome_score
        }
    }

    fn expand(&mut self, tree: &mut Tree) {
        let moves = self.position.generate_moves();

        for m in moves.as_slice() {
            let mut new_position = self.position;
            new_position.make_move(*m);

            let new_node = Node {
                idx: tree.nodes.len(),
                parent: Some(self.idx),
                children: Rc::new(RefCell::new(Vec::new())),
                visits: 0,
                total_value: 0.0,
                position: new_position,
                from_action: *m,
            };

            (*self.children).borrow_mut().push(new_node.idx);
            tree.nodes.push(new_node);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {
        use super::*;
        let mut tree = Tree::new(Position::default());
        tree.select_expand_simulate();
        assert!(!tree.nodes.is_empty());
    }
}
