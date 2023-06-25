mod conditions;
mod functions;
mod integers;
use conditions::test_conditions;
use functions::test_functions;
use integers::overflow;

fn main() {
    overflow();
    test_functions();
    test_conditions();
}
