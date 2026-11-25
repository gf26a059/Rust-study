// 【解答例】これは一つの書き方です。他の書き方でもテストが通れば正解です！
// ============================================================
// 13: ライフタイム
// ============================================================
//
// ■ ライフタイム = 参照が有効な期間
//   Rust は「もう破棄された値を指す参照 (ダングリング参照)」を絶対に許しません。
//   そのために、参照がいつまで有効かをコンパイラがチェックしています。
//
// ■ ほとんどの場合は自動で推論される (省略規則)
//     fn first(s: &str) -> &str      // 引数の参照が 1 つ → 戻り値はそれと同じ寿命、と推論
//
// ■ 注釈が必要なケース
//   引数に参照が 2 つ以上あり、参照を返すとき、
//   「戻り値はどちらの引数と同じだけ生きるのか」をコンパイラは判断できません。
//     fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
//   読み方: 「戻り値は、a と b の両方が生きている間だけ有効」
//   ※ 'a は寿命を伸ばすものではなく、関係を「説明する」ラベルです。
//
// ■ 構造体に参照を持たせる場合も必要
//     struct Excerpt<'a> { part: &'a str }
//     impl<'a> Excerpt<'a> { ... }
//   意味: 「Excerpt は、part が指す文字列より長生きできない」
//
// ■ 'static: プログラムの最後まで有効 (文字列リテラル "..." は &'static str)
//
// ------------------------------------------------------------
// 課題: ライフタイム注釈を付けてコンパイルを通し、テストを通してください。
// ------------------------------------------------------------

// TODO: ライフタイム注釈を付けてください
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// TODO: 参照を持つ構造体です。ライフタイムを付けてください。
struct Highlight<'a> {
    text: &'a str,
}

// TODO: impl にもライフタイムが必要です
impl<'a> Highlight<'a> {
    // 文章の最初の文 ('.' の手前まで。'.' がなければ全体) を切り出す
    fn first_sentence(text: &'a str) -> Highlight<'a> {
        let end = text.find('.').unwrap_or(text.len());
        Highlight { text: &text[..end] }
    }

    fn len(&self) -> usize {
        self.text.len()
    }
}

// TODO: 先頭の '#' をすべて取り除いた部分を返す ("##見出し" → "見出し")
//   引数の参照が 1 つだけなので、ライフタイム注釈は不要です (省略規則)。
//   ヒント: s.trim_start_matches('#')
fn trim_hashes(s: &str) -> &str {
    s.trim_start_matches('#')
}

// ------------------------------------------------------------
// テスト (ここは変更しないでください)
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("hello", "hi"), "hello");
        assert_eq!(longest("a", "abc"), "abc");

        let s1 = String::from("long string is long");
        let result;
        {
            let s2 = String::from("xyz");
            // result は s2 のスコープの外でも使うので、コピーした String にしておく
            result = longest(s1.as_str(), s2.as_str()).to_string();
        }
        assert_eq!(result, "long string is long");
    }

    #[test]
    fn test_highlight() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let h = Highlight::first_sentence(&novel);
        assert_eq!(h.text, "Call me Ishmael");
        assert_eq!(h.len(), 15);

        let h2 = Highlight::first_sentence("no period here");
        assert_eq!(h2.text, "no period here");
    }

    #[test]
    fn test_trim_hashes() {
        assert_eq!(trim_hashes("##見出し"), "見出し");
        assert_eq!(trim_hashes("text"), "text");
        assert_eq!(trim_hashes("###"), "");
    }
}
