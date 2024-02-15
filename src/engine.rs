use crate::{action::Action, action_generator::ActionGenerator, board::Board};

pub struct Engine {
    board: Board,
    generator: ActionGenerator,
    max_depth: i32,
    nodes: i64,
}

impl Engine {
    pub fn new(fen: &str) -> Self {
        let board = Board::new(fen);
        let generator = ActionGenerator::default();
        board.print();
        Engine {
            board,
            generator,
            max_depth: 4,
            nodes: 0,
        }
    }

    pub fn move_piece(&self) {
        let mut new_board = self.board.clone();
        new_board.perform_action(Action::new(0, 32, None));
        new_board.print();
    }

    pub fn evaluate(&mut self) {
        Self::recurse_moves(self, &mut self.board.to_owned(), true, 0);
    }

    fn recurse_moves(&mut self, board: &mut Board, for_white: bool, depth: i32) {
        if depth == self.max_depth {
            // println!("Max Depth Reached {}", depth);
            return;
        }

        let valid_actions = self
            .generator
            .generate_valid_actions(board.to_owned(), for_white);

        println!(
            "Depth: {}  |  Valid actions {:?}  |  Nodes {}",
            depth + 1,
            valid_actions.len(),
            self.nodes
        );
        self.nodes += valid_actions.len() as i64;
        for action in valid_actions {
            let mut board_after_move = board.clone();
            board_after_move.perform_action(action);
            // println!("Moving for white? {}", for_white);
            // board_after_move.print();
            // println!(" ");

            self.recurse_moves(&mut board_after_move, !for_white, depth + 1);
        }
    }
}
