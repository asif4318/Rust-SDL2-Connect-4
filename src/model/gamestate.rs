use crate::model::tilestate::TileState;

pub enum GameCondition {
    PLAYING,
    ENDED
}

pub struct GameState {
    pub turn: TileState,
    pub game_condition: GameCondition
}

impl GameState {
    pub fn new() -> Self {
        let turn = TileState::RED;
        let game_condition = GameCondition::PLAYING;
        GameState{turn, game_condition}
    }

    pub fn next_turn(&mut self) {
        match self.turn {
            TileState::EMPTY => {}
            TileState::YELLOW => {self.turn = TileState::RED}
            TileState::RED => {self.turn = TileState::YELLOW}
        }
    }

    pub fn game_won(&mut self) {
        self.game_condition = GameCondition::ENDED;
    }
}