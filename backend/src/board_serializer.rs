use crate::board::Board;
use crate::piece_rules::StandardChess;
use crate::piece_serializer::piece_serialize;

pub fn board_serialize(b: &Board) -> String {
    let mut out = String::new();
    //let mut pieces = Vec::new();
    /*
    for piece in &b.pieces {
        pieces.push(piece_serialize(piece));
    }
    out += &format!("{{\"turn\": {}, \"wc\": {}, \"bc\": {}, \"pieces\": [{}], \"white_pawns\": [{}], \"black_pawns\": [{}]}}", b.turn, b.white_can_castle, b.black_can_castle, pieces.join(","), b.white_pawns.to_string(), b.black_pawns.to_string());
    */
    out
}