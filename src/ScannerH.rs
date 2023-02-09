use std::{io::Read, vec::IntoIter};

struct Scanner {
    iter: IntoIter<String>,
}
impl Scanner {
    fn new() -> Scanner {
        let mut source = Vec::new();
        std::io::stdin().read_to_end(&mut source).unwrap();
        return Scanner {
            iter: source
                .split(|x| (*x as char).is_ascii_whitespace())
                .map(|x| String::from_utf8(x.to_vec()).unwrap())
                .filter(|x| !x.is_empty())
                .collect::<Vec<String>>()
                .into_iter(),
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