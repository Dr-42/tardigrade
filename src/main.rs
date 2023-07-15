use tardigrade::{
    analyzer::Evaluator,
    board::{Board, Move, MoveType},
};
fn main() {
    let mut board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");

    board.print();
    for i in 0..9 {
        let _res = Evaluator::perft(&board, i, false);
        println!("depth: {}, {:?}", i, _res);
    }
    /*
    let _res = Evaluator::perft(&board, 2, true);
    let mvs = MoveGenerator::gen_legal_moves(&board);
    for mv in mvs {
        mv.print();
    }
    */
}
