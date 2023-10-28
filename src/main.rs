/// Performs modulo addition on an arbitrary number of inputs.
/// Call like this: madd!(MOD_FACTOR; ...elements);
/// ex:
/// assert!(madd!(10; 4, 5, 6, 7) == (4 + 5 + 6 + 7) % 10);
macro_rules! madd {
    ($mod_factor:expr; $($x:expr),*) => {
        {
            let mut total = 0;
            let mod_factor = $mod_factor;
            $(
                total += ($x) % mod_factor;
                total %= mod_factor;
            )*
            total
        }

    };
}

fn main() {
    assert!(madd!(10; 4, 5, 6, 7) == (4 + 5 + 6 + 7) % 10);
}
