use rust_krabs::engine::Engine;

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let mut engine = Engine::new(DEFAULT_FEN);

    engine.evaluate();
}
