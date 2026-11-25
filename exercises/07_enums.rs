// ============================================================
// 07: 列挙型 (enum) と match・Option
// ============================================================
//
// ■ enum: 「いくつかの候補のうちどれか 1 つ」を表す型
//   Rust の enum は、それぞれの候補 (バリアント) にデータを持たせられます。
//     enum Message {
//         Quit,                       // データなし
//         Move { x: i32, y: i32 },    // 名前付きフィールド
//         Write(String),              // タプル風
//     }
//
// ■ match: パターンで分岐する
//     match msg {
//         Message::Quit => ...,
//         Message::Move { x, y } => ...,   // 中のデータを取り出せる
//         Message::Write(text) => ...,
//     }
//   ・すべての候補を網羅しないとコンパイルエラー (書き忘れを防げる！)
//   ・`_ => ...` は「それ以外ぜんぶ」
//   ・範囲 `1..=9`、ガード `n if n < 0`、複数 `1 | 2` なども書けます
//
// ■ Option<T>: 「値があるかもしれないし、ないかもしれない」
//     enum Option<T> { Some(T), None }    ← 標準ライブラリにある普通の enum
//   Rust には null がありません。代わりに Option を使うので、
//   「値がないケース」の処理し忘れをコンパイラが教えてくれます。
//
// ■ if let: 1 つのパターンだけ扱いたいときの短い書き方
//     if let Some(x) = maybe { ... } else { ... }
//
// ------------------------------------------------------------
// 課題: コンパイルエラーを直し、todo!() を実装してください。
// ------------------------------------------------------------

enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    // TODO: match が網羅的でないのでエラーになります。Triangle の場合を追加してください。
    //       三角形の面積 = 底辺 × 高さ ÷ 2
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
        }
    }
}

// TODO: match のパターン (範囲・ガード) を使って数を分類してください。
//   負の数 → "negative"、0 → "zero"、1〜9 → "small"、10 以上 → "large"
fn classify(n: i32) -> &'static str {
    todo!()
}

// TODO: b が 0 なら None、そうでなければ Some(a / b) を返す
fn safe_divide(a: i32, b: i32) -> Option<i32> {
    todo!()
}

// TODO: None の場合が抜けているのでエラーです。
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(n) => Some(n + 1),
    }
}

// TODO: if let を使って、
//   name が Some なら "こんにちは、{名前}さん"、None なら "こんにちは、ゲストさん" を返す
fn greet(name: Option<&str>) -> String {
    todo!()
}

// ------------------------------------------------------------
// テスト (ここは変更しないでください)
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let c = Shape::Circle { radius: 1.0 };
        let r = Shape::Rectangle { width: 3.0, height: 4.0 };
        let t = Shape::Triangle { base: 6.0, height: 5.0 };
        assert!((c.area() - 3.14159).abs() < 0.001);
        assert_eq!(r.area(), 12.0);
        assert_eq!(t.area(), 15.0);
    }

    #[test]
    fn test_classify() {
        assert_eq!(classify(-5), "negative");
        assert_eq!(classify(0), "zero");
        assert_eq!(classify(1), "small");
        assert_eq!(classify(9), "small");
        assert_eq!(classify(10), "large");
        assert_eq!(classify(12345), "large");
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(7, 0), None);
    }

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(Some(41)), Some(42));
        assert_eq!(plus_one(None), None);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet(Some("Ferris")), "こんにちは、Ferrisさん");
        assert_eq!(greet(None), "こんにちは、ゲストさん");
    }
}
