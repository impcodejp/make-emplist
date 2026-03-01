import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";

function App() {
  const [filePath, setFilePath] = useState("");
  // 新しい変数: 処理番号を保存するための状態を追加します
  const [processNumber, setProcessNumber] = useState("");

  const handleBrowse = async () => {
    try {
      const selected = await open({
        multiple: false,
        title: "ファイルを選択してください",
        filters: [{ name: 'CSVファイル', extensions: ['csv'] }]
      });
      if (selected) {
        setFilePath(selected);
      }
    } catch (error) {
      console.error("ファイル選択エラー:", error);
    }
  };

  const handleExecute = async () => {
    if (!filePath) {
      alert("ファイルを選択してください。");
      return;
    }

    // 処理番号のチェック（画面側でも1〜9999のルールを守らせます）
    const num = parseInt(processNumber, 10);
    if (!num || num <= 0 || num > 9999) {
      alert("処理番号は1から9999の間の数字を入力してください。");
      return;
    }

    try {
      // Rust側にパスと「処理番号」の両方を送ります
      const result = await invoke("process_file", { 
        path: filePath,
        processNumber: num // ここで番号を渡します
      });
      alert(result);
    } catch (error) {
      console.error("実行エラー:", error);
      alert("エラーが発生しました: " + error);
    }
  };

  return (
    <div className="app-wrapper">
      <div className="container">
        <h1>従業員リスト作成</h1>
        <p className="subtitle">CSVファイルからリストを生成します</p>
        
        {/* 処理番号の入力エリアを追加 */}
        <div className="input-group">
          <input
            type="number"
            min="1"
            max="9999"
            value={processNumber}
            onChange={(e) => setProcessNumber(e.target.value)}
            placeholder="処理番号 (1〜9999)"
            className="custom-input number-input"
          />
        </div>

        <div className="input-group">
          <input
            type="text"
            value={filePath}
            readOnly
            placeholder="ファイルパス..."
            className="custom-input path-input"
          />
          <button onClick={handleBrowse} className="btn-browse">
            参照
          </button>
        </div>

        <button onClick={handleExecute} className="btn-execute">
          実行する
        </button>
      </div>
    </div>
  );
}

export default App;