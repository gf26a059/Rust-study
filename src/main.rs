//! Rust Study — 練習問題チェッカー
//!
//! `exercises/` 以下の問題をコンパイルしてテストを実行し、進捗を表示します。
//! 使い方は `cargo run -- help` を見てください。

mod exercises;

use exercises::{EXERCISES, Exercise};
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{self, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

/// テストがこれ以上かかったら無限ループとみなして止める
const TEST_TIMEOUT: Duration = Duration::from_secs(10);
const EDITION: &str = "2024";
const LINE: &str = "────────────────────────────────────────────────────────";

// ------------------------------------------------------------
// チェック結果
// ------------------------------------------------------------

enum Outcome {
    Passed,
    CompileError(String),
    TestFailed(String),
    TimedOut,
}

impl Outcome {
    fn passed(&self) -> bool {
        matches!(self, Outcome::Passed)
    }
}

// ------------------------------------------------------------
// 実行環境 (パスや色)
// ------------------------------------------------------------

struct Ctx {
    root: PathBuf,
    color: bool,
}

impl Ctx {
    fn new() -> Self {
        Ctx {
            root: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            color: io::stdout().is_terminal() && env::var_os("NO_COLOR").is_none(),
        }
    }

    fn paint(&self, code: &str, s: &str) -> String {
        if self.color {
            format!("\x1b[{code}m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
    fn bold(&self, s: &str) -> String {
        self.paint("1", s)
    }
    fn dim(&self, s: &str) -> String {
        self.paint("2", s)
    }
    fn green(&self, s: &str) -> String {
        self.paint("32", s)
    }
    fn red(&self, s: &str) -> String {
        self.paint("31", s)
    }
    fn yellow(&self, s: &str) -> String {
        self.paint("33", s)
    }
    fn cyan(&self, s: &str) -> String {
        self.paint("36", s)
    }

    fn color_flag(&self) -> &'static str {
        if self.color { "always" } else { "never" }
    }

    fn source(&self, dir: &str, ex: &Exercise) -> PathBuf {
        self.root.join(dir).join(ex.file)
    }

    /// エラー表示用の相対パス (例: exercises/01_variables.rs)
    fn relative(&self, dir: &str, ex: &Exercise) -> PathBuf {
        Path::new(dir).join(ex.file)
    }

    fn build_dir(&self, dir: &str) -> io::Result<PathBuf> {
        let path = self.root.join("target").join("study").join(dir);
        fs::create_dir_all(&path)?;
        Ok(path)
    }
}

// ------------------------------------------------------------
// コンパイルとテスト
// ------------------------------------------------------------

fn compile(ctx: &Ctx, src: &Path, crate_name: &str, out: &Path, test: bool) -> io::Result<Option<String>> {
    let mut cmd = Command::new("rustc");
    // プロジェクトのルートで実行し、エラー表示のパスを短くする
    cmd.current_dir(&ctx.root)
        .args(["--edition", EDITION, "-A", "unused", "--color", ctx.color_flag()])
        .args(["--crate-name", crate_name, "-o"])
        .arg(out)
        .arg(src);
    if test {
        cmd.arg("--test");
    }
    let output = cmd.output().map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("rustc を起動できませんでした ({e})。Rust はインストールされていますか？"),
        )
    })?;
    if output.status.success() {
        Ok(None)
    } else {
        Ok(Some(String::from_utf8_lossy(&output.stderr).into_owned()))
    }
}

fn check(ctx: &Ctx, dir: &str, ex: &Exercise) -> io::Result<Outcome> {
    let build = ctx.build_dir(dir)?;
    let crate_name = ex.crate_name();
    let bin = build.join(format!("{crate_name}{}", env::consts::EXE_SUFFIX));

    if let Some(err) = compile(ctx, &ctx.relative(dir, ex), &crate_name, &bin, true)? {
        return Ok(Outcome::CompileError(err));
    }

    // 出力はファイルに書き出す (パイプが詰まらないように)
    let log = build.join(format!("{crate_name}.log"));
    let file = File::create(&log)?;
    let mut child = Command::new(&bin)
        .args(["--color", ctx.color_flag(), "--test-threads", "1"])
        .env("RUST_BACKTRACE", "0")
        .stdout(Stdio::from(file.try_clone()?))
        .stderr(Stdio::from(file))
        .spawn()?;

    let start = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }
        if start.elapsed() > TEST_TIMEOUT {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        thread::sleep(Duration::from_millis(20));
    };

    let output = fs::read_to_string(&log).unwrap_or_default();
    Ok(match status {
        None => Outcome::TimedOut,
        Some(s) if s.success() => Outcome::Passed,
        Some(_) => Outcome::TestFailed(output),
    })
}

// ------------------------------------------------------------
// 合格済みキャッシュ (変更のないファイルは再コンパイルしない)
// ------------------------------------------------------------

struct Cache {
    path: PathBuf,
    passed: HashMap<String, u64>,
}

impl Cache {
    fn load(ctx: &Ctx, dir: &str) -> Cache {
        let path = ctx.root.join("target").join("study").join(format!("{dir}-passed.txt"));
        let passed = fs::read_to_string(&path)
            .unwrap_or_default()
            .lines()
            .filter_map(|line| {
                let (name, hash) = line.split_once('\t')?;
                Some((name.to_string(), hash.parse().ok()?))
            })
            .collect();
        Cache { path, passed }
    }

    fn save(&self) {
        let mut lines: Vec<String> = self.passed.iter().map(|(k, v)| format!("{k}\t{v}")).collect();
        lines.sort();
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&self.path, lines.join("\n"));
    }

    fn is_passed(&self, ex: &Exercise, hash: Option<u64>) -> bool {
        hash.is_some() && self.passed.get(ex.name()).copied() == hash
    }
}

fn file_hash(path: &Path) -> Option<u64> {
    let content = fs::read(path).ok()?;
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    EDITION.hash(&mut hasher);
    Some(hasher.finish())
}

fn check_cached(ctx: &Ctx, cache: &mut Cache, dir: &str, ex: &Exercise) -> io::Result<Outcome> {
    let hash = file_hash(&ctx.source(dir, ex));
    if cache.is_passed(ex, hash) {
        return Ok(Outcome::Passed);
    }
    let outcome = check(ctx, dir, ex)?;
    match (outcome.passed(), hash) {
        (true, Some(h)) => {
            cache.passed.insert(ex.name().to_string(), h);
        }
        _ => {
            cache.passed.remove(ex.name());
        }
    }
    cache.save();
    Ok(outcome)
}

// ------------------------------------------------------------
// 表示
// ------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Passed,
    Failed,
    Unknown,
}

fn print_progress(ctx: &Ctx, statuses: &[Status]) {
    let done = statuses.iter().filter(|s| **s == Status::Passed).count();
    let total = statuses.len();
    let bar: String = statuses
        .iter()
        .map(|s| {
            if *s == Status::Passed {
                ctx.green("■")
            } else {
                ctx.dim("□")
            }
        })
        .collect();
    println!();
    println!(
        "  {}  {}  {}",
        ctx.bold("🦀 Rust Study"),
        bar,
        ctx.bold(&format!("{done}/{total}"))
    );
    println!();

    let current = statuses.iter().position(|s| *s != Status::Passed);
    for (i, (ex, status)) in EXERCISES.iter().zip(statuses).enumerate() {
        let mark = match status {
            Status::Passed => "✅",
            Status::Failed => "❌",
            Status::Unknown => "・",
        };
        let name = format!("{:<18}", ex.name());
        let here = if Some(i) == current {
            ctx.yellow("  ← いまここ")
        } else {
            String::new()
        };
        let line = format!("  {mark} {name} {}{here}", ex.title);
        if *status == Status::Unknown {
            println!("{}", ctx.dim(&line))
        } else {
            println!("{line}")
        }
    }
    println!();
}

fn print_failure(ctx: &Ctx, dir: &str, ex: &Exercise, outcome: &Outcome) {
    let (label, detail) = match outcome {
        Outcome::Passed => return,
        Outcome::CompileError(out) => ("コンパイルエラー", out.as_str()),
        Outcome::TestFailed(out) => ("テスト失敗", out.as_str()),
        Outcome::TimedOut => (
            "タイムアウト",
            "テストが 10 秒以内に終わりませんでした。無限ループになっていませんか？\n",
        ),
    };
    println!("{}", ctx.dim(LINE));
    println!("{} {}  {}", ctx.red("❌"), ctx.bold(ex.name()), ctx.red(label));
    println!("{}", ctx.dim(LINE));
    print!("{detail}");
    println!("{}", ctx.dim(LINE));
    println!("  📝 編集するファイル : {}", ctx.cyan(&format!("{dir}/{}", ex.file)));
    println!(
        "  💡 行き詰まったら   : {}",
        ctx.cyan(&format!("cargo run -- hint {}", ex.number()))
    );
    println!();
}

fn print_all_done(ctx: &Ctx) {
    println!("{}", ctx.green(&ctx.bold("  🎉 全問クリア！おめでとうございます！")));
    println!();
    println!("  次のステップのアイデア:");
    println!("   ・`cargo run -- run 15` で ToDo アプリを動かして、機能を追加してみる");
    println!("   ・src/bin/playground.rs で自由にコードを書いてみる");
    println!("   ・The Rust Book (日本語版) を読む: https://doc.rust-jp.rs/book-ja/");
    println!();
}

// ------------------------------------------------------------
// サブコマンド
// ------------------------------------------------------------

/// 先頭から順にチェックし、最初に失敗した問題の詳細を表示する
fn cmd_progress(ctx: &Ctx) -> io::Result<()> {
    let mut cache = Cache::load(ctx, "exercises");
    let mut statuses = vec![Status::Unknown; EXERCISES.len()];
    let mut failure = None;

    for (i, ex) in EXERCISES.iter().enumerate() {
        if io::stdout().is_terminal() {
            print!("\r  チェック中... {:<20}", ex.name());
            io::stdout().flush()?;
        }
        let outcome = check_cached(ctx, &mut cache, "exercises", ex)?;
        if outcome.passed() {
            statuses[i] = Status::Passed;
        } else {
            statuses[i] = Status::Failed;
            failure = Some((ex, outcome));
            break;
        }
    }
    if io::stdout().is_terminal() {
        print!("\r{:<40}\r", "");
    }

    print_progress(ctx, &statuses);
    match failure {
        Some((ex, outcome)) => print_failure(ctx, "exercises", ex, &outcome),
        None => print_all_done(ctx),
    }
    Ok(())
}

/// 指定した 1 問だけチェックする
fn cmd_check(ctx: &Ctx, ex: &Exercise) -> io::Result<()> {
    let mut cache = Cache::load(ctx, "exercises");
    let outcome = check_cached(ctx, &mut cache, "exercises", ex)?;
    println!();
    if outcome.passed() {
        println!("  ✅ {}  {}", ctx.bold(ex.name()), ctx.green("合格！"));
        println!("     次へ進むには: {}", ctx.cyan("cargo run"));
        println!();
    } else {
        print_failure(ctx, "exercises", ex, &outcome);
    }
    Ok(())
}

/// 全問チェックして一覧を表示する (途中で止まらない)
fn cmd_all(ctx: &Ctx) -> io::Result<()> {
    let mut cache = Cache::load(ctx, "exercises");
    let mut statuses = Vec::new();
    for ex in EXERCISES {
        let outcome = check_cached(ctx, &mut cache, "exercises", ex)?;
        statuses.push(if outcome.passed() {
            Status::Passed
        } else {
            Status::Failed
        });
    }
    print_progress(ctx, &statuses);
    println!("  詳細を見るには: {}", ctx.cyan("cargo run -- check <番号>"));
    println!();
    Ok(())
}

/// コンパイルせずに、キャッシュから分かる範囲で一覧を表示する
fn cmd_list(ctx: &Ctx) {
    let cache = Cache::load(ctx, "exercises");
    let statuses: Vec<Status> = EXERCISES
        .iter()
        .map(|ex| {
            if cache.is_passed(ex, file_hash(&ctx.source("exercises", ex))) {
                Status::Passed
            } else {
                Status::Unknown
            }
        })
        .collect();
    print_progress(ctx, &statuses);
}

fn cmd_hint(ctx: &Ctx, ex: &Exercise) {
    println!();
    println!("  💡 {} {}", ctx.bold(ex.name()), ex.title);
    println!("{}", ctx.dim(LINE));
    for line in ex.hint.lines() {
        println!("  {line}");
    }
    println!("{}", ctx.dim(LINE));
    println!(
        "  それでも分からなければ解答例: {}",
        ctx.cyan(&format!("solutions/{}", ex.file))
    );
    println!();
}

/// main 関数を持つ問題を普通のプログラムとして実行する
fn cmd_run(ctx: &Ctx, ex: &Exercise) -> io::Result<()> {
    let src = ctx.source("exercises", ex);
    let code = fs::read_to_string(&src)?;
    if !code.contains("fn main") {
        println!();
        println!("  {} には main 関数がありません。", ex.file);
        println!("  ファイルの末尾に `fn main() {{ ... }}` を書けば、println! で好きなものを表示できます。");
        println!();
        return Ok(());
    }
    let build = ctx.build_dir("run")?;
    let crate_name = ex.crate_name();
    let bin = build.join(format!("{crate_name}{}", env::consts::EXE_SUFFIX));
    if let Some(err) = compile(ctx, &ctx.relative("exercises", ex), &crate_name, &bin, false)? {
        print!("{err}");
        return Ok(());
    }
    Command::new(&bin).status()?;
    Ok(())
}

/// ファイルの変更を監視して、保存のたびに自動でチェックする
fn cmd_watch(ctx: &Ctx) -> io::Result<()> {
    let snapshot = || -> Vec<Option<SystemTime>> {
        EXERCISES
            .iter()
            .map(|ex| {
                fs::metadata(ctx.source("exercises", ex))
                    .and_then(|m| m.modified())
                    .ok()
            })
            .collect()
    };
    let mut last = None;
    loop {
        let now = snapshot();
        if last.as_ref() != Some(&now) {
            print!("\x1b[2J\x1b[H"); // 画面クリア
            cmd_progress(ctx)?;
            println!(
                "  {}",
                ctx.dim("👀 監視中… ファイルを保存すると自動で再チェックします (Ctrl+C で終了)")
            );
            last = Some(now);
        }
        thread::sleep(Duration::from_millis(500));
    }
}

/// solutions/ の解答例がすべて合格するか確認する (メンテナンス用)
fn cmd_verify(ctx: &Ctx) -> io::Result<bool> {
    let mut ok = true;
    for ex in EXERCISES {
        let outcome = check(ctx, "solutions", ex)?;
        if outcome.passed() {
            println!("  ✅ solutions/{}", ex.file);
        } else {
            ok = false;
            print_failure(ctx, "solutions", ex, &outcome);
        }
    }
    Ok(ok)
}

fn cmd_help(ctx: &Ctx) {
    let c = |s: &str| ctx.cyan(s);
    println!();
    println!("  {}", ctx.bold("🦀 Rust Study — 手を動かして学ぶ Rust 練習問題"));
    println!();
    println!("  {}             次に解く問題をチェック (進捗を表示)", c("cargo run"));
    println!(
        "  {}       保存するたびに自動チェック (おすすめ)",
        c("cargo run -- watch")
    );
    println!(
        "  {}  ヒントを表示  (例: cargo run -- hint 3)",
        c("cargo run -- hint <番号>")
    );
    println!("  {} 1 問だけチェック", c("cargo run -- check <番号>"));
    println!(
        "  {}   main 関数を実行 (例: cargo run -- run 15)",
        c("cargo run -- run <番号>")
    );
    println!("  {}        全問をチェックして一覧表示", c("cargo run -- all"));
    println!("  {}        問題の一覧", c("cargo run -- list"));
    println!();
    println!("  問題ファイルは exercises/ にあります。最初から解き直したいときは:");
    println!("    {}", c("git checkout -- exercises/01_variables.rs"));
    println!();
}

fn find_exercise(arg: Option<&String>) -> Result<&'static Exercise, String> {
    let Some(arg) = arg else {
        return Err("問題の番号を指定してください (例: 3 または 03_control_flow)".to_string());
    };
    let key = arg.trim_end_matches(".rs");
    let key = key.rsplit('/').next().unwrap_or(key);
    EXERCISES
        .iter()
        .find(|ex| {
            key.parse::<usize>().ok() == Some(ex.number()) || ex.name() == key || ex.name().get(3..) == Some(key)
        })
        .ok_or_else(|| format!("問題 `{arg}` が見つかりません。`cargo run -- list` で一覧を確認できます"))
}

fn main() -> ExitCode {
    let ctx = Ctx::new();
    let args: Vec<String> = env::args().skip(1).collect();
    let sub = args.first().map(String::as_str).unwrap_or("check");
    let target = args.get(1);

    let result: Result<bool, String> = match sub {
        "check" if target.is_none() => cmd_progress(&ctx).map(|_| true).map_err(|e| e.to_string()),
        "check" => find_exercise(target).and_then(|ex| cmd_check(&ctx, ex).map(|_| true).map_err(|e| e.to_string())),
        "hint" => find_exercise(target).map(|ex| {
            cmd_hint(&ctx, ex);
            true
        }),
        "run" => find_exercise(target).and_then(|ex| cmd_run(&ctx, ex).map(|_| true).map_err(|e| e.to_string())),
        "all" => cmd_all(&ctx).map(|_| true).map_err(|e| e.to_string()),
        "list" => {
            cmd_list(&ctx);
            Ok(true)
        }
        "watch" => cmd_watch(&ctx).map(|_| true).map_err(|e| e.to_string()),
        "verify" => cmd_verify(&ctx).map_err(|e| e.to_string()),
        "help" | "-h" | "--help" => {
            cmd_help(&ctx);
            Ok(true)
        }
        other => Err(format!(
            "不明なコマンド `{other}` です。`cargo run -- help` を見てください"
        )),
    };

    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(msg) => {
            eprintln!();
            eprintln!("  {} {msg}", ctx.red("エラー:"));
            eprintln!();
            ExitCode::FAILURE
        }
    }
}
