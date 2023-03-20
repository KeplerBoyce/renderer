mod test_window;
use test_window::run;

fn main() {
    pollster::block_on(run());
}
