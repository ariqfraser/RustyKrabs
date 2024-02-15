use crate::{
    action::Action,
    board::{Board, PieceIndices},
    directions::{allowed_directions, EAST, NORTH, SOUTH, WEST},
    piece::PieceType,
    precomputed_data::PrecomputedData,
};

#[derive(Default)]
pub struct ActionGenerator {
    precomputed: PrecomputedData,
    current_indices: PieceIndices,
    opposition_indicies: PieceIndices,
}

impl ActionGenerator {
    pub fn generate_valid_actions(&mut self, board: Board, for_white: bool) -> Vec<Action> {
        let (current_indices, opposition_indicies): (PieceIndices, PieceIndices) = match for_white {
            true => (board.white_pieces, board.black_pieces),
            _ => (board.black_pieces, board.white_pieces),
        };

        self.opposition_indicies = opposition_indicies;
        self.current_indices = current_indices;

        let mut actions = Vec::new();
        for pos in self.current_indices.iter() {
            let piece = board.array[pos.to_owned() as usize];

            let new_actions = match piece.piece_type {
                PieceType::Pawn => {
                    self.generate_pawn_moves(pos.to_owned(), piece.is_white, piece.has_moved)
                }
                PieceType::Knight => self.generate_knight_moves(pos.to_owned()),
                _ => self.generate_sliding_moves(pos.to_owned(), piece.piece_type, piece.is_white),
            };

            actions.extend(new_actions);
        }
        actions
    }

    fn in_check(&self, board: &Board, check_for_white: bool) -> bool {
        match check_for_white {
            true => board.white_in_check,
            false => board.black_in_check,
        }
    }

    fn generate_pawn_moves(&self, pos: i32, is_white: bool, has_moved: bool) -> Vec<Action> {
        let mut actions = Vec::new();

        let directions = allowed_directions(PieceType::Pawn, is_white);

        for direction in directions {
            let target = pos + direction.offset;
            if direction.limit == 2 {
                if !self.friendly_obstruction(&target) && !self.enemy_obstruction(&target) {
                    actions.push(Action::new(pos, target, None));

                    let double_push = target + direction.offset;
                    if !self.friendly_obstruction(&double_push)
                        && !self.enemy_obstruction(&double_push)
                        && !has_moved
                    {
                        actions.push(Action::new(pos, double_push, None));
                    }
                }
                continue;
            }

            if self.enemy_obstruction(&target) {
                actions.push(Action::new(pos, target, None));
            }
        }

        actions
    }

    fn generate_knight_moves(&self, pos: i32) -> Vec<Action> {
        allowed_directions(PieceType::Knight, true)
            .into_iter()
            .flat_map(|dir| {
                if self.precomputed.get_edge_distance(pos, dir.name) < 2 {
                    return Vec::new();
                }

                let p_pos = pos + dir.offset * 2;
                let dist = self.precomputed.moves_to_edge[p_pos as usize];

                match dir.name {
                    "NORTH" | "SOUTH" => {
                        self.get_knight_targets(p_pos, (WEST, EAST), (dist.west, dist.east))
                    }

                    "EAST" | "WEST" => {
                        self.get_knight_targets(p_pos, (NORTH, SOUTH), (dist.north, dist.south))
                    }
                    _ => vec![],
                }
                .iter()
                .filter(move |&t| !self.friendly_obstruction(t))
                .map(move |t| Action::new(pos, t.to_owned(), None))
                .collect()
            })
            .collect()
    }

    fn get_knight_targets(&self, pos: i32, offsets: (i32, i32), distances: (i32, i32)) -> Vec<i32> {
        let mut targets: Vec<i32> = Vec::new();
        targets.extend(distances.0.gt(&0).then_some(pos + offsets.0));
        targets.extend(distances.1.gt(&0).then_some(pos + offsets.1));
        targets.into_iter().collect()
    }

    fn generate_sliding_moves(
        &self,
        pos: i32,
        piece_type: PieceType,
        is_white: bool,
    ) -> Vec<Action> {
        let mut moves: Vec<Action> = Vec::new();

        for direction in allowed_directions(piece_type, is_white) {
            let dist_from_edge = self.precomputed.get_edge_distance(pos, direction.name);

            for n in 1..=dist_from_edge.to_owned() {
                if n > direction.limit {
                    break;
                }

                let target_pos: i32 = pos + direction.offset * n;

                if self.friendly_obstruction(&target_pos) {
                    break;
                }

                moves.push(Action::new(pos, target_pos, None));

                if self.enemy_obstruction(&target_pos) {
                    break;
                }
            }
        }

        moves
    }

    fn friendly_obstruction(&self, target: &i32) -> bool {
        self.current_indices.contains(target)
    }

    fn enemy_obstruction(&self, target: &i32) -> bool {
        self.opposition_indicies.contains(target)
    }
}

#[cfg(test)]
mod tests {
    use super::ActionGenerator;
    use crate::precomputed_data::PrecomputedData;

    // KNIGHT MOVES

    #[test]
    fn generate_knight_moves_at_center_board() {
        let gen = ActionGenerator::default();

        let moves = gen.generate_knight_moves(27);

        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn generate_knight_moves_at_corner() {
        let gen = ActionGenerator::default();

        let mut moves = gen.generate_knight_moves(0); // bottom left
        moves.extend(gen.generate_knight_moves(7)); // bottom right
        moves.extend(gen.generate_knight_moves(63)); // top right
        moves.extend(gen.generate_knight_moves(56)); // top left

        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn generate_knight_moves_at_center_edge() {
        let gen = ActionGenerator::default();

        let moves = gen.generate_knight_moves(24);

        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn generate_knight_moves_at_corner_edge() {
        let gen = ActionGenerator::default();

        let moves = gen.generate_knight_moves(15);

        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn generate_knight_moves_with_blocks() {
        let gen = ActionGenerator {
            current_indices: vec![10, 17],
            opposition_indicies: vec![],
            precomputed: PrecomputedData::default(),
        };
        let moves = gen.generate_knight_moves(0);

        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn generate_knight_moves_with_with_captures() {
        let gen = ActionGenerator {
            opposition_indicies: vec![10, 17],
            current_indices: vec![],
            precomputed: PrecomputedData::default(),
        };

        let moves = gen.generate_knight_moves(0);

        assert_eq!(moves.len(), 2);
    }

    // PAWN MOVES - WHITE

    #[test]
    fn generate_pawn_moves_white_when_not_moved_and_no_captures() {
        let gen = ActionGenerator::default();
        let moves = gen.generate_pawn_moves(8, true, false);

        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn generate_pawn_moves_white_when_has_moved_and_no_caps() {
        let gen = ActionGenerator::default();
        let moves = gen.generate_pawn_moves(8, true, true);

        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn generate_pawn_moves_white_when_not_moved_and_captures() {
        let gen = ActionGenerator {
            opposition_indicies: vec![10, 17],
            current_indices: vec![],
            precomputed: PrecomputedData::default(),
        };
        let moves = gen.generate_pawn_moves(8, true, false);

        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn generate_pawn_moves_white_when_not_moved_and_captures_and_blocks() {
        let gen = ActionGenerator {
            opposition_indicies: vec![10, 17],
            current_indices: vec![16],
            precomputed: PrecomputedData::default(),
        };
        let moves = gen.generate_pawn_moves(8, true, false);

        assert_eq!(moves.len(), 1);
    }

    // PAWN MOVES - BLACK

    #[test]
    fn generate_pawn_moves_black_when_not_moved_and_no_captures() {
        let gen = ActionGenerator::default();
        let moves = gen.generate_pawn_moves(43, false, false);

        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn generate_pawn_moves_black_when_has_moved_and_no_caps() {
        let gen = ActionGenerator::default();
        let moves = gen.generate_pawn_moves(43, false, true);

        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn generate_pawn_moves_black_when_not_moved_and_captures() {
        let gen = ActionGenerator {
            opposition_indicies: vec![8],
            current_indices: vec![],
            precomputed: PrecomputedData::default(),
        };

        let moves = gen.generate_pawn_moves(17, false, false);
        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn generate_pawn_moves_black_when_not_moved_and_captures_and_blocks() {
        let gen = ActionGenerator {
            opposition_indicies: vec![10, 8],
            current_indices: vec![9],
            precomputed: PrecomputedData::default(),
        };
        let moves = gen.generate_pawn_moves(17, false, false);

        assert_eq!(moves.len(), 2);
    }
}
