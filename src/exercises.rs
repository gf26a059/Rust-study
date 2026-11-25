//! 練習問題の一覧とヒント。

pub struct Exercise {
    /// `exercises/` ディレクトリ内のファイル名
    pub file: &'static str,
    /// テーマ
    pub title: &'static str,
    /// `cargo run -- hint <番号>` で表示されるヒント
    pub hint: &'static str,
}

impl Exercise {
    /// 拡張子を除いた名前 (例: "01_variables")
    pub fn name(&self) -> &'static str {
        self.file.trim_end_matches(".rs")
    }

    /// 番号 (例: 1)
    pub fn number(&self) -> usize {
        self.file[..2].parse().unwrap_or(0)
    }

    /// rustc に渡すクレート名 (数字始まりは不可なので "ex" を付ける)
    pub fn crate_name(&self) -> String {
        format!("ex{}", self.name())
    }
}

pub const EXERCISES: &[Exercise] = &[
    Exercise {
        file: "01_variables.rs",
        title: "変数と可変性",
        hint: "\
・値を書き換える変数には `let mut` が必要です。
・シャドーイングは `let text = text.len();` のように同じ名前で let し直します。
・定数 (const) は型の注釈が必須です: `const NAME: 型 = 値;`",
    },
    Exercise {
        file: "02_functions.rs",
        title: "関数と基本の型",
        hint: "\
・関数の引数と戻り値の型は省略できません: `fn add(a: i32, b: i32) -> i32`
・最後の式にセミコロンを付けると「文」になり、戻り値が () になります。
・整数の割り算は小数点以下が切り捨てられます。先に `as f64` で変換してから割りましょう。
・タプルは `(min, max)` で作り、`let (a, b) = pair;` で分解できます。",
    },
    Exercise {
        file: "03_control_flow.rs",
        title: "制御フロー (if / loop / while / for)",
        hint: "\
・FizzBuzz は 15 の倍数の判定を最初に書くのがコツです。`n % 15 == 0`
・`for i in 1..=n { total += i; }` で 1〜n を回せます。
・`loop` の中で `break n;` と書くと、loop 全体の値が n になります。
・引数を書き換えたいときは `fn f(mut n: u64)` のように mut を付けられます。",
    },
    Exercise {
        file: "04_ownership.rs",
        title: "所有権とムーブ",
        hint: "\
・`count(v)` で v の所有権が関数に移ります。1 回目を `count(v.clone())` にしましょう。
・`let s2 = s1;` の後は s2 を使います。push_str するには `let mut s2` が必要です。
・引数を変更するには `fn add_four(mut v: Vec<i32>)` と書けます。
・escape_scope: outer の型は String (所有権を持つ型) です。
  &inner (借りただけの参照) ではなく、inner そのものをムーブして渡しましょう。",
    },
    Exercise {
        file: "05_borrowing.rs",
        title: "参照と借用・スライス",
        hint: "\
・`fn length(s: &str)` にして、呼び出しを `length(&s)` にしましょう。
・中身を変更する参照は `&mut String` です。
・可変参照は同時に 1 つだけ。r1 を使い終わってから r2 を作れば OK です。
・first_word は `s.split(' ').next().unwrap_or(\"\")` や、
  `match s.find(' ') { Some(i) => &s[..i], None => s }` で書けます。
・sum は `for v in values { total += v; }` で。",
    },
    Exercise {
        file: "06_structs.rs",
        title: "構造体とメソッド",
        hint: "\
・コンストラクタは `Rectangle { width, height }` (フィールド名と変数名が同じなら省略記法)。
・自分自身を変更するメソッドは `&mut self` を受け取ります。
・square は impl ブロックの中に `fn square(size: u32) -> Self { ... }` を追加します。
・16 進数 2 桁 (大文字) は `format!(\"{:02X}\", 値)` です。タプル構造体の値は self.0, self.1, self.2。",
    },
    Exercise {
        file: "07_enums.rs",
        title: "列挙型 (enum) と match・Option",
        hint: "\
・match は全パターンを網羅する必要があります。Triangle の腕を追加しましょう (面積 = 底辺 × 高さ ÷ 2)。
・範囲パターン `1..=9`、ガード `n if n < 0` が使えます。
・Option は `Some(値)` と `None`。plus_one には `None => None` が足りません。
・`if let Some(n) = name { ... } else { ... }` で 1 パターンだけ取り出せます。",
    },
    Exercise {
        file: "08_collections.rs",
        title: "コレクション (Vec / String / HashMap)",
        hint: "\
・evens: `for &v in values { if v % 2 == 0 { result.push(v); } }`
・capitalize: `match chars.next() { Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(), None => String::new() }`
・word_count: `for w in text.split_whitespace() { *counts.entry(w.to_string()).or_insert(0) += 1; }`
・score_of: `scores.get(name).copied().unwrap_or(0)`",
    },
    Exercise {
        file: "09_errors.rs",
        title: "エラー処理 (Result と ? 演算子)",
        hint: "\
・double: `match s.parse::<i32>() { Ok(n) => Ok(n * 2), Err(e) => Err(e) }`
・add_strs: `let x: i32 = a.parse()?;` のように ? を付けると、Err のとき即 return します。
・parse_age: まず `let n = s.parse::<i64>().map_err(|_| AgeError::NotANumber)?;`
  その後 if で範囲をチェックし、最後に `Ok(n as u32)`。
・Option でも ? が使えます: `let word = text.split_whitespace().next()?;` → `word.chars().next()`",
    },
    Exercise {
        file: "10_traits.rs",
        title: "トレイト",
        hint: "\
・`impl Animal for Cat { fn name(&self) -> String { self.name.clone() } ... }`
・Bird では introduce も定義すれば、デフォルト実装を上書きできます。
・introduce_all: `animals.iter().map(|a| a.introduce()).collect()` か for ループで。
・Display: `impl fmt::Display for Point { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, \"({}, {})\", self.x, self.y) } }`
・Add: `impl std::ops::Add for Point { type Output = Point; fn add(self, other: Point) -> Point { ... } }`",
    },
    Exercise {
        file: "11_generics.rs",
        title: "ジェネリクス",
        hint: "\
・largest には「比較できる」「コピーできる」という条件が必要です: `fn largest<T: PartialOrd + Copy>`
・swap: `Pair { first: self.second, second: self.first }`
・larger: `if self.first >= self.second { &self.first } else { &self.second }`
・Stack は中の Vec のメソッド (push / pop / last / len / is_empty) をそのまま使えます。",
    },
    Exercise {
        file: "12_iterators.rs",
        title: "クロージャとイテレータ",
        hint: "\
・squares: `v.iter().map(|x| x * x).collect()`
・sum_of_odds: `v.iter().filter(|x| *x % 2 != 0).sum()`
・count_long_words: `text.split_whitespace().filter(|w| w.chars().count() >= min_len).count()`
・adult_names: filter してから `map(|p| p.name.clone())`
・make_adder: n を中に閉じ込めるので `move |x| x + n`
・dot: `a.iter().zip(b).map(|(x, y)| x * y).sum()`
・count_calls: `let mut increment = || count += 1;`",
    },
    Exercise {
        file: "13_lifetimes.rs",
        title: "ライフタイム",
        hint: "\
・`fn longest<'a>(a: &'a str, b: &'a str) -> &'a str`
・`struct Highlight<'a> { text: &'a str }`
・`impl<'a> Highlight<'a> { fn first_sentence(text: &'a str) -> Highlight<'a> { ... } }`
・trim_hashes は `s.trim_start_matches('#')`。引数の参照が 1 つなら注釈は不要です。",
    },
    Exercise {
        file: "14_modules.rs",
        title: "モジュールと公開範囲 (pub)",
        hint: "\
・モジュールの外から使う構造体・関数・メソッドには `pub` を付けます。
  (構造体を pub にしても、フィールドは非公開のままにできます)
・withdraw: `if amount > self.balance { return Err(\"残高不足\".to_string()); }`
・yen: 数字の文字列を後ろから 3 桁ずつ区切ります。
  例: `let digits = amount.to_string();` を 1 文字ずつ見て、
  「残りの桁数が 3 の倍数」の位置 (先頭以外) で ',' を入れる。",
    },
    Exercise {
        file: "15_todo_app.rs",
        title: "総まとめ: ToDo アプリ",
        hint: "\
・ID でタスクを探すには `self.tasks.iter_mut().find(|t| t.id == id)` が便利です。
・削除は `position` で位置を見つけて `self.tasks.remove(i)`。
・parse_command: `let (cmd, rest) = input.trim().split_once(' ').unwrap_or((input.trim(), \"\"));`
  で 1 単語目と残りに分けて match します。
・数値への変換は `rest.trim().parse::<u32>().map_err(|_| \"...\".to_string())?`
・完成したら `cargo run -- run 15` で実際に動かせます！",
    },
];
