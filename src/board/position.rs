use crate::board::{board::BOARD_LENGTH, digit::Digit};


/// Unique ID of a position on the board
type PositionId = u8;

/// Bitmask of the peers of a position
/// (PeersMask >> PositionId) & 0b1 == 0b1 means the position is a peer of the position with the given ID
type PeersMask = u128;

/// A lookup table that can be directly indexed by a PositionId
type LookupTableByPositionId<T> = [T; TOTAL_POSITIONS as usize];


pub const TOTAL_POSITIONS: u8 = BOARD_LENGTH * BOARD_LENGTH;
pub const MAX_POSITION_ID: PositionId = TOTAL_POSITIONS - 1;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub id: PositionId
}

impl Position {
    const BOX_ID_BY_POSITION_ID_LUT: LookupTableByPositionId<Digit> = build_box_id_by_position_id_lut();
    const ROW_AND_COL_PEERS_MASK_BY_POSITION_ID_LUT: (LookupTableByPositionId<PeersMask>, LookupTableByPositionId<PeersMask>) = build_row_and_col_peers_mask_by_id_lut();

    pub const fn new(row: Digit, col: Digit) -> Self {
        let row_offset = (row.as_u8() - 1) * BOARD_LENGTH;
        let col_offset = col.as_u8() - 1;

        return Self { id: row_offset + col_offset };
    }

    pub const fn from_id(id: PositionId) -> Self {
        return Self { id };
    }

    pub const fn row(self) -> Digit {
        return match self.id / BOARD_LENGTH {
            0 => Digit::ONE,
            1 => Digit::TWO,
            2 => Digit::THREE,
            3 => Digit::FOUR,
            4 => Digit::FIVE,
            5 => Digit::SIX,
            6 => Digit::SEVEN,
            7 => Digit::EIGHT,
            8 => Digit::NINE,
            _ => unreachable!(),
        };
    }

    pub const fn col(self) -> Digit {
        return match self.id % BOARD_LENGTH {
            0 => Digit::ONE,
            1 => Digit::TWO,
            2 => Digit::THREE,
            3 => Digit::FOUR,
            4 => Digit::FIVE,
            5 => Digit::SIX,
            6 => Digit::SEVEN,
            7 => Digit::EIGHT,
            8 => Digit::NINE,
            _ => unreachable!(),
        };
    }

    pub const fn box_id(self) -> Digit {
        return Self::BOX_ID_BY_POSITION_ID_LUT[self.id as usize];
    }

    pub const fn row_peers(self) -> PeersMask {
        return Self::ROW_AND_COL_PEERS_MASK_BY_POSITION_ID_LUT.0[self.id as usize];
    }

    pub const fn col_peers(self) -> PeersMask {
        return Self::ROW_AND_COL_PEERS_MASK_BY_POSITION_ID_LUT.1[self.id as usize];
    }

    pub const fn id(self) -> PositionId {
        return self.id;
    }
}

const fn build_box_id_by_position_id_lut() -> LookupTableByPositionId<Digit> {
    let mut table: LookupTableByPositionId<Digit> = [Digit::ONE; TOTAL_POSITIONS as usize];

    let mut position_id: PositionId = 0;
    while position_id <= MAX_POSITION_ID {
        let position = Position::from_id(position_id);

        let box_id = match (position.row(), position.col()) {
            (
                Digit::ONE | Digit::TWO | Digit::THREE,
                Digit::ONE | Digit::TWO | Digit::THREE
            ) => Digit::ONE,
            (
                Digit::ONE | Digit::TWO | Digit::THREE,
                Digit::FOUR | Digit::FIVE | Digit::SIX
            ) => Digit::TWO,
            (
                Digit::ONE | Digit::TWO | Digit::THREE,
                Digit::SEVEN | Digit::EIGHT | Digit::NINE
            ) => Digit::THREE,
            (
                Digit::FOUR | Digit::FIVE | Digit::SIX,
                Digit::ONE | Digit::TWO | Digit::THREE
            ) => Digit::FOUR,
            (
                Digit::FOUR | Digit::FIVE | Digit::SIX,
                Digit::FOUR | Digit::FIVE | Digit::SIX
            ) => Digit::FIVE,
            (
                Digit::FOUR | Digit::FIVE | Digit::SIX,
                Digit::SEVEN | Digit::EIGHT | Digit::NINE
            ) => Digit::SIX,
            (
                Digit::SEVEN | Digit::EIGHT | Digit::NINE,
                Digit::ONE | Digit::TWO | Digit::THREE
            ) => Digit::SEVEN,
            (
                Digit::SEVEN | Digit::EIGHT | Digit::NINE,
                Digit::FOUR | Digit::FIVE | Digit::SIX
            ) => Digit::EIGHT,
            (
                Digit::SEVEN | Digit::EIGHT | Digit::NINE,
                Digit::SEVEN | Digit::EIGHT | Digit::NINE
            ) => Digit::NINE,
            _ => unreachable!(),
        };

        table[position_id as usize] = box_id;
        position_id += 1;
    }

    return table;
}

const fn build_row_and_col_peers_mask_by_id_lut() -> (LookupTableByPositionId<PeersMask>, LookupTableByPositionId<PeersMask>) {
    let mut row_peers_mask_table: LookupTableByPositionId<PeersMask> = [0; TOTAL_POSITIONS as usize];
    let mut col_peers_mask_table: LookupTableByPositionId<PeersMask> = [0; TOTAL_POSITIONS as usize];

    let mut position_id: PositionId = 0;
    while position_id <= MAX_POSITION_ID {
        let position = Position::from_id(position_id);

        let mut row_peers_mask: PeersMask = 0;
        let mut col_peers_mask: PeersMask = 0;

        let mut peer_id = 0;
        while peer_id < TOTAL_POSITIONS {
            let peer_position = Position::from_id(peer_id);

            if peer_id != position_id {
                if peer_position.row().as_u8() == position.row().as_u8() {
                    row_peers_mask |= 1 << peer_id;
                }

                if peer_position.col().as_u8() == position.col().as_u8() {
                    col_peers_mask |= 1 << peer_id;
                }
            }

            peer_id += 1;
        }


        assert!(row_peers_mask.count_ones() == BOARD_LENGTH as u32 - 1);

        row_peers_mask_table[position_id as usize] = row_peers_mask;
        col_peers_mask_table[position_id as usize] = col_peers_mask;
        position_id += 1;
    }

    return (row_peers_mask_table, col_peers_mask_table);
}
