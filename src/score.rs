pub struct Score {
    pub current_score: i32,
    pub blocks_destroyed: i32,
    pub gold: i32,
}

impl Score {
    pub fn init() -> Self {
        Score {
            current_score: 0,
            blocks_destroyed: 0,
            gold: 0,
        }
    }
}
