// src-tauri/src/domain/models.rs

/// 処理番号を表す専用のデータ型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessNumber(u16);

impl ProcessNumber {
    pub fn new(value: u16) -> Result<Self, String> {
        if value == 0 {
            return Err("処理番号に0は設定できません。".to_string());
        }
        if value > 9999 {
            return Err("処理番号は最大4桁（9999以下）である必要があります。".to_string());
        }
        Ok(ProcessNumber(value))
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

/// 従業員のデータ構造（設計図）
#[derive(Debug, Clone)]
pub struct Employee {
    pub id: u16,
    pub uid: String,
    pub login_kbn: String,
    pub first_name: String,
    pub last_name: String,
    pub name: String,
    pub password: String,
    pub group_name: String,
    pub company_code: u16,
}

impl Employee {
    pub fn new(
        id: u16,
        uid: String,
        login_kbn: String,
        first_name: String,
        last_name: String,
        name: String,
        password: String,
        group_name: String,
        company_code: u16,
    ) -> Result<Self, String> {
        // --- ここでドメインのルール（バリデーション）をチェックします ---
        
        // ルール1: id が0はダメ
        if id == 0 {
            return Err("エラー: IDに0は設定できません。".to_string());
        }

        match (first_name.trim(), last_name.trim(), group_name.trim(), company_code) {
            ("", _, _, _) => return Err("エラー: 名は空にできません。".to_string()),
            (_, "", _, _) => return Err("エラー: 姓は空にできません。".to_string()),
            (_, _, "", _) => return Err("エラー: グループ名は空にできません。".to_string()),
            (_, _, _, 0) => return Err("エラー: 会社コードに0は設定できません。".to_string()),
            _ => {}
        }

        // ルールをすべてクリアしたら、データを組み立てて返します
        Ok(Employee {
            id,
            uid,
            login_kbn,
            first_name,
            last_name,
            name,
            password,
            group_name,
            company_code,
        })
    }
}