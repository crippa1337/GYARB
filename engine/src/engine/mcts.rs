use super::moves::Move;
use crate::ataxx::position::{Outcome, Position, Side};
use std::{cell::RefCell, f32::consts::SQRT_2, rc::Rc, time::Instant};

const INFINITY: f32 = 1_000_000.0;
const C: f32 = SQRT_2; // sqrt(2)
const NODEPOOL_MAX_MEM: usize = 2 * 1024 * 1024 * 1024; // 2GB

#[derive(Clone, Debug, PartialEq)]
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

        let mut root = Node {
            idx: 0,
            parent: None,
            children: Rc::new(RefCell::new(Vec::new())),
            visits: 0,
            total_value: 0.0,
            position,
            from_action: Move::null(),
        };

        v.push(root.clone());
        let mut t = Tree { nodes: v };
        root.expand(&mut t);
        t
    }

    pub fn uct(&mut self) -> Move {
        let time = Instant::now();

        // Each move is given 5 seconds
        while time.elapsed().as_millis() < 5000 {
            let mut node_idx = 0;

            node_idx = self.tree_policy(node_idx);
            let node = &mut self.nodes[node_idx];
            let value = node.rollout();

            self.backup(node_idx, value);
        }

        let best_move = self.best_move();
        debug_assert_ne!(best_move, Move::null(), "No best move found");
        self.confirm_logic();

        best_move
    }

    fn tree_policy(&mut self, mut node_idx: usize) -> usize {
        let mut node = self.nodes[node_idx].clone();

        while !node.is_terminal() {
            if node.is_expandable() {
                node.expand(self);
                break;
            } else {
                node_idx = node.select_child(self);
                node = self.nodes[node_idx].clone();
            }
        }

        node_idx
    }

    fn backup(&mut self, mut node_idx: usize, mut value: i32) {
        let mut node = &mut self.nodes[node_idx];

        loop {
            node.visits += 1;
            node.total_value += value as f32;
            value = -value;
            node_idx = node.parent.unwrap();
            node = &mut self.nodes[node_idx];

            if node_idx == 0 {
                node.visits += 1;
                node.total_value += value as f32;
                break;
            }
        }
    }

    pub fn best_move(&self) -> Move {
        debug_assert!(!self.nodes.is_empty());
        let root = &self.nodes[0];
        let mut best_value = -INFINITY;
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

        best_move
    }

    pub fn confirm_logic(&self) {
        for node in self.nodes.iter() {
            let mut child_visits = 0;
            for child_idx in (*node.children).borrow().iter() {
                let child = &self.nodes[*child_idx];
                child_visits += child.visits;
            }
            assert_eq!(node.visits, child_visits + 1)
        }
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

    fn select_child(&self, tree: &Tree) -> usize {
        let mut best_value = -INFINITY;
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

        best_child.copied().unwrap()
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
        debug_assert!(self.is_expandable());
        debug_assert!(!self.is_terminal());

        let idx = self.children.borrow().len();
        let mut new_pos = self.position;
        let mv = new_pos.generate_moves().data[idx];
        new_pos.make_move(mv);

        let new_node = Node {
            idx: tree.nodes.len(),
            parent: Some(self.idx),
            children: Rc::new(RefCell::new(Vec::new())),
            visits: 0,
            total_value: 0.0,
            position: new_pos,
            from_action: mv,
        };

        debug_assert_eq!(tree.nodes[new_node.parent.unwrap()], tree.nodes[self.idx]);
        debug_assert_eq!(
            new_node.from_action,
            self.position.generate_moves().data[idx]
        );
        debug_assert_eq!(new_node.visits, 0);
        debug_assert_eq!(new_node.total_value, 0.0);
        debug_assert_eq!(new_node.children.borrow().len(), 0);
        debug_assert!(new_node.is_expanded());

        (*self.children).borrow_mut().push(new_node.idx);
        tree.nodes.push(new_node);
    }

    fn is_expandable(&self) -> bool {
        self.children.borrow().len() < self.position.generate_moves().len()
    }

    fn is_terminal(&self) -> bool {
        self.position.game_over()
    }

    fn is_expanded(&self) -> bool {
        !self.children.borrow().is_empty()
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
