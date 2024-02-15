use std::default;

use crate::piece::PieceType;

#[derive(Debug)]
pub struct Action {
    pub to: i32,
    pub from: i32,
    pub action_type: ActionType,
}

impl Action {
    pub fn new(from: i32, to: i32, action_type: Option<ActionType>) -> Action {
        Action {
            from,
            to,
            action_type: match action_type {
                Some(a_type) => a_type,
                None => ActionType::default(),
            },
        }
    }
}

#[derive(Debug, Default)]
pub enum ActionType {
    #[default]
    Normal,
    Castle,
    Promote(PieceType),
}
