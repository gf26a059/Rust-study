// ============================================================
// 08: コレクション (Vec / String / HashMap)
// ============================================================
//
// ■ Vec<T>: 伸び縮みする配列
//     let mut v = Vec::new();   v.push(1);   v.len();
//     let v = vec![1, 2, 3];
//     v[0]        … 範囲外だと panic
//     v.get(10)   … 範囲外なら None (Option<&T>)
//     for x in &v { ... }      … 借用して順に読む
//
// ■ String: 伸び縮みする UTF-8 文字列 (&str はその「参照/スライス」)
//     let mut s = String::from("ab");   s.push('c');   s.push_str("de");
//     s.chars()          … 1 文字 (char) ずつのイテレータ
//     s.to_uppercase()   … 大文字にした新しい String
//     format!("{a}-{b}") … 文字列の組み立て
//   ⚠ s[0] のような添字アクセスはできません
//     (UTF-8 では 1 文字のバイト数がバラバラなので)
//
// ■ HashMap<K, V>: キーと値の対応表
//     use std::collections::HashMap;
//     let mut m = HashMap::new();
//     m.insert("apple", 3);
//     m.get("apple")                       … Option<&V>
//     *m.entry("apple").or_insert(0) += 1; … 「なければ 0 を入れてから +1」カウントの定番
//
// ------------------------------------------------------------
// 課題: todo!() を実装して、テストを通してください。
// ------------------------------------------------------------

use std::collections::HashMap;

// TODO: 偶数だけを集めた新しい Vec を返す
fn evens(values: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    // TODO
    result
}

// TODO: 先頭の 1 文字だけ大文字にする ("rust" → "Rust", "" → "")
//   ヒント: let mut chars = s.chars();
//           chars.next() で最初の文字 (Option<char>) が取れ、
//           chars.as_str() で残りの部分 (&str) が取れます。
//           c.to_uppercase() は「大文字の文字たち」なので .to_string() か .collect::<String>() で String に。
fn capitalize(s: &str) -> String {
    todo!()
}

// TODO: 文字列中の単語 (空白区切り) の出現回数を数える
//   ヒント: text.split_whitespace() で単語ごとに取り出せます
fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    // TODO
    counts
}

// TODO: name の点数を返す。登録されていなければ 0
//   ヒント: get は Option<&u32> を返します。
//           match で書いてもいいし、.copied().unwrap_or(0) でも OK
fn score_of(scores: &HashMap<String, u32>, name: &str) -> u32 {
    todo!()
}

// ------------------------------------------------------------
// テスト (ここは変更しないでください)
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evens() {
        assert_eq!(evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(evens(&[1, 3, 5]), Vec::<i32>::new());
        assert_eq!(evens(&[-2, 0, 7]), vec![-2, 0]);
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("rust"), "Rust");
        assert_eq!(capitalize("hello world"), "Hello world");
        assert_eq!(capitalize("A"), "A");
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn test_word_count() {
        let counts = word_count("the cat and the hat and the bat");
        assert_eq!(counts["the"], 3);
        assert_eq!(counts["and"], 2);
        assert_eq!(counts["cat"], 1);
        assert_eq!(counts.get("dog"), None);
        assert_eq!(counts.len(), 5);
    }

    #[test]
    fn test_score_of() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Alice"), 90);
        scores.insert(String::from("Bob"), 75);
        assert_eq!(score_of(&scores, "Alice"), 90);
        assert_eq!(score_of(&scores, "Bob"), 75);
        assert_eq!(score_of(&scores, "Carol"), 0);
    }
}
