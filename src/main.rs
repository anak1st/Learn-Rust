use std::{io::Read, vec::IntoIter};

fn str_to_words(line: &str) -> Vec<String> {
    line.split_ascii_whitespace().map(str::to_string).collect()
}
struct Scanner {
    iter: IntoIter<String>,
}
impl Scanner {
    fn new() -> Scanner {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        return Scanner {
            iter : str_to_words(&buf).into_iter(),
        };
    }
    fn next_string(&mut self) -> String {
        return self.iter.next().unwrap();
    }
    fn next<T: std::str::FromStr>(&mut self) -> T
    where 
        <T as std::str::FromStr>::Err: std::fmt::Debug, 
    {
        return self.next_string().parse::<T>().unwrap();
    }
}

fn solve(scan : &mut Scanner) {
    let n = scan.next();
    let mut a : Vec<i64> = vec![0; n];
    for i in 0..n {
        a[i] = scan.next();
    }
    let ans : i64 = a.iter().sum();
    println!("{}", ans);
}

fn main() {
    let mut scan = Scanner::new();
    let mut t = 1;
    t = scan.next();
    while t > 0 {
        t -= 1;
        solve(&mut scan);
    }
}