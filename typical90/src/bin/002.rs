use proconio::input;

fn check(x: usize, n: usize) -> Option<String> {
    let mut cnt = 0;
    let mut s = vec![];
    for i in 0..n {
        if x & (1 << i) != 0 {
            cnt += 1;
            s.push('(');
        } else {
            cnt -= 1;
            if cnt < 0 {
                return None;
            }
            s.push(')');
        }
    }
    if cnt == 0 {
        Some(s.into_iter().collect())
    } else {
        None
    }
}

fn main() {
    input! { n: usize }
    if n % 2 == 0 {
        let mut x = vec![];
        for i in 0..(1 << n) {
            if let Some(s) = check(i, n) {
                x.push(s);
            }
        }
        x.sort();
        println!("{}", x.join("\n"));
    }
}
