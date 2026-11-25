// 【解答例】これは一つの書き方です。他の書き方でもテストが通れば正解です！
// ============================================================
// 03: 制御フロー (if / loop / while / for)
// ============================================================
//
// ■ if は「式」なので値を返せる
//     let label = if n > 0 { "正" } else { "0 か負" };
//   ※ 条件は必ず bool。`if n { ... }` のように数値は使えません。
//
// ■ ループ 3 兄弟
//     loop { ... }                 無限ループ。`break 値;` でループ全体が値になる
//     while 条件 { ... }           条件が true の間くり返す
//     for i in 1..=10 { ... }      範囲: 1..=10 は 10 を含む / 1..10 は含まない
//     for x in [10, 20, 30] { ... } 配列や Vec の要素を順番に
//
// ■ 余り: n % 3 == 0 なら 3 の倍数
//
// ------------------------------------------------------------
// 課題: todo!() を実装して、テストを通してください。
// ------------------------------------------------------------

// TODO: FizzBuzz
//   3 の倍数なら "Fizz"、5 の倍数なら "Buzz"、両方の倍数なら "FizzBuzz"、
//   それ以外は数字をそのまま文字列にして返す。
//   ヒント: 数値 → String は n.to_string()、"Fizz" → String は "Fizz".to_string()
fn fizzbuzz(n: u32) -> String {
    if n % 15 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}

// TODO: for と範囲を使って、1 から n までの合計を返す
fn sum_to(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

// TODO: loop と break を使って、limit を「超える」最初の 2 のべき乗を返す
//   2 のべき乗: 1, 2, 4, 8, 16, 32, 64, 128, ...
//   例: first_power_of_two_over(100) == 128
fn first_power_of_two_over(limit: u32) -> u32 {
    let mut n = 1;
    let result = loop {
        if n > limit {
            break n;
        }
        n *= 2;
    };
    result
}

// TODO: while を使って「コラッツ数列」が 1 になるまでの手数を数える
//   ・n が偶数なら n / 2
//   ・n が奇数なら 3 * n + 1
//   これを n が 1 になるまでくり返す。
//   例: 6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1 なので 8 手
//   ヒント: 引数を書き換えたいときは `fn collatz_steps(mut n: u64)` にできます
fn collatz_steps(mut n: u64) -> u32 {
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
    steps
}

// ------------------------------------------------------------
// テスト (ここは変更しないでください)
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(98), "98");
    }

    #[test]
    fn test_sum_to() {
        assert_eq!(sum_to(1), 1);
        assert_eq!(sum_to(10), 55);
        assert_eq!(sum_to(100), 5050);
    }

    #[test]
    fn test_power_of_two() {
        assert_eq!(first_power_of_two_over(100), 128);
        assert_eq!(first_power_of_two_over(0), 1);
        assert_eq!(first_power_of_two_over(1024), 2048);
    }

    #[test]
    fn test_collatz() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(27), 111);
    }
}
