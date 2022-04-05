#[macro_use]
mod macros {
    macro_rules! min {
        ($lerror_a: expr, $lpos_a: expr, $($lerror_b: expr, $lpos_b: expr), +) => {
            {
                let mut lerror = $lerror_a;
                let mut lpos = $lpos_a;
                $({
                    if $lerror_b < lerror {
                        lerror = $lerror_b;
                        lpos = $lpos_b;
                    }
                })*
                (lerror, lpos)
            }
        }
    }
}
