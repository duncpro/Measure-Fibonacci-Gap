enum Fibonacci {
    First,
    Second,
    Rest(/* prev: */ f64, /* prev_prev: */ f64)
}

impl Iterator for Fibonacci {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_term: Self::Item = match self {
            Fibonacci::First => 1.0,
            Fibonacci::Second => 2.0,
            Fibonacci::Rest(prev, prev_prev) => *prev + *prev_prev,
        };

        let next_self = match self {
            Fibonacci::First => Fibonacci::Second,
            Fibonacci::Second => Fibonacci::Rest(1.0, 2.0),
            Fibonacci::Rest(prev, _prev_prev) 
                => Fibonacci::Rest(next_term, *prev),
        };

        *self = next_self;

        return Some(next_term);
    }
}

impl Default for Fibonacci {
    fn default() -> Self { Self::First }
}


fn f(n: usize) -> f64 {
    let steps = std::iter::zip(Fibonacci::default(), Fibonacci::default().skip(1))
        .take(n);

    let mut average = 0f64;

    for (prev, curr) in steps {
        let delta = curr - prev;
        let percent_change = (delta / prev);
        average += percent_change / (n as f64);
    }

    return average;
}

fn main() {
    let n = 1000;
    let avg = f(n);
    println!("n: {n}, avg: {avg}");
}
