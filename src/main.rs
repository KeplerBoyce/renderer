mod test_window;
mod texture;
use test_window::run;

fn main() {
    pollster::block_on(run());
}
