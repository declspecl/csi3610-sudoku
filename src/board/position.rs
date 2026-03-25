use crate::board::board::BOARD_LENGTH;


/// Unique ID of a position on the board (0-80)
pub type PositionId = u8;

/// The ID of a box on the board (1-9), where 1 is the top left box and 9 is the bottom right box
pub type BoxId = u8;

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


/// Bitmask of the peers of a position
/// (PeersMask >> PositionId) & 0b1 == 0b1 means the position is a peer of the position with the given ID
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
    const BOX_ID_BY_POSITION_ID_LUT: LookupTableByPositionId<u8> = build_box_id_by_position_id_lut();
    const ALL_PEERS_MASK_BY_POSITION_ID_LUT: LookupTableByPositionId<PeersMaskBits> = build_all_peers_mask_by_position_id_lut();

    pub const fn new(row: u8, col: u8) -> Self {
        let row_offset = (row - 1) * BOARD_LENGTH;
        let col_offset = col - 1;

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

    pub const fn row(self) -> u8 {
        return self.id / BOARD_LENGTH + 1;
    }

    pub const fn col(self) -> u8 {
        return self.id % BOARD_LENGTH + 1;
    }

    pub const fn box_id(self) -> u8 {
        return Self::BOX_ID_BY_POSITION_ID_LUT[self.id as usize];
    }
}

/// build lookup table that answers "what box is this cell in?" for each cell
const fn build_box_id_by_position_id_lut() -> LookupTableByPositionId<u8> {
    let mut table: LookupTableByPositionId<u8> = [0; TOTAL_POSITIONS as usize];

    let mut position_id: PositionId = 0;
    while position_id <= MAX_POSITION_ID {
        let position = Position::from_id(position_id);
        let row0 = (position.row() - 1) / 3;
        let col0 = (position.col() - 1) / 3;

        table[position_id as usize] = row0 * 3 + col0 + 1;
        position_id += 1;
    }

    return table;
}

/// build lookup table that answers "what are the all the peer cell IDs of this cell?" for each cell
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

                if peer_position.row() == position.row() {
                    row_peers_mask |= 1 << peer_id;
                }

                if peer_position.col() == position.col() {
                    col_peers_mask |= 1 << peer_id;
                }

                if peer_position.box_id() == position.box_id() {
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
