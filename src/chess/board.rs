use crate::chess::piece::*;

pub struct Color();
impl Color {
    const NUM_COLORS: usize = 2;

    const WHITE: usize = 0;
    const BLACK: usize = 1;
}

#[derive(Copy, Clone)]
pub struct Bitboard(pub i64);

pub struct Board {
    pub bb_side: [Bitboard; 2],         /* Bitboard for each side (WHITE and BLACK) */
    pub bb_pieces: [[Bitboard; Pieces::NUM_PIECES]; Color::NUM_COLORS]   /* Bitboard for each piece (KING, QUEEN, ROOK, BISHOP, KNIGHT, PAWN) */
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            bb_side: [Bitboard(0); 2],
            bb_pieces: [[Bitboard(0); Pieces::NUM_PIECES]; Color::NUM_COLORS],
        };
        board.init();
        board
    }

    pub fn init(&mut self) {
        /* 
        White's starting position is represented as a bitboard where the 16 least significant bits are set to 1:
        
        00000000 (A8)
        00000000 (A7)
        00000000 (A6)
        00000000 (A5)
        00000000 (A4)
        00000000 (A3)
        11111111 (A2)
        11111111 (A1)

        Black's starting position is the opposite. The 16 most significant bits are set to 1.

        We can do the same for the starting position of each piece.
        */
        self.bb_side[Color::WHITE] = Bitboard(0x0000FFFF);
        self.bb_pieces[Color::WHITE][Pieces::KING] = Bitboard(0x00000010);
        self.bb_pieces[Color::WHITE][Pieces::QUEEN] = Bitboard(0x00000008);
        self.bb_pieces[Color::WHITE][Pieces::ROOK] = Bitboard(0x00000081);
        self.bb_pieces[Color::WHITE][Pieces::BISHOP] = Bitboard(0x00000024);
        self.bb_pieces[Color::WHITE][Pieces::KNIGHT] = Bitboard(0x00000042);
        self.bb_pieces[Color::WHITE][Pieces::PAWN] = Bitboard(0x0000FF00);
        
        self.bb_side[Color::BLACK] = Bitboard(0xFFFF0000);
        self.bb_pieces[Color::BLACK][Pieces::KING] = Bitboard(0x10000000);
        self.bb_pieces[Color::BLACK][Pieces::QUEEN] = Bitboard(0x80000000);
        self.bb_pieces[Color::BLACK][Pieces::ROOK] = Bitboard(0x81000000);
        self.bb_pieces[Color::BLACK][Pieces::BISHOP] = Bitboard(0x24000000);
        self.bb_pieces[Color::BLACK][Pieces::KNIGHT] = Bitboard(0x42000000);
        self.bb_pieces[Color::BLACK][Pieces::PAWN] = Bitboard(0xFF00000000);

    }

    pub fn print(&mut self, bb: Bitboard) {
        let state: i64 = bb.0;

        for rank in (0..8).rev() {
            for file in 0..8 {
                let pos= rank * 8 + file;
                let ch= if ((state >> pos) & 1) == 1 { '1' } else { '0' };

                print!("{ch}");
            }
            println!();
        }

    }
}