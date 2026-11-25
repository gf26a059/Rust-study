// ============================================================
// 14: モジュールと公開範囲 (pub)
// ============================================================
//
// ■ mod でコードをまとめる
//     mod garden {
//         pub fn plant() { ... }   // pub を付けると外から使える
//         fn secret() { ... }      // 付けなければ非公開 (モジュールの中だけ)
//     }
//     garden::plant();
//
// ■ 公開範囲のルール
//   ・何も付けなければ非公開 (private) がデフォルト
//   ・構造体を pub にしても、フィールドは個別に pub を付けない限り非公開
//     → 外からは決められたメソッド経由でしか触れない (カプセル化)
//   ・子モジュールは親の非公開アイテムを使える (super:: で親を指す)
//
// ■ use で名前を短くする
//     use garden::plant;   →  plant();
//
// ■ 実際のプロジェクトではファイルに分ける
//     src/main.rs に `mod bank;` と書くと src/bank.rs が読み込まれます。
//
// ------------------------------------------------------------
// 課題: 必要なところに pub を付け、todo!() を実装してください。
//   ※ テストは bank モジュールの「外」から使っています。
//     フィールドは pub にせず、メソッド経由で使うのがポイントです。
// ------------------------------------------------------------

mod bank {
    // TODO: 外から使えるようにしてください (フィールドは非公開のままで OK)
    struct Account {
        owner: String,
        balance: u64,
    }

    impl Account {
        // TODO: 外から呼べるようにしてください
        fn new(owner: &str) -> Self {
            Account {
                owner: owner.to_string(),
                balance: 0,
            }
        }

        pub fn owner(&self) -> &str {
            &self.owner
        }

        pub fn balance(&self) -> u64 {
            self.balance
        }

        pub fn deposit(&mut self, amount: u64) {
            self.balance += amount;
        }

        // TODO: 残高が足りなければ Err("残高不足") を返し、残高は変えない。
        //       足りれば残高から引いて Ok(())
        pub fn withdraw(&mut self, amount: u64) -> Result<(), String> {
            todo!()
        }
    }

    // TODO: 外から使えるようにしてください
    mod format {
        // TODO: 金額を 3 桁ごとにカンマで区切って "円" を付ける
        //   例: 0 → "0円"、1234 → "1,234円"、1000000 → "1,000,000円"
        pub fn yen(amount: u64) -> String {
            todo!()
        }
    }
}

use bank::Account;

// ------------------------------------------------------------
// テスト (ここは変更しないでください)
// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::bank::format::yen;
    use super::*;

    #[test]
    fn test_account() {
        let mut acc = Account::new("Ferris");
        assert_eq!(acc.owner(), "Ferris");
        assert_eq!(acc.balance(), 0);

        acc.deposit(1000);
        assert_eq!(acc.withdraw(300), Ok(()));
        assert_eq!(acc.balance(), 700);

        assert_eq!(acc.withdraw(701), Err("残高不足".to_string()));
        assert_eq!(acc.balance(), 700);
    }

    #[test]
    fn test_yen() {
        assert_eq!(yen(0), "0円");
        assert_eq!(yen(999), "999円");
        assert_eq!(yen(1234), "1,234円");
        assert_eq!(yen(1000000), "1,000,000円");
        assert_eq!(yen(12345678), "12,345,678円");
    }
}
