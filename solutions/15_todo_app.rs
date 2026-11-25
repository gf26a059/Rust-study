// 【解答例】これは一つの書き方です。他の書き方でもテストが通れば正解です！
// ============================================================
// 15: 総まとめ — ToDo アプリを作ろう
// ============================================================
//
// これまで学んだ知識を総動員して、小さな ToDo 管理アプリを作ります。
//   構造体 / enum / match / Vec / String / Result / ? / イテレータ / 借用
//
// テストが全部通ったら、実際に動かしてみましょう！
//
//     cargo run -- run 15
//
// 動かせたら、自分で機能を追加してみるのもおすすめです。
//   例: タイトルの変更 (edit 1 新しいタイトル)、完了済みの一括削除 (clear) など
//
// ------------------------------------------------------------
// 課題: todo!() を実装して、テストを通してください。
// ------------------------------------------------------------

use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, PartialEq)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Debug, PartialEq)]
enum Command {
    Add(String),
    Done(u32),
    Remove(u32),
    List,
    Quit,
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TodoList {
    // TODO: 空のリストを作る。ID は 1 から振る。
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // TODO: タスクを追加して、振った ID を返す (ID は追加するたびに 1 ずつ増える)
    fn add(&mut self, title: &str) -> u32 {
        let id = self.next_id;
        self.tasks.push(Task {
            id,
            title: title.to_string(),
            done: false,
        });
        self.next_id += 1;
        id
    }

    // TODO: 指定 ID のタスクを完了にする。
    //       見つからなければ Err(format!("ID {id} のタスクはありません"))
    fn complete(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.done = true;
                Ok(())
            }
            None => Err(format!("ID {id} のタスクはありません")),
        }
    }

    // TODO: 指定 ID のタスクを削除して、削除したタスクを返す。
    //       見つからなければ Err(format!("ID {id} のタスクはありません"))
    fn remove(&mut self, id: u32) -> Result<Task, String> {
        let index = self
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or(format!("ID {id} のタスクはありません"))?;
        Ok(self.tasks.remove(index))
    }

    // TODO: 未完了のタスクだけを返す
    fn pending(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| !t.done).collect()
    }

    // TODO: 一覧を文字列にする。1 行に 1 タスク、改行 (\n) でつなぐ。
    //   未完了: "[ ] 1: 牛乳を買う"
    //   完了  : "[x] 2: 掃除する"
    //   タスクが 1 つもなければ "タスクはありません"
    fn render(&self) -> String {
        if self.tasks.is_empty() {
            return "タスクはありません".to_string();
        }
        self.tasks
            .iter()
            .map(|t| format!("[{}] {}: {}", if t.done { 'x' } else { ' ' }, t.id, t.title))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// TODO: 入力された 1 行をコマンドに変換する
//   "add 牛乳を買う" → Ok(Command::Add("牛乳を買う".to_string()))
//   "done 1"        → Ok(Command::Done(1))
//   "remove 2"      → Ok(Command::Remove(2))
//   "list"          → Ok(Command::List)
//   "quit"          → Ok(Command::Quit)
//   それ以外 (タイトルが空、ID が数字でない、知らないコマンド) → Err(説明メッセージ)
//   ※ 前後の空白は無視する
fn parse_command(input: &str) -> Result<Command, String> {
    let input = input.trim();
    let (cmd, rest) = input.split_once(' ').unwrap_or((input, ""));
    let rest = rest.trim();

    let parse_id = |s: &str| -> Result<u32, String> {
        s.parse::<u32>()
            .map_err(|_| format!("ID は数字で指定してください: `{s}`"))
    };

    match cmd {
        "add" if rest.is_empty() => Err("タイトルを入力してください".to_string()),
        "add" => Ok(Command::Add(rest.to_string())),
        "done" => Ok(Command::Done(parse_id(rest)?)),
        "remove" => Ok(Command::Remove(parse_id(rest)?)),
        "list" => Ok(Command::List),
        "quit" => Ok(Command::Quit),
        "" => Err("コマンドを入力してください".to_string()),
        other => Err(format!("不明なコマンドです: `{other}`")),
    }
}

// コマンドを実行して、表示するメッセージを返す (ここは完成済み)
fn execute(list: &mut TodoList, cmd: Command) -> String {
    match cmd {
        Command::Add(title) => {
            let id = list.add(&title);
            format!("追加しました (ID: {id})")
        }
        Command::Done(id) => match list.complete(id) {
            Ok(()) => format!("完了にしました (ID: {id})"),
            Err(e) => e,
        },
        Command::Remove(id) => match list.remove(id) {
            Ok(task) => format!("削除しました: {}", task.title),
            Err(e) => e,
        },
        Command::List => list.render(),
        Command::Quit => "またね！".to_string(),
    }
}

// `cargo run -- run 15` で実行される main 関数 (ここは完成済み)
fn main() {
    println!("📝 ToDo アプリ  (add <タイトル> / done <ID> / remove <ID> / list / quit)");
    let mut list = TodoList::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap() == 0 {
            break; // 入力の終わり (Ctrl+D)
        }
        if line.trim().is_empty() {
            continue;
        }
        match parse_command(&line) {
            Ok(Command::Quit) => {
                println!("{}", execute(&mut list, Command::Quit));
                break;
            }
            Ok(cmd) => println!("{}", execute(&mut list, cmd)),
            Err(e) => println!("⚠ {e}"),
        }
    }
}

// ------------------------------------------------------------
// テスト (ここは変更しないでください)
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_assigns_ids() {
        let mut list = TodoList::new();
        assert_eq!(list.add("牛乳を買う"), 1);
        assert_eq!(list.add("掃除する"), 2);
        assert_eq!(list.tasks.len(), 2);
        assert_eq!(list.tasks[1].title, "掃除する");
        assert!(!list.tasks[0].done);
    }

    #[test]
    fn test_complete() {
        let mut list = TodoList::new();
        list.add("a");
        list.add("b");
        assert_eq!(list.complete(2), Ok(()));
        assert!(list.tasks[1].done);
        assert_eq!(list.complete(99), Err("ID 99 のタスクはありません".to_string()));
    }

    #[test]
    fn test_remove_keeps_ids() {
        let mut list = TodoList::new();
        list.add("a");
        list.add("b");
        let removed = list.remove(1).unwrap();
        assert_eq!(removed.title, "a");
        assert!(list.remove(1).is_err());
        // 削除しても ID は使い回さない
        assert_eq!(list.add("c"), 3);
    }

    #[test]
    fn test_pending() {
        let mut list = TodoList::new();
        list.add("a");
        list.add("b");
        list.add("c");
        list.complete(2).unwrap();
        let titles: Vec<&str> = list.pending().iter().map(|t| t.title.as_str()).collect();
        assert_eq!(titles, vec!["a", "c"]);
    }

    #[test]
    fn test_render() {
        let mut list = TodoList::new();
        assert_eq!(list.render(), "タスクはありません");
        list.add("牛乳を買う");
        list.add("掃除する");
        list.complete(2).unwrap();
        assert_eq!(list.render(), "[ ] 1: 牛乳を買う\n[x] 2: 掃除する");
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("add 牛乳を買う"), Ok(Command::Add("牛乳を買う".to_string())));
        assert_eq!(parse_command("  add  本を読む 30 分 \n"), Ok(Command::Add("本を読む 30 分".to_string())));
        assert_eq!(parse_command("done 3"), Ok(Command::Done(3)));
        assert_eq!(parse_command("remove 10"), Ok(Command::Remove(10)));
        assert_eq!(parse_command("list"), Ok(Command::List));
        assert_eq!(parse_command("quit\n"), Ok(Command::Quit));
    }

    #[test]
    fn test_parse_command_errors() {
        assert!(parse_command("add").is_err());
        assert!(parse_command("add    ").is_err());
        assert!(parse_command("done abc").is_err());
        assert!(parse_command("remove").is_err());
        assert!(parse_command("jump").is_err());
        assert!(parse_command("").is_err());
    }

    #[test]
    fn test_execute_flow() {
        let mut list = TodoList::new();
        assert_eq!(execute(&mut list, Command::Add("散歩".to_string())), "追加しました (ID: 1)");
        assert_eq!(execute(&mut list, Command::Done(1)), "完了にしました (ID: 1)");
        assert_eq!(execute(&mut list, Command::List), "[x] 1: 散歩");
        assert_eq!(execute(&mut list, Command::Remove(1)), "削除しました: 散歩");
        assert_eq!(execute(&mut list, Command::Done(1)), "ID 1 のタスクはありません");
    }
}
