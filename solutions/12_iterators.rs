// 【解答例】これは一つの書き方です。他の書き方でもテストが通れば正解です！
// ============================================================
// 12: クロージャとイテレータ
// ============================================================
//
// ■ クロージャ = その場で作る無名関数。周りの変数を「キャプチャ」できる
//     let add = |a, b| a + b;          add(1, 2) == 3
//     let offset = 10;
//     let shift = |x| x + offset;      // offset をキャプチャ
//     let f = move |x| x + offset;     // move: キャプチャした値の所有権ごと持っていく
//
//   クロージャの種類 (トレイト)
//     Fn    … キャプチャした値を読むだけ
//     FnMut … キャプチャした値を書き換える
//     FnOnce… キャプチャした値を消費する (1 回しか呼べない)
//
// ■ イテレータ = 要素を 1 つずつ取り出す仕組み
//     v.iter()        … &T を順に
//     v.into_iter()   … T を順に (v は消費される)
//     text.chars() / text.split_whitespace() / (1..=10) なども イテレータ
//
//   アダプタ (変換してまたイテレータを返す)
//     .map(|x| ...)      変換
//     .filter(|x| ...)   条件に合うものだけ
//     .enumerate()       (番号, 要素) のペアに
//     .zip(other)        2 つを並べてペアに
//     .rev() / .take(n) / .skip(n) ...
//
//   消費メソッド (ここで初めて実際に動く = 遅延評価)
//     .collect()  .sum()  .count()  .max()  .min()  .any(..)  .all(..)
//
//   例: 正の数だけ 2 倍して Vec に
//     let r: Vec<i32> = v.iter().filter(|x| **x > 0).map(|x| x * 2).collect();
//
// ------------------------------------------------------------
// 課題: todo!() をイテレータやクロージャを使って実装してください。
//       (for ループでも解けますが、ぜひメソッドチェーンに挑戦！)
// ------------------------------------------------------------

// TODO: すべての要素を 2 乗した Vec を返す (map + collect)
fn squares(v: &[i32]) -> Vec<i32> {
    v.iter().map(|x| x * x).collect()
}

// TODO: 奇数だけを合計する (filter + sum)
fn sum_of_odds(v: &[i32]) -> i32 {
    v.iter().filter(|x| *x % 2 != 0).sum()
}

// TODO: min_len 文字以上の単語がいくつあるか数える
//   ヒント: 文字数は w.chars().count() (len() はバイト数なので日本語だとズレます)
fn count_long_words(text: &str, min_len: usize) -> usize {
    text.split_whitespace()
        .filter(|w| w.chars().count() >= min_len)
        .count()
}

struct Person {
    name: String,
    age: u32,
}

// TODO: 20 歳以上の人の名前だけを集める
fn adult_names(people: &[Person]) -> Vec<String> {
    people
        .iter()
        .filter(|p| p.age >= 20)
        .map(|p| p.name.clone())
        .collect()
}

// TODO: 2 つのスライスの内積 (a[0]*b[0] + a[1]*b[1] + ...) を返す (zip を使おう)
fn dot(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

// TODO: 「x に n を足す」クロージャを返す関数。
//       今は x をそのまま返すだけになっています。
//       n を使うように直すと、あるエラーが出るはず。メッセージをよく読もう！
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // move で n の値をクロージャの中に持っていく
    move |x| x + n
}

// TODO: FnMut の練習。increment を呼ぶたびに count が 1 増えるようにする
fn count_calls() -> u32 {
    let mut count = 0;
    let mut increment = || {
        count += 1;
    };
    increment();
    increment();
    increment();
    count
}

// ------------------------------------------------------------
// テスト (ここは変更しないでください)
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squares() {
        assert_eq!(squares(&[1, 2, 3, -4]), vec![1, 4, 9, 16]);
        assert_eq!(squares(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_sum_of_odds() {
        assert_eq!(sum_of_odds(&[1, 2, 3, 4, 5]), 9);
        assert_eq!(sum_of_odds(&[2, 4]), 0);
        assert_eq!(sum_of_odds(&[-3, 3, 7]), 7);
    }

    #[test]
    fn test_count_long_words() {
        assert_eq!(count_long_words("a quick brown fox jumps", 5), 3);
        assert_eq!(count_long_words("Rust は とても たのしい", 3), 3);
        assert_eq!(count_long_words("", 1), 0);
    }

    #[test]
    fn test_adult_names() {
        let people = vec![
            Person { name: "Alice".to_string(), age: 25 },
            Person { name: "Bob".to_string(), age: 17 },
            Person { name: "Carol".to_string(), age: 20 },
        ];
        assert_eq!(adult_names(&people), vec!["Alice", "Carol"]);
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(&[1, 2, 3], &[4, 5, 6]), 32);
        assert_eq!(dot(&[], &[]), 0);
    }

    #[test]
    fn test_make_adder() {
        let add5 = make_adder(5);
        let add100 = make_adder(100);
        assert_eq!(add5(1), 6);
        assert_eq!(add100(1), 101);
        let results: Vec<i32> = [1, 2, 3].iter().map(|x| add5(*x)).collect();
        assert_eq!(results, vec![6, 7, 8]);
    }

    #[test]
    fn test_count_calls() {
        assert_eq!(count_calls(), 3);
    }
}
