// 【解答例】これは一つの書き方です。他の書き方でもテストが通れば正解です！
// ============================================================
// 09: エラー処理 (Result と ? 演算子)
// ============================================================
//
// ■ 回復不能なエラー: panic!("メッセージ")
//   プログラムがその場で止まります。「バグ」を表すときに使います。
//
// ■ 回復可能なエラー: Result<T, E>
//     enum Result<T, E> { Ok(T), Err(E) }
//   例: "42".parse::<i32>()   → Ok(42)
//       "abc".parse::<i32>()  → Err(ParseIntError)
//   失敗するかもしれない処理は Result を返し、呼び出し側に対処を任せます。
//
// ■ ? 演算子
//     let n = s.parse::<i32>()?;
//   Ok なら中身を取り出し、Err ならその場で `return Err(...)` します。
//   ※ ? は Result (または Option) を返す関数の中でしか使えません。
//
// ■ unwrap() / expect("msg")
//   Err (や None) だと panic します。手軽ですが、本番コードでは慎重に。
//
// ■ map_err: エラーの型を変換する
//     s.parse::<i32>().map_err(|_| MyError::Bad)?
//
// ------------------------------------------------------------
// 課題: todo!() を実装して、テストを通してください。
// ------------------------------------------------------------

use std::num::ParseIntError;

// TODO: 文字列を i32 にパースして、2 倍にして返す。
//       まずは match で Ok / Err を分けて書いてみよう。
fn double(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n * 2),
        Err(e) => Err(e),
    }
}

// TODO: 2 つの文字列をパースして足し算する。今度は ? 演算子を使ってみよう。
fn add_strs(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}

// 独自のエラー型
#[derive(Debug, PartialEq)]
enum AgeError {
    NotANumber,
    Negative,
    TooOld,
}

// TODO: 年齢をパースする。
//   ・数字でない   → Err(AgeError::NotANumber)
//   ・負の数       → Err(AgeError::Negative)
//   ・150 より大きい → Err(AgeError::TooOld)
//   ・それ以外     → Ok(年齢)
//   ヒント: まず i64 としてパースし、map_err でエラー型を変換して ? で返すと楽です。
fn parse_age(s: &str) -> Result<u32, AgeError> {
    let n = s.parse::<i64>().map_err(|_| AgeError::NotANumber)?;
    if n < 0 {
        Err(AgeError::Negative)
    } else if n > 150 {
        Err(AgeError::TooOld)
    } else {
        Ok(n as u32)
    }
}

// ? は Option にも使えます (None ならその場で return None)
// TODO: 文章の「最初の単語」の「最初の文字」を返す。単語がなければ None
//   ヒント: text.split_whitespace().next()? で最初の単語が取れます
fn first_initial(text: &str) -> Option<char> {
    let word = text.split_whitespace().next()?;
    word.chars().next()
}

// ------------------------------------------------------------
// テスト (ここは変更しないでください)
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
        assert_eq!(double("21"), Ok(42));
        assert_eq!(double("-5"), Ok(-10));
        assert!(double("abc").is_err());
    }

    #[test]
    fn test_add_strs() {
        assert_eq!(add_strs("1", "2"), Ok(3));
        assert!(add_strs("1", "x").is_err());
        assert!(add_strs("y", "2").is_err());
    }

    #[test]
    fn test_parse_age() {
        assert_eq!(parse_age("20"), Ok(20));
        assert_eq!(parse_age("0"), Ok(0));
        assert_eq!(parse_age("150"), Ok(150));
        assert_eq!(parse_age("151"), Err(AgeError::TooOld));
        assert_eq!(parse_age("-1"), Err(AgeError::Negative));
        assert_eq!(parse_age("twenty"), Err(AgeError::NotANumber));
    }

    #[test]
    fn test_first_initial() {
        assert_eq!(first_initial("hello world"), Some('h'));
        assert_eq!(first_initial("   rust"), Some('r'));
        assert_eq!(first_initial("こんにちは 世界"), Some('こ'));
        assert_eq!(first_initial("   "), None);
        assert_eq!(first_initial(""), None);
    }
}
