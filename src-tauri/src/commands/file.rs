// src-tauri/src/commands/file.rs

use crate::usecase::emplist;
use tauri_plugin_log::log::{error, info};

/// Reactから呼び出されるコマンド（窓口）
/// Reactの invoke("process_file", { path, processNumber }) と対応します。
#[tauri::command]
pub fn process_file(path: String, process_number: u16) -> Result<String, String> {
    info!("フロントエンドからのリクエスト受信: パス={}, 処理番号={}", path, process_number);

    // ユースケース（現場監督）にすべてお任せします
    match emplist::create_employee_list(&path, process_number) {
        Ok(success_msg) => {
            info!("処理成功: {}", success_msg);
            Ok(success_msg)
        }
        Err(error_msg) => {
            error!("処理失敗: {}", error_msg);
            Err(error_msg)
        }
    }
}