// ============================================================
// 06: 構造体 (struct) とメソッド
// ============================================================
//
// ■ 構造体の定義と生成
//     struct User { name: String, age: u32 }
//     let u = User { name: String::from("Ferris"), age: 10 };
//     println!("{}", u.name);
//
//   変数名とフィールド名が同じなら省略できます (フィールド初期化省略記法)
//     let age = 10;
//     let u = User { name, age };
//
// ■ impl ブロックでメソッドを定義
//     impl User {
//         fn new(name: &str) -> Self { ... }   // 関連関数 (self なし) → User::new("..")
//         fn greet(&self) -> String { ... }    // メソッド (読むだけ)  → u.greet()
//         fn birthday(&mut self) { ... }       // メソッド (書き換える) → u.birthday()
//     }
//   ・Self は「この型自身」(ここでは User) の別名です。
//
// ■ タプル構造体: フィールドに名前がない構造体
//     struct Color(u8, u8, u8);
//     let c = Color(255, 0, 0);  c.0 で取り出す
//
// ■ #[derive(Debug, PartialEq)] を付けると {:?} で表示したり == で比べたりできます
//
// ------------------------------------------------------------
// 課題: todo!() を実装し、足りないものを追加して、テストを通してください。
// ------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // TODO: 関連関数 (コンストラクタ)。Rectangle::new(3, 4) のように呼ばれます。
    fn new(width: u32, height: u32) -> Self {
        todo!()
    }

    // TODO: 面積を返す
    fn area(&self) -> u32 {
        todo!()
    }

    // TODO: other が自分の中にすっぽり収まる (幅も高さも自分より小さい) なら true
    fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }

    // TODO: 幅と高さを factor 倍する。
    //       自分自身を書き換えるには、self の受け取り方を変える必要があります。
    fn scale(&self, factor: u32) {
        todo!()
    }

    // TODO: 一辺が size の正方形を作る関連関数 `square` をここに自分で定義してください。
}

// 色を表すタプル構造体 (赤, 緑, 青)
struct Color(u8, u8, u8);

impl Color {
    // TODO: "#RRGGBB" 形式 (16 進数・大文字) の文字列を返す
    //   例: Color(255, 128, 0) → "#FF8000"
    //   ヒント: format!("{:02X}", 255) は "FF"、format!("{:02X}", 9) は "09"
    fn to_hex(&self) -> String {
        todo!()
    }
}

// ------------------------------------------------------------
// テスト (ここは変更しないでください)
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_area() {
        let r = Rectangle::new(3, 4);
        assert_eq!(r, Rectangle { width: 3, height: 4 });
        assert_eq!(r.area(), 12);
    }

    #[test]
    fn test_can_hold() {
        let big = Rectangle::new(10, 8);
        let small = Rectangle::new(4, 5);
        let wide = Rectangle::new(12, 1);
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
        assert!(!big.can_hold(&wide));
    }

    #[test]
    fn test_scale() {
        let mut r = Rectangle::new(2, 5);
        r.scale(3);
        assert_eq!(r, Rectangle::new(6, 15));
    }

    #[test]
    fn test_square() {
        let sq = Rectangle::square(7);
        assert_eq!(sq.width, 7);
        assert_eq!(sq.height, 7);
        assert_eq!(sq.area(), 49);
    }

    #[test]
    fn test_to_hex() {
        assert_eq!(Color(255, 128, 0).to_hex(), "#FF8000");
        assert_eq!(Color(0, 0, 0).to_hex(), "#000000");
        assert_eq!(Color(9, 10, 171).to_hex(), "#090AAB");
    }
}
