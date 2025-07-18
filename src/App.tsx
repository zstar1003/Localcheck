import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import "./App.css";

interface TextIssue {
  line_number: number;
  start: number;
  end: number;
  issue_type: string;
  message: string;
  suggestion: string;
}

interface AnalysisResult {
  issues: TextIssue[];
  stats: Record<string, number>;
}

function App() {
  const [text, setText] = useState<string>("");
  const [fileName, setFileName] = useState<string>("");
  // 自动检测语言，不再需要语言选择器
  const [analysisResult, setAnalysisResult] = useState<AnalysisResult | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState<boolean>(false);
  const [selectedIssue, setSelectedIssue] = useState<TextIssue | null>(null);
  // Removed unused tooltipPosition state
  const editorRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  // 添加调试信息
  useEffect(() => {
    console.log("App component mounted");
    console.log("Tauri API available:", !!invoke);
    
    // 检查DOM元素是否正确渲染
    setTimeout(() => {
      console.log("Editor container:", document.querySelector('.editor-container'));
      console.log("Results container:", document.querySelector('.results-container'));
      console.log("Main content:", document.querySelector('.main-content'));
    }, 1000);
  }, []);

  // 分析文本
  const analyzeText = async () => {
    if (!text.trim()) return;
    
    setIsAnalyzing(true);
    try {
      console.log("Analyzing text:", text.substring(0, 50) + "...");
      
      // 不再需要传递language参数，后端会自动检测
      const result = await invoke<AnalysisResult>("analyze_text", { text });
      console.log("Analysis result:", result);
      setAnalysisResult(result);
    } catch (error) {
      console.error("分析文本时出错:", error);
    } finally {
      setIsAnalyzing(false);
    }
  };

  // 打开文件
  const openFile = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          { name: "文本文件", extensions: ["txt", "md", "doc", "docx"] },
          { name: "所有文件", extensions: ["*"] }
        ]
      });
      
      if (selected && typeof selected === "string") {
        console.log("Selected file:", selected);
        const content = await invoke<string>("read_file_content", { path: selected });
        console.log("File content length:", content.length);
        setText(content);
        
        // 提取文件名
        const pathParts = selected.split(/[/\\]/);
        setFileName(pathParts[pathParts.length - 1]);
        
        // 自动分析
        setTimeout(() => {
          analyzeText();
        }, 100);
      }
    } catch (error) {
      console.error("打开文件时出错:", error);
    }
  };

  // 用于存储防抖定时器ID
  const debounceTimerRef = useRef<number | null>(null);

  // 处理文本变化
  const handleTextChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setText(e.target.value);
    // 不清除分析结果，保持显示
    
    // 设置防抖定时器，在用户停止输入1秒后自动重新分析
    if (debounceTimerRef.current) {
      clearTimeout(debounceTimerRef.current);
    }
    
    debounceTimerRef.current = window.setTimeout(() => {
      if (e.target.value.trim()) {
        analyzeText();
      }
    }, 1000);
  };

  // 点击问题项，高亮对应文本
  const handleIssueClick = (issue: TextIssue) => {
    setSelectedIssue(issue);
    
    if (textareaRef.current) {
      try {
        // 计算行的位置
        const lines = text.split("\n");
        
        // 确保行号在有效范围内
        const lineIndex = Math.min(issue.line_number - 1, lines.length - 1);
        const line = lines[lineIndex];
        
        // 计算行的起始位置（字符偏移量）
        let position = 0;
        for (let i = 0; i < lineIndex; i++) {
          position += lines[i].length + 1; // +1 for newline character
        }
        
        // 确保起始和结束位置在有效范围内
        const start = position + Math.min(issue.start, line.length);
        const end = position + Math.min(issue.end, line.length);
        
        console.log(`Highlighting issue: Line ${issue.line_number}, Start: ${issue.start}, End: ${issue.end}`);
        console.log(`Text position: Start: ${start}, End: ${end}`);
        console.log(`Line content: "${line}"`);
        console.log(`Highlighted text: "${text.substring(start, end)}"`);
        
        // 设置选择范围
        textareaRef.current.focus();
        textareaRef.current.setSelectionRange(start, end);
        
        // 滚动到可见区域
        const lineHeight = 24; // 估计的行高
        const scrollTop = lineIndex * lineHeight - 100;
        if (editorRef.current) {
          editorRef.current.scrollTop = scrollTop > 0 ? scrollTop : 0;
        }
      } catch (error) {
        console.error("高亮文本时出错:", error);
      }
    }
  };

  // 渲染带有高亮的文本
  const renderHighlightedText = () => {
    if (!analysisResult || analysisResult.issues.length === 0) {
      return <textarea
        ref={textareaRef}
        className="editor-textarea"
        value={text}
        onChange={handleTextChange}
        placeholder="在此输入或粘贴文本，或者点击'打开文件'按钮导入文件..."
      />;
    }

    return (
      <textarea
        ref={textareaRef}
        className="editor-textarea"
        value={text}
        onChange={handleTextChange}
        placeholder="在此输入或粘贴文本，或者点击'打开文件'按钮导入文件..."
      />
    );
  };

  return (
    <div className="app-container">
      <div className="header">
        <div>
          <button className="button" onClick={openFile}>打开文件</button>
          <button 
            className="button" 
            onClick={analyzeText} 
            disabled={!text.trim() || isAnalyzing}
            style={{ marginLeft: '10px' }}
          >
            {isAnalyzing ? "分析中..." : "分析文本"}
          </button>
        </div>
        <div>
          <button 
            className="button button-info" 
            onClick={() => alert("论文本地校验工具 v1.0\n\n一个基于Tauri的论文本地校验工具，可以对导入的文本进行逐行校验，检测出文本中的错误并给出优化建议。\n\n支持中英文混合文本分析，自动识别语言。")}
          >
            关于软件
          </button>
        </div>
      </div>

      <div className="main-content">
        <div className="editor-container">
          <div className="editor-header">
            <div className="file-info">
              {fileName ? `文件: ${fileName}` : "新文档"}
            </div>
          </div>
          <div className="editor-content" ref={editorRef}>
            {renderHighlightedText()}
          </div>
        </div>

        <div className="results-container">
          <div className="results-header">
            分析结果
          </div>
          <div className="results-content">
            {analysisResult ? (
              <>
                <div className="stats-container">
                  <div className="stats-item">
                    <span>总字符数:</span>
                    <span>{analysisResult.stats.total_chars || 0}</span>
                  </div>
                  <div className="stats-item">
                    <span>总词数:</span>
                    <span>{analysisResult.stats.total_words || 0}</span>
                  </div>
                  <div className="stats-item">
                    <span>总行数:</span>
                    <span>{analysisResult.stats.total_lines || 0}</span>
                  </div>
                  <div className="stats-item">
                    <span>检测到的问题:</span>
                    <span>{analysisResult.issues.length}</span>
                  </div>
                </div>

                {analysisResult.issues.length > 0 ? (
                  analysisResult.issues.map((issue, index) => (
                    <div 
                      key={index} 
                      className="issue-item"
                      onClick={() => handleIssueClick(issue)}
                    >
                      <div className="issue-header">
                        <span className="issue-type">{issue.issue_type}</span>
                        <span className="issue-location">行 {issue.line_number}</span>
                      </div>
                      <div className="issue-message">{issue.message}</div>
                      <div className="issue-suggestion">{issue.suggestion}</div>
                    </div>
                  ))
                ) : (
                  <div style={{ padding: '1rem', textAlign: 'center' }}>
                    没有检测到问题，文本质量良好！
                  </div>
                )}
              </>
            ) : (
              <div style={{ padding: '1rem', textAlign: 'center' }}>
                {text.trim() ? (
                  isAnalyzing ? 
                    "正在分析文本..." : 
                    "点击'分析文本'按钮开始检查"
                ) : (
                  "请输入或导入文本进行分析"
                )}
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Tooltip removed as it's not being used */}
    </div>
  );
}

export default App;