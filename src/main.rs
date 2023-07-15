use tardigrade::{analyzer::Evaluator, board::Board};
fn main() {
    let board =
        Board::from_fen("r3k2r/p1ppqpb1/bnN1pnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 1 1");

    board.print();
    let _res = Evaluator::perft(&board, 1, true);

    /*

    for i in 0..4 {
        let _res = Evaluator::perft(&board, i, true);
        println!("depth: {}, {:?}", i, _res);
    }
    let mvs = MoveGenerator::gen_legal_moves(&board);
    for mv in mvs {
        mv.print();
    }
    */
}
