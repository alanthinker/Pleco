// A move needs 16 bits to be stored
//
// bit  0- 5: destination square (from 0 to 63)
// bit  6-11: origin square (from 0 to 63)
// bit 12-13: promotion piece type - 2 (from KNIGHT-2 to QUEEN-2)
// bit 14-15: special move flag: promotion (1), en passant (2), castling (3)
// NOTE: EN-PASSANT bit is set only when a pawn can be captured
//
// Special cases are MOVE_NONE and MOVE_NULL. We can sneak these in because in
// any normal move destination square is always different from origin square
// while MOVE_NONE and MOVE_NULL have the same origin and destination square.

// x??? --> Promotion bit
// ?x?? --> Capture bit
// ??xx --> flaf Bit

// 0000  ===> Quiet move
// 0001  ===> Double Pawn Push
// 0010  ===> King Castle
// 0011  ===> Queen Castle
// 0100  ===> Capture
// 0101  ===> EP Capture
// 0110  ===>
// 0111  ===>
// 1000  ===> Knight Promotion
// 1001  ===> Bishop Promo
// 1010  ===> Rook   Promo
// 1011  ===> Queen  Capture  Promo
// 1100  ===> Knight Capture  Promotion
// 1101  ===> Bishop Capture  Promo
// 1110  ===> Rook   Capture  Promo
// 1111  ===> Queen  Capture  Promo

use templates::SQ;
use templates::Piece;
use std::mem;

static SRC_MASK: u16 = 0b0000000000111111;
static DST_MASK: u16 = 0b0000111111000000;
static PR_MASK: u16 = 0b1000000000000000;
static CP_MASK: u16 = 0b0100000000000000;
static FLAG_MASK: u16 = 0b1111000000000000;
static SP_MASK: u16 = 0b0011000000000000;

#[derive(Copy, Clone)]
pub struct BitMove {
    data: u16,
}

#[derive(Copy, Clone)]
pub enum MoveFlag {
    Promotion { capture: bool, prom: Piece },
    Castle { king_side: bool },
    DoublePawnPush,
    Capture { ep_capture: bool },
    QuietMove,
}

#[derive(Copy, Clone)]
pub struct PreMoveInfo {
    pub src: SQ,
    pub dst: SQ,
    pub flags: MoveFlag,
}

// https://chessprogramming.wikispaces.com/Encoding+Moves
impl BitMove {
    pub fn new(input: u16) -> BitMove {
        let mut bit_move = BitMove { data: input };
        bit_move
    }

    pub fn init(info: PreMoveInfo) -> BitMove {
        let src = info.src as u16;
        let dst = (info.dst as u16) << 6;
        let flags = info.flags;
        let flag_bits: u16 = match flags {
            MoveFlag::Promotion { capture, prom } => {
                let p_bit: u16 = match prom {
                    Piece::Q => { 3 }
                    Piece::R => { 2 }
                    Piece::B => { 1 }
                    Piece::N => { 0 }
                    _ => { 3 }
                };
                let cp_bit = match capture {
                    true => { 4 }
                    false => { 0 }
                };
                p_bit + cp_bit + 8
            }
            MoveFlag::Capture { ep_capture } => {
                match ep_capture {
                    true => 5,
                    _ => 4
                }
            }
            MoveFlag::Castle { king_side } => {
                match king_side {
                    true => 2,
                    _ => 3
                }
            }
            MoveFlag::DoublePawnPush => { 1 }
            MoveFlag::QuietMove => { 0 }
            _ => { 0 }
        };
        let mut bit_move = BitMove { data: (flag_bits << 12) | src | dst };
        bit_move
    }

    // Note: Encompasses two missing Spots
    pub fn is_capture(&self) -> bool { ((&self.data & CP_MASK) >> 14) == 1 }

    pub fn is_promo(&self) -> bool { ((&self.data & PR_MASK) >> 15) == 1 }
    pub fn get_dest(&self) -> u8 { ((&self.data & DST_MASK) >> 6) as u8 }
    pub fn get_src(&self) -> u8 { (&self.data & SRC_MASK) as u8 }
    pub fn is_castle(&self) -> bool { ((&self.data & FLAG_MASK) >> 13) == 1 }
    pub fn is_king_castle(&self) -> bool { ((&self.data & FLAG_MASK) >> 12) == 2 }
    pub fn is_queen_castle(&self) -> bool { ((&self.data & FLAG_MASK) >> 12) == 3 }
    pub fn is_en_passant(&self) -> bool { (&self.data & FLAG_MASK) >> 12 == 5 }
    pub fn is_double_push(&self) -> (bool, u8) {
        let is_double_push: u8 = (&self.data & FLAG_MASK) as u8;
        match is_double_push {
            1 => (true, self.get_dest() as u8),
            _ => (false, 64)
        }
    }
    pub fn dest_row(&self) -> u8 {
        ((self.data & DST_MASK) >> 6) as u8 / 8
    }
    pub fn dest_col(&self) -> u8 {
        ((self.data & DST_MASK) >> 6) as u8 % 8
    }
    pub fn src_row(&self) -> u8 {
        (self.data & SRC_MASK) as u8 / 8
    }
    pub fn src_col(&self) -> u8 {
        (self.data & SRC_MASK) as u8 % 8
    }
    pub fn promo_piece(&self) -> Piece {
        match (self.data >> 12) & 0b0011 {
            0 => Piece::N,
            1 => Piece::B,
            2 => Piece::R,
            3 => Piece::Q,
            _ => Piece::Q,
        }
    }
}
