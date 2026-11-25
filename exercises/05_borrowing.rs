// ============================================================
// 05: 参照と借用・スライス
// ============================================================
//
// ■ 参照 (借用)
//   所有権を渡さずに、値を「貸す」には & を使います。
//     fn len(s: &String) -> usize { s.len() }
//     let s = String::from("hi");
//     len(&s);              // 貸しただけなので s はまだ使える
//
// ■ 可変参照
//   貸した先で中身を書き換えてもらうには &mut を使います。
//     fn excite(s: &mut String) { s.push('!'); }
//     let mut s = String::from("hi");
//     excite(&mut s);       // s は "hi!" になる
//
// ■ 借用のルール (同時にはどちらか一方だけ)
//   ・不変参照 &T   … いくつでも同時に OK (みんなで読むだけなら安全)
//   ・可変参照 &mut T … 同時に 1 つだけ   (書き換え中に他から触られない)
//   → これでデータ競合がコンパイル時に防がれます。
//
// ■ スライス
//   &str  … 文字列の一部への参照       例: &s[0..3]
//   &[T]  … 配列や Vec の一部への参照  例: &v[1..]
//   引数は &String より &str、&Vec<T> より &[T] にすると、何でも受け取れて便利です。
//
// ------------------------------------------------------------
// 課題: コンパイルエラーを直し、テストを通してください。
// ------------------------------------------------------------

// TODO: 所有権を奪わないように、引数を参照 (&str) にしてください。
//       呼び出し側の borrow_and_return も直す必要があります。
fn length(s: String) -> usize {
    s.len()
}

fn borrow_and_return() -> (usize, String) {
    let s = String::from("rust");
    let n = length(s);
    (n, s)
}

// TODO: s の末尾に '!' を追加したいのですが…引数の型を直してください。
fn shout(s: &String) {
    s.push('!');
}

// TODO: 可変参照が同時に 2 つあるのでエラーになります。
//       r1 を使い終わってから r2 を作るように並べ替えてください。
fn two_edits() -> String {
    let mut s = String::from("a");
    let r1 = &mut s;
    let r2 = &mut s;
    r1.push('b');
    r2.push('c');
    s
}

// TODO: 文字列の最初の単語 (最初のスペースの手前まで) をスライスで返してください。
//       スペースがなければ文字列全体を返します。
//       ヒント: s.find(' ') はスペースの位置を Option<usize> で返します。
//               match s.find(' ') { Some(i) => ..., None => ... }
fn first_word(s: &str) -> &str {
    todo!()
}

// TODO: スライスの合計を返してください。配列でも Vec でも渡せます。
fn sum(values: &[i32]) -> i32 {
    todo!()
}

// ------------------------------------------------------------
// テスト (ここは変更しないでください)
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_and_return() {
        assert_eq!(borrow_and_return(), (4, String::from("rust")));
    }

    #[test]
    fn test_shout() {
        let mut s = String::from("hey");
        shout(&mut s);
        shout(&mut s);
        assert_eq!(s, "hey!!");
    }

    #[test]
    fn test_two_edits() {
        assert_eq!(two_edits(), "abc");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word(""), "");
        let owned = String::from("借用 は 楽しい");
        assert_eq!(first_word(&owned), "借用");
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3]), 6);
        let v = vec![10, 20, 30, 40];
        assert_eq!(sum(&v), 100);
        assert_eq!(sum(&v[1..3]), 50);
        assert_eq!(sum(&[]), 0);
    }
}
