// ============================================================
// 10: トレイト
// ============================================================
//
// ■ トレイト = 「この型はこういうことができる」という約束
//   (他の言語の「インターフェース」に近いものです)
//     trait Speak {
//         fn speak(&self) -> String;                 // 必ず実装してもらうメソッド
//         fn twice(&self) -> String {                // デフォルト実装つき (上書きも可)
//             format!("{0} {0}", self.speak())
//         }
//     }
//     impl Speak for Dog {
//         fn speak(&self) -> String { "ワン".to_string() }
//     }
//
// ■ トレイトを使う関数
//     fn talk(x: &impl Speak) { ... }        // Speak を実装した何か
//     fn talk<T: Speak>(x: &T) { ... }       // ↑と同じ意味 (トレイト境界)
//
// ■ 違う型をまとめて扱う: トレイトオブジェクト
//     let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Cat)];
//
// ■ 標準ライブラリのトレイトを実装すると、Rust の機能と連携できます
//     Display       → println!("{}", x) や x.to_string() が使える
//     Add           → a + b が使える (演算子オーバーロード)
//     Debug / Clone / PartialEq などは #[derive(...)] で自動実装できる
//
// ------------------------------------------------------------
// 課題: 足りない impl を追加して、テストを通してください。
// ------------------------------------------------------------

use std::fmt;
use std::ops::Add;

trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;

    // デフォルト実装
    fn introduce(&self) -> String {
        format!("{}は「{}」と鳴きます", self.name(), self.sound())
    }
}

struct Dog;

impl Animal for Dog {
    fn name(&self) -> String {
        "犬".to_string()
    }
    fn sound(&self) -> String {
        "ワン".to_string()
    }
}

struct Cat {
    name: String,
}

// TODO: Cat に Animal を実装してください。
//   name は Cat の name フィールド、sound は "ニャー"

struct Bird;

// TODO: Bird に Animal を実装してください。
//   name は "鳥"、sound は "チュン"。
//   さらに introduce を上書きして "鳥は空を飛びます" を返すようにしてください。

// TODO: すべての動物の introduce() の結果を、順番どおり Vec に集めて返す
fn introduce_all(animals: &[Box<dyn Animal>]) -> Vec<String> {
    todo!()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// TODO: Point に fmt::Display を実装して、"(1, -2)" のように表示されるようにする
//   impl fmt::Display for Point {
//       fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//           write!(f, ...)
//       }
//   }

// TODO: Point に Add を実装して、p1 + p2 で座標ごとの足し算ができるようにする
//   impl Add for Point {
//       type Output = Point;
//       fn add(self, other: Point) -> Point { ... }
//   }

// ------------------------------------------------------------
// テスト (ここは変更しないでください)
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dog() {
        assert_eq!(Dog.introduce(), "犬は「ワン」と鳴きます");
    }

    #[test]
    fn test_cat() {
        let tama = Cat { name: "タマ".to_string() };
        assert_eq!(tama.sound(), "ニャー");
        assert_eq!(tama.introduce(), "タマは「ニャー」と鳴きます");
    }

    #[test]
    fn test_bird() {
        assert_eq!(Bird.name(), "鳥");
        assert_eq!(Bird.sound(), "チュン");
        assert_eq!(Bird.introduce(), "鳥は空を飛びます");
    }

    #[test]
    fn test_introduce_all() {
        let animals: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog),
            Box::new(Cat { name: "ミケ".to_string() }),
            Box::new(Bird),
        ];
        assert_eq!(
            introduce_all(&animals),
            vec![
                "犬は「ワン」と鳴きます",
                "ミケは「ニャー」と鳴きます",
                "鳥は空を飛びます",
            ]
        );
    }

    #[test]
    fn test_display() {
        let p = Point { x: 1, y: -2 };
        assert_eq!(p.to_string(), "(1, -2)");
        assert_eq!(format!("点 {p}"), "点 (1, -2)");
    }

    #[test]
    fn test_add() {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: 10, y: -5 };
        assert_eq!(a + b, Point { x: 11, y: -3 });
    }
}
