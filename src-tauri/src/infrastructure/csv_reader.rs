// src-tauri/src/infrastructure/csv_reader.rs

use csv::ReaderBuilder;
use encoding_rs::SHIFT_JIS;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::path::Path;
use tauri_plugin_log::log::{error, info};

/// SJISファイルを読み込んでUTF-8の文字列に変換する処理
fn decode_sjis_reader(file: File) -> std::io::Cursor<Vec<u8>> {
    let mut reader = BufReader::new(file);
    let mut bytes = Vec::new();
    let _ = reader.read_to_end(&mut bytes);
    let (res, _, _) = SHIFT_JIS.decode(&bytes);
    std::io::Cursor::new(res.into_owned().into_bytes())
}

/// UTF-8の文字列をSJISのバイト列に変換する処理
fn encode_to_sjis(text: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let (bytes, _, has_error) = SHIFT_JIS.encode(text);
    if has_error {
        return Err("SJIS変換エラー".into());
    }
    Ok(bytes.into_owned())
}

/// CSVファイルを読み込み、指定のルールで分割・追記する処理
pub fn distribute_csv_data(input_path: &str, process_number: u16) -> Result<(), String> {
    info!("CSVファイルの分割・追記処理を開始します: {}", input_path);

    // 入力ファイルから「親フォルダ」のパスを取得
    let input_dir = Path::new(input_path)
        .parent()
        .unwrap_or(Path::new(""));

    let file = File::open(input_path).map_err(|e| format!("ファイルが開けません: {}", e))?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false) 
        .from_reader(decode_sjis_reader(file));

    for (line_number, result) in rdr.records().enumerate() {
        let record = match result {
            Ok(rec) => rec,
            Err(e) => {
                error!("{}行目の読み込みエラー: {}", line_number + 1, e);
                continue;
            }
        };

        if record.len() < 9 {
            error!("{}行目は列が足りないためスキップしました", line_number + 1);
            continue;
        }

        // 1〜8列目を抽出してカンマ区切りの文字列を作成
        let output_data: Vec<&str> = record.iter().take(8).collect();
        let row_string = format!("{}\r\n", output_data.join(","));

        // 9列目を取得
        let target_number = record.get(8).unwrap_or("").trim();
        if target_number.is_empty() {
            error!("{}行目は9列目の数字が空のためスキップしました", line_number + 1);
            continue;
        }

        // 処理番号と9列目の数字を組み合わせてファイル名を作成（例: 12_101.csv）
        let output_filename = format!("社員マスタ_{}_{}.csv", target_number, process_number);
        let output_path = input_dir.join(&output_filename);

        let sjis_bytes = match encode_to_sjis(&row_string) {
            Ok(b) => b,
            Err(e) => {
                error!("{}行目のSJIS変換に失敗: {}", line_number + 1, e);
                continue;
            }
        };

        // 追記モードでファイルを開き、書き込む
        let mut output_file = match OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&output_path)
        {
            Ok(f) => f,
            Err(e) => {
                error!("出力ファイル {:?} の準備に失敗: {}", output_path, e);
                continue;
            }
        };

        if let Err(e) = output_file.write_all(&sjis_bytes) {
            error!("出力ファイル {:?} への書き込みに失敗: {}", output_path, e);
        }
    }

    info!("すべてのデータの追記が完了しました！");
    Ok(())
}