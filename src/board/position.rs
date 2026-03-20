use crate::board::{board::BOARD_LENGTH, digit::Digit};


/// Unique ID of a position on the board
pub type PositionId = u8;

/// Bitmask of the peers of a position
/// (PeersMask >> PositionId) & 0b1 == 0b1 means the position is a peer of the position with the given ID
type PeersMaskBits = u128;

/// A lookup table that can be directly indexed by a PositionId
type LookupTableByPositionId<T> = [T; TOTAL_POSITIONS as usize];


pub const TOTAL_PEERS_PER_CELL: u8 = 0
    + (BOARD_LENGTH as u8 - 1) // row peers
    + (BOARD_LENGTH as u8 - 1) // col peers
    + (BOARD_LENGTH as u8 - 1) // box peers
    - 4; // overcounting the two row peers and col peers that are also box peers

pub const TOTAL_POSITIONS: u8 = BOARD_LENGTH * BOARD_LENGTH;
pub const MAX_POSITION_ID: PositionId = TOTAL_POSITIONS - 1;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeerIds {
    mask_bits: PeersMaskBits,
}

impl PeerIds {
    pub const fn new(mask_bits: PeersMaskBits) -> Self {
        return Self { mask_bits };
    }
}

impl Iterator for PeerIds {
    type Item = PositionId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mask_bits == 0 {
            return None;
        }

        let position_id = self.mask_bits.trailing_zeros() as PositionId;
        self.mask_bits &= self.mask_bits - 1;

        return Some(position_id);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub id: PositionId,
}

impl Position {
    const BOX_ID_BY_POSITION_ID_LUT: LookupTableByPositionId<Digit> = build_box_id_by_position_id_lut();
    const ALL_PEERS_MASK_BY_POSITION_ID_LUT: LookupTableByPositionId<PeersMaskBits> = build_all_peers_mask_by_position_id_lut();

    pub const fn new(row: Digit, col: Digit) -> Self {
        let row_offset = (row.as_u8() - 1) * BOARD_LENGTH;
        let col_offset = col.as_u8() - 1;

        return Self { id: row_offset + col_offset };
    }

    pub const fn from_id(id: PositionId) -> Self {
        return Self { id };
    }

    pub const fn id(self) -> PositionId {
        return self.id;
    }

    pub fn peer_ids(self) -> PeerIds {
        return PeerIds::new(Self::ALL_PEERS_MASK_BY_POSITION_ID_LUT[self.id as usize]);
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

const fn build_all_peers_mask_by_position_id_lut() -> LookupTableByPositionId<PeersMaskBits> {
    let mut all_peers_mask_lookup_table: LookupTableByPositionId<PeersMaskBits> = [0; TOTAL_POSITIONS as usize];

    let mut position_id: PositionId = 0;
    while position_id <= MAX_POSITION_ID {
        let position = Position::from_id(position_id);

        let mut row_peers_mask: PeersMaskBits = 0;
        let mut col_peers_mask: PeersMaskBits = 0;
        let mut box_peers_mask: PeersMaskBits = 0;

        let mut peer_id = 0;
        while peer_id < TOTAL_POSITIONS {
            if peer_id != position_id {
                let peer_position = Position::from_id(peer_id);

                if peer_position.row().as_u8() == position.row().as_u8() {
                    row_peers_mask |= 1 << peer_id;
                }

                if peer_position.col().as_u8() == position.col().as_u8() {
                    col_peers_mask |= 1 << peer_id;
                }

                if peer_position.box_id().as_u8() == position.box_id().as_u8() {
                    box_peers_mask |= 1 << peer_id;
                }
            }

            peer_id += 1;
        }

        assert!(row_peers_mask.count_ones() == BOARD_LENGTH as u32 - 1);
        assert!(col_peers_mask.count_ones() == BOARD_LENGTH as u32 - 1);
        assert!(box_peers_mask.count_ones() == BOARD_LENGTH as u32 - 1);

        let combined = row_peers_mask | col_peers_mask | box_peers_mask;
        assert!(combined.count_ones() == TOTAL_PEERS_PER_CELL as u32);

        all_peers_mask_lookup_table[position_id as usize] = combined;

        position_id += 1;
    }

    return all_peers_mask_lookup_table;
}