use crate::moving::Move;

pub trait ChebyshevDistance<O> {
    fn chebyshev_distance(&self, other: O) -> usize;
}

pub trait ChebyshevDistanceSelf {
    fn chebyshev_distance(&self) -> usize;
}

impl<S, O> ChebyshevDistance<O> for S
where
    S: Into<(u8, u8)> + Copy,
    O: Into<(u8, u8)> + Copy,
{
    fn chebyshev_distance(&self, other: O) -> usize {
        let self_pos: (u8, u8) = (*self).into();
        let other_pos = other.into();
        let x = (self_pos.0 as i32 - other_pos.0 as i32).abs();
        let y = (self_pos.1 as i32 - other_pos.1 as i32).abs();
        x.max(y) as usize
    }
}

impl ChebyshevDistanceSelf for Move {
    fn chebyshev_distance(&self) -> usize {
        self.from.chebyshev_distance(self.to)
    }
}
