#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        let valid_range = 0..8;
        let valid = valid_range.contains(&rank) && valid_range.contains(&file);

        valid.then_some(Self {
            rank,
            file
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let p1 = &self.0;
        let p2 = &other.0;

        p1.file == p2.file || p1.rank == p2.rank || (p1.rank - p2.rank).abs() == (p1.file - p2.file).abs()
    }
}
