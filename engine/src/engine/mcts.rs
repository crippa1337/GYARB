use super::moves::Move;
use crate::ataxx::position::{Outcome, Position, Side};
use std::{cell::RefCell, f32::consts::SQRT_2, rc::Rc, time::Instant};

const INFINITY: f32 = 10_000_000.0;
const C: f32 = SQRT_2;
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
    pub fn new() -> Self {
        const NODEPOOL_SIZE: usize = NODEPOOL_MAX_MEM / std::mem::size_of::<Node>();
        let v = Vec::with_capacity(NODEPOOL_SIZE);
        Tree { nodes: v }
    }

    pub fn uct(&mut self, pos: Position, move_time: u128) -> Move {
        let time = Instant::now();
        let moves = pos.generate_moves();
        if moves.len() == 1 {
            return moves.data[0];
        }

        let mut root = Node::new();
        root.position = pos;
        self.nodes.push(root.clone());

        while time.elapsed().as_millis() < move_time {
            let selection = self.tree_policy(root.clone());
            let value = selection.default_policy();
            self.backup_negamax(selection.idx, value);
        }

        let best_move = self.best_move();
        debug_assert_ne!(best_move, Move::null(), "No best move found");
        self.confirm_logic();

        best_move
    }

    fn tree_policy(&mut self, mut node: Node) -> Node {
        while !node.is_terminal() {
            if node.is_expandable() {
                node = node.expand(self);
                break;
            } else {
                let node_idx = node.best_child(self);
                node = self.nodes[node_idx].clone();
            }
        }

        node
    }

    fn backup_negamax(&mut self, mut node_idx: usize, mut delta: f32) {
        let mut node = &mut self.nodes[node_idx];
        debug_assert!(!node.is_expanded());

        loop {
            node.visits += 1;
            node.total_value += delta;
            debug_assert!(0.0 <= node.total_value && node.total_value as usize <= node.visits);

            delta = 1.0 - delta;
            node_idx = node.parent.unwrap();
            node = &mut self.nodes[node_idx];

            // Root
            if node_idx == 0 {
                node.visits += 1;
                node.total_value += delta;
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
            debug_assert_eq!(node.visits, child_visits + 1)
        }
    }
}

impl Node {
    fn new() -> Self {
        Node {
            idx: 0,
            parent: None,
            children: Rc::new(RefCell::new(Vec::new())),
            visits: 0,
            total_value: 0.0,
            position: Position::default(),
            from_action: Move::null(),
        }
    }

    fn ucb1(&self, tree: &Tree) -> f32 {
        if self.visits == 0 {
            return INFINITY;
        }

        let exploitation = self.total_value / self.visits as f32;
        let exploration = C
            * ((2.0 * (tree.nodes[self.parent.unwrap()].visits as f32).ln()) / self.visits as f32)
                .sqrt();
        let reward = exploitation + exploration;

        debug_assert!(!reward.is_nan());

        reward
    }

    fn best_child(&self, tree: &Tree) -> usize {
        debug_assert!(self.is_expanded());
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

        if best_child.is_none() {
            println!("No best child found");
            for child_idx in children.iter() {
                let child = &tree.nodes[*child_idx];
                println!(
                    "Child: {} | Visits: {} | Value: {}",
                    child.from_action,
                    child.visits,
                    child.ucb1(tree)
                );
            }
        }
        best_child.copied().unwrap()
    }

    fn rollout(&self) -> f32 {
        // Random mean rollouts
        let mut position: Position = self.position;
        let s2m = position.turn;

        while !position.game_over() {
            let moves = position.generate_moves();
            let random_move = moves.data[fastrand::usize(..moves.len())];
            position.make_move(random_move);
        }

        let value = match position.winner().unwrap() {
            Outcome::WhiteWin => 1.0,
            Outcome::BlackWin => 0.0,
            Outcome::Draw => 0.5,
        };

        if s2m == Side::White {
            value
        } else {
            1.0 - value
        }
    }

    fn default_policy(&self) -> f32 {
        self.rollout()
    }

    fn expand(&self, tree: &mut Tree) -> Node {
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

        (*self.children).borrow_mut().push(new_node.idx);
        tree.nodes.push(new_node.clone());
        debug_assert!(self.is_expanded());

        new_node
    }

    fn is_expandable(&self) -> bool {
        self.children.borrow().len() < self.position.generate_moves().len()
    }

    fn is_terminal(&self) -> bool {
        !self.is_expanded() && !self.is_expandable()
    }

    fn is_expanded(&self) -> bool {
        !self.children.borrow().is_empty()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn sanity() {
        use super::*;
        let mut tree = Tree::new();
        tree.uct(Position::default(), 5000);
        assert!(!tree.nodes.is_empty());
    }
}
