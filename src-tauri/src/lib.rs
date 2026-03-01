// src-tauri/src/lib.rs

// 1. 作成した各レイヤー（フォルダ）をモジュールとして読み込みます（目次）
pub mod commands;
pub mod domain;
pub mod infrastructure;
pub mod usecase;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // ログプラグインの初期化
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        // ダイアログプラグインの初期化
        .plugin(tauri_plugin_dialog::init())
        
        // 2. コマンドの登録先を `commands::file` の中に変更します
        .invoke_handler(tauri::generate_handler![
            commands::file::process_file
        ])
        
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}