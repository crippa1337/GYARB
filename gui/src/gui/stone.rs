use tetra::graphics::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Stone {
    Dark,
    Light,
}

impl std::ops::Not for Stone {
    type Output = Stone;

    fn not(self) -> Self::Output {
        match self {
            Stone::Dark => Stone::Light,
            Stone::Light => Stone::Dark,
        }
    }
}

impl From<Stone> for usize {
    fn from(stone: Stone) -> Self {
        match stone {
            Stone::Dark => 0,
            Stone::Light => 1,
        }
    }
}

impl From<usize> for Stone {
    fn from(n: usize) -> Self {
        match n % 2 {
            0 => Stone::Dark,
            1 => Stone::Light,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Stone {
    fn from(c: char) -> Self {
        match c {
            'x' => Stone::Dark,
            'o' => Stone::Light,
            _ => panic!("Invalid character: {}", c),
        }
    }
}

impl Stone {
    pub fn color(&self) -> Color {
        match self {
            Stone::Dark => Color::rgb(1., 0., 0.),
            Stone::Light => Color::rgb(0., 0., 1.),
        }
    }
}
