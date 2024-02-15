use crate::piece::PieceType;

pub const EAST: i32 = 1;
pub const WEST: i32 = -1;

pub const NORTH: i32 = -8;
pub const NORTH_WEST: i32 = NORTH + WEST;
pub const NORTH_EAST: i32 = NORTH + EAST;

pub const SOUTH: i32 = 8;
pub const SOUTH_WEST: i32 = SOUTH + WEST;
pub const SOUTH_EAST: i32 = SOUTH + EAST;

pub const DIRECTIONS: [&str; 8] = [
    "NORTH",
    "NORTH_EAST",
    "EAST",
    "SOUTH_EAST",
    "SOUTH",
    "SOUTH_WEST",
    "WEST",
    "NORTH_WEST",
];

#[derive(Debug, Clone, Copy)]
pub struct Direction<'a> {
    pub name: &'a str,
    pub offset: i32,
    pub limit: i32,
}

impl<'a> Direction<'a> {
    fn new(name: &'a str, offset: i32, limit: i32) -> Self {
        Direction {
            name,
            offset,
            limit,
        }
    }
}

pub fn allowed_directions(piece_type: PieceType, is_white: bool) -> Vec<Direction<'static>> {
    let rook: Vec<Direction> = vec![
        Direction::new("NORTH", NORTH, 8),
        Direction::new("EAST", EAST, 8),
        Direction::new("SOUTH", SOUTH, 8),
        Direction::new("WEST", WEST, 8),
    ];

    let bishop = vec![
        Direction::new("NORTH_EAST", NORTH_EAST, 8),
        Direction::new("NORTH_WEST", NORTH_WEST, 8),
        Direction::new("SOUTH_EAST", SOUTH_EAST, 8),
        Direction::new("SOUTH_WEST", SOUTH_WEST, 8),
    ];

    match piece_type {
        PieceType::Pawn => match is_white {
            false => vec![
                Direction::new("NORTH", NORTH, 2),
                Direction::new("NORTH_EAST", NORTH_EAST, 1),
                Direction::new("NORTH_WEST", NORTH_WEST, 1),
            ],
            true => vec![
                Direction::new("SOUTH", SOUTH, 2),
                Direction::new("SOUTH_EAST", SOUTH_EAST, 1),
                Direction::new("SOUTH_WEST", SOUTH_WEST, 1),
            ],
        },
        // using rook here then just manually offsetting and limiting
        PieceType::Knight => rook,
        PieceType::Bishop => bishop,
        PieceType::Rook => rook,
        PieceType::Queen => [&bishop[..], &rook[..]].concat(),
        PieceType::King => vec![
            Direction::new("NORTH", NORTH, 1),
            Direction::new("EAST", EAST, 1),
            Direction::new("SOUTH", SOUTH, 1),
            Direction::new("WEST", WEST, 1),
            Direction::new("NORTH_EAST", NORTH_EAST, 1),
            Direction::new("NORTH_WEST", NORTH_WEST, 1),
            Direction::new("SOUTH_EAST", SOUTH_EAST, 1),
            Direction::new("SOUTH_WEST", SOUTH_WEST, 1),
        ],
        PieceType::Empty => vec![],
    }
}
