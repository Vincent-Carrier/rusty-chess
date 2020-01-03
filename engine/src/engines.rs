use base::moves::Move;
use base::state::State;

pub trait Engine {
  fn best_move(&mut self, state: &State) -> Move;
}
