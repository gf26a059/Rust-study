// ============================================================
// 11: ジェネリクス
// ============================================================
//
// ■ ジェネリクス = 型を「引数」にして、いろいろな型で使い回せるコードを書く仕組み
//     fn first<T>(list: &[T]) -> &T { &list[0] }   // i32 でも String でも OK
//     struct Wrapper<T> { value: T }
//     Option<T> や Vec<T> もジェネリクスです。
//
// ■ トレイト境界: 「T はこういうことができる型に限る」という条件
//     fn show<T: std::fmt::Display>(x: T) { println!("{x}") }
//     fn f<T: PartialOrd + Copy>(...)          // 複数の条件は + でつなぐ
//     fn f<T>(...) where T: PartialOrd + Copy  // where で書いてもよい
//
//   よく使う境界:
//     PartialOrd … < > で比べられる
//     PartialEq  … == で比べられる
//     Copy       … 代入してもムーブされない (コピーされる)
//     Clone      … .clone() できる
//
// ■ 条件付き実装
//     impl<T: PartialOrd> Pair<T> { ... }  // T が比較できるときだけ使えるメソッド
//
// ------------------------------------------------------------
// 課題: コンパイルエラーを直し、todo!() を実装してください。
// ------------------------------------------------------------

// TODO: どんな型のスライスでも最大値を返す関数。
//       このままだと「T 同士を > で比べられるか分からない」
//       「list[0] をコピーして取り出せるか分からない」ためエラーになります。
//       T にトレイト境界を付けてください。
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    // TODO: first と second を入れ替えた新しい Pair を返す (self を消費する)
    fn swap(self) -> Pair<T> {
        todo!()
    }
}

impl<T: PartialOrd> Pair<T> {
    // TODO: 大きいほうへの参照を返す (同じなら first)
    fn larger(&self) -> &T {
        todo!()
    }
}

// TODO: ジェネリックなスタック (後入れ先出し / LIFO) を完成させる
//   中身は Vec<T> なので、Vec のメソッドをうまく使いましょう。
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        todo!()
    }

    fn push(&mut self, item: T) {
        todo!()
    }

    // 一番上の要素を取り出す。空なら None
    fn pop(&mut self) -> Option<T> {
        todo!()
    }

    // 一番上の要素を「見るだけ」。空なら None
    fn peek(&self) -> Option<&T> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn is_empty(&self) -> bool {
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
    fn test_largest() {
        assert_eq!(largest(&[34, 50, 25, 100, 65]), 100);
        assert_eq!(largest(&['y', 'm', 'a', 'q']), 'y');
        assert_eq!(largest(&[1.5, -2.0, 0.3]), 1.5);
    }

    #[test]
    fn test_pair() {
        let p = Pair::new(1, 2).swap();
        assert_eq!((p.first, p.second), (2, 1));

        let words = Pair::new(String::from("apple"), String::from("banana"));
        assert_eq!(words.larger(), "banana");
        assert_eq!(Pair::new(10, 3).larger(), &10);
        assert_eq!(Pair::new(7, 7).larger(), &7);
    }

    #[test]
    fn test_stack() {
        let mut s = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.pop(), None);

        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.len(), 3);
        assert_eq!(s.peek(), Some(&3));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.len(), 1);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_stack_of_strings() {
        let mut s: Stack<String> = Stack::new();
        s.push("hello".to_string());
        s.push("world".to_string());
        assert_eq!(s.peek().map(|x| x.as_str()), Some("world"));
        assert_eq!(s.pop(), Some("world".to_string()));
    }
}
