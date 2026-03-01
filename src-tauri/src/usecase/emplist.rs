// src-tauri/src/usecase/emplist.rs

use crate::domain::models::ProcessNumber;
use crate::infrastructure::csv_reader;
use tauri_plugin_log::log::info;

/// 従業員リスト作成のメインシナリオ（現場監督）
pub fn create_employee_list(input_path: &str, process_number: u16) -> Result<String, String> {
    info!("ユースケース開始: 従業員リスト作成 (パス: {}, 番号: {})", input_path, process_number);

    // 1. ドメイン（ルールブック）を使って、処理番号が正しいかチェックします
    let proc_num = ProcessNumber::new(process_number)?;

    // 2. インフラ（作業員）に、パスと処理番号を渡して実際のファイル処理を指示します
    csv_reader::distribute_csv_data(input_path, proc_num.value())?;

    // 3. すべて無事に終わったら、成功メッセージを返します
    let success_msg = format!("処理番号【{}】でリストの作成が完了しました！✨", proc_num.value());
    Ok(success_msg)
}