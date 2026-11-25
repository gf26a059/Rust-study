// ============================================================
// 遊び場 (playground)
// ============================================================
//
// ここは自由に書き換えて実験するためのファイルです。
//
//     cargo run --bin playground
//
// で実行できます。「これってどうなるんだろう？」と思ったら、
// ここに書いて試してみましょう。コンパイルエラーも大事な先生です。

fn main() {
    println!("Hello, Rust! 🦀");

    // 例: いろいろな値を表示してみる
    let name = "Ferris";
    let age = 10;
    println!("{name} は {age} 歳です");

    // {:?} を使うとデバッグ表示 (Vec やタプルもそのまま表示できる)
    let numbers = vec![1, 2, 3];
    println!("numbers = {numbers:?}");

    // dbg! はファイル名・行番号・式・値をまとめて表示してくれる
    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    dbg!(&doubled);
}
