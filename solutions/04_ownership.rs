// 【解答例】これは一つの書き方です。他の書き方でもテストが通れば正解です！
// ============================================================
// 04: 所有権とムーブ  ★Rust でいちばん大事なところ★
// ============================================================
//
// ■ 所有権の 3 つのルール
//   1. 値には「所有者」となる変数がある
//   2. 所有者は同時に 1 つだけ
//   3. 所有者がスコープ ({ } の範囲) を抜けると、値は自動で破棄 (drop) される
//   → これのおかげで、ガベージコレクタなしでメモリが安全に管理されます。
//
// ■ ムーブ (所有権の移動)
//     let s1 = String::from("hi");
//     let s2 = s1;          // s1 → s2 に所有権が「移動」
//     println!("{s1}");     // ← エラー！ s1 はもう空っぽ扱い
//
//   関数に渡すのもムーブです:
//     takes(s2);            // 以後 s2 は使えない
//
// ■ コピー
//   i32, f64, bool, char などの小さな型は「Copy」なので、代入しても元が使えます。
//     let a = 5;
//     let b = a;            // a もまだ使える
//
// ■ クローン
//   String や Vec を本当に複製したいときは .clone() (ヒープのデータごと複製)
//     let s3 = s2.clone();
//
// ------------------------------------------------------------
// 課題: コンパイルエラーを直し、テストを通してください。
// ------------------------------------------------------------

fn count(v: Vec<i32>) -> usize {
    v.len()
}

// TODO: v を 2 回 count に渡したいのに、1 回目でムーブされてしまいます。
//       .clone() を使って直してください。
fn count_twice() -> (usize, usize) {
    let v = vec![1, 2, 3];
    let a = count(v.clone());
    let b = count(v);
    (a, b)
}

// TODO: s1 の所有権は s2 に移っています。s2 を使うように直してください。
//       (書き換えるので mut も必要です)
fn append_world() -> String {
    let s1 = String::from("hello");
    let mut s2 = s1;
    s2.push_str(", world");
    s2
}

// 関数は所有権を受け取って、また「返す」こともできます。
// TODO: v に 4 を push して、そのまま返してください。
fn add_four(mut v: Vec<i32>) -> Vec<i32> {
    v.push(4);
    v
}

// TODO: スコープと drop。
//   内側の { } で作った String を、外に持ち出したい。
//   今は inner がスコープを抜けると破棄されるのでエラーになります。
//   inner を「ムーブ」して外の変数 outer に渡すように直してください。
fn escape_scope() -> String {
    let outer: String;
    {
        let inner = String::from("こんにちは");
        outer = inner;
    }
    outer
}

// ------------------------------------------------------------
// テスト (ここは変更しないでください)
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_twice() {
        assert_eq!(count_twice(), (3, 3));
    }

    #[test]
    fn test_append_world() {
        assert_eq!(append_world(), "hello, world");
    }

    #[test]
    fn test_add_four() {
        assert_eq!(add_four(vec![1, 2, 3]), vec![1, 2, 3, 4]);
        assert_eq!(add_four(Vec::new()), vec![4]);
    }

    #[test]
    fn test_escape_scope() {
        assert_eq!(escape_scope(), "こんにちは");
    }
}
