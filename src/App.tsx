import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greeting, setGreeting] = useState("");
  const [inputText, setInputText] = useState("");

  // 简单的测试函数
  async function testBackend() {
    try {
      // 尝试调用后端的analyze_text函数
      const result = await invoke("analyze_text", { 
        text: "这是一个测试文本。这是一个测试文本。", 
        language: "zh" 
      });
      console.log("Backend response:", result);
      setGreeting("后端连接成功！");
    } catch (error) {
      console.error("Error calling backend:", error);
      setGreeting(`后端连接失败: ${error}`);
    }
  }

  return (
    <div className="container">
      <h1>论文本地校验工具 - 测试版</h1>
      <div className="row">
        <button onClick={testBackend}>测试后端连接</button>
      </div>
      <p>{greeting}</p>
      <div className="row">
        <textarea 
          value={inputText}
          onChange={(e) => setInputText(e.target.value)}
          placeholder="在此输入文本..."
          rows={10}
          cols={50}
        />
      </div>
    </div>
  );
}

export default App;