pub trait Game {
    type GameState: GameState<Action = Self::GameAction>;
    type GameAction: GameAction;
    type GameOutcome: GameOutcome;
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
enum PlayerColor {
    Black,
    White,
}

/// A trait describing a game's state.
/// A GameState can be updated to its next state
/// by applying a GameAction.
trait GameState: Clone + Sized {
    type Action: GameAction;
    type Outcome: GameOutcome;

    fn next(&self, action: Self::Action) -> Self {
        let mut next = self.clone();
        next.make_next(action);
        next
    }

    fn make_next(&mut self, action: Self::Action);

    fn legal_actions(&self) -> Vec<Self::Action>;

    fn current_player_turn(&self) -> PlayerColor;

    fn outcome(&self) -> Option<Self::Outcome>;
}

/// A trait describing a game's action,
/// which is the input from a Player that updates
/// a GameState from one state to the next.
trait GameAction: Copy {}

/// A trait describing the final outcome of a Game, after it is played to completion.
trait GameOutcome: Copy {
    fn is_final(&self) -> bool;
}

/// A trait describing an agent.
/// A GameAgent is shown a GameState,
/// and from that GameState it picks the
/// GameAction it wants to take in that GameState.
trait GameAgent<G: Game> {
    fn pick_action(&self, state: &G::GameState, actions: &[G::GameAction]) -> G::GameAction;
}

struct GameRunner<G: Game> {
    black_agent: Box<dyn GameAgent<G>>,
    white_agent: Box<dyn GameAgent<G>>,
    game_state: G::GameState,
}

impl<G: Game> GameRunner<G> {
    pub fn new(
        black_agent: Box<dyn GameAgent<G>>,
        white_agent: Box<dyn GameAgent<G>>,
        start_state: G::GameState,
    ) -> Self {
        Self {
            black_agent,
            white_agent,
            game_state: start_state,
        }
    }

    pub fn play(mut self) {
        while self.game_state.outcome().is_none() {
            let active_player = match self.game_state.current_player_turn() {
                PlayerColor::Black => &self.black_agent,
                PlayerColor::White => &self.white_agent,
            };

            let legal_actions = self.game_state.legal_actions();
            let selected_action = active_player.pick_action(&self.game_state, &legal_actions);
            self.game_state.make_next(selected_action);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;
    use std::marker::PhantomData;

    #[derive(Default, Debug)]
    struct SimpleGame;

    #[derive(Debug, Clone)]
    struct SimpleGameState {
        num: usize,
        cur_player: PlayerColor,
    }

    impl SimpleGameState {
        fn new() -> Self {
            Self {
                num: 0,
                cur_player: PlayerColor::Black,
            }
        }
    }

    #[derive(Copy, Clone)]
    struct SimpleGameAction {
        bump: usize,
    }

    impl SimpleGameAction {
        fn new(bump: usize) -> Self {
            Self { bump }
        }
    }

    #[derive(Copy, Clone)]
    enum SimpleGameOutcome {
        BlackWins,
        WhiteWins,
        BothLose,
    }

    #[derive(Default)]
    struct SimpleAgent<G: Game> {
        g: PhantomData<G>,
    }

    impl GameAction for SimpleGameAction {}

    impl GameOutcome for SimpleGameOutcome {
        fn is_final(&self) -> bool {
            todo!()
        }
    }

    impl GameState for SimpleGameState {
        type Action = SimpleGameAction;
        type Outcome = SimpleGameOutcome;

        fn make_next(&mut self, action: Self::Action) {
            self.num += action.bump;
        }

        fn legal_actions(&self) -> Vec<Self::Action> {
            vec![
                SimpleGameAction::new(2),
                SimpleGameAction::new(3),
                SimpleGameAction::new(4),
            ]
        }

        fn current_player_turn(&self) -> PlayerColor {
            self.cur_player
        }

        fn outcome(&self) -> Option<Self::Outcome> {
            if self.num < 42 {
                None
            } else if self.num > 42 {
                Some(SimpleGameOutcome::BothLose)
            } else {
                let outcome = match self.cur_player {
                    PlayerColor::Black => SimpleGameOutcome::BlackWins,
                    PlayerColor::White => SimpleGameOutcome::WhiteWins,
                };

                Some(outcome)
            }
        }
    }

    impl<G: Game> GameAgent<G> for SimpleAgent<G> {
        fn pick_action(&self, _: &G::GameState, actions: &[G::GameAction]) -> G::GameAction {
            actions[0]
        }
    }

    impl Game for SimpleGame {
        type GameState = SimpleGameState;
        type GameAction = SimpleGameAction;
        type GameOutcome = SimpleGameOutcome;
    }

    #[test]
    fn it_works() {
        let black_agent = Box::new(SimpleAgent::<SimpleGame>::default());
        let white_agent = Box::new(SimpleAgent::<SimpleGame>::default());

        let start_state = SimpleGameState::new();

        let runner = GameRunner::new(black_agent, white_agent, start_state);

        runner.play();
    }
}
