mod functions;
mod integers;
use functions::test_functions;
use integers::overflow;

fn main() {
    overflow();
    test_functions();
}
