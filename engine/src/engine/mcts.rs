use crate::ataxx::position::Position;

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
}

impl Node {
    fn ucb1(self, tree: Tree) -> f32 {
        if self.visits == 0 {
            return INFINITY;
        }

        let exploitation = self.total_value / self.visits as f32;
        let exploration = C * ((2.0 * self.total_visits(tree).ln()) / self.visits as f32).sqrt();
        let v = exploitation + exploration;

        v
    }

    fn total_visits(self, tree: Tree) -> f32 {
        // Recursively travel up to the root and return it's visits
        match self.parent {
            Some(parent_idx) => {
                let parent = tree.nodes[parent_idx];
                return parent.total_visits(tree);
            }
            None => return self.visits as f32,
        }
    }

    fn select_child(self, tree: Tree) -> Node {
        if self.children.len() == 0 {
            self.expand();
            return self.select_child(tree);
        }

        let mut best_value = 0.0;
        let mut best_child = None;

        for child_idx in self.children {
            let child = tree.nodes[child_idx];
            let child_value = child.ucb1(tree);

            if child_value > best_value {
                best_value = child_value;
                best_child = Some(child_idx);
            }
        }

        return tree.nodes[best_child.unwrap()];
    }

    fn rollout(self) -> i32 {
        let mut position: Position = self.position.clone();

        while !position.game_over() {
            let moves = position.generate_moves();
            let random_move = moves.data[fastrand::usize(..moves.len())];
            position.make_move(random_move);
        }

        // return winner
    }

    fn expand(self) {
        panic!("Not implemented")
    }
}
