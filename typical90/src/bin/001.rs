use proconio::input;

fn main() {
    input! {
        n: usize, l: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.push(l);

    let mut ok = 0;
    let mut ng = l;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        let mut counter = 0;
        let mut current = 0;
        for &val in a.iter() {
            if val >= current + mid {
                current = val;
                counter += 1;
            }
        }

        if counter > k {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
