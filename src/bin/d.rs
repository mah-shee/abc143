#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut l: [isize; n],
    }
    l.sort();
    let mut ans = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                if l[i] + l[j] > l[k] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
