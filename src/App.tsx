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
  const [language, setLanguage] = useState<string>("zh");
  const [analysisResult, setAnalysisResult] = useState<AnalysisResult | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState<boolean>(false);
  const [selectedIssue, setSelectedIssue] = useState<TextIssue | null>(null);
  const [tooltipPosition, setTooltipPosition] = useState<{ top: number; left: number } | null>(null);
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
      console.log("Language:", language);
      
      const result = await invoke<AnalysisResult>("analyze_text", { text, language });
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

  // 处理文本变化
  const handleTextChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setText(e.target.value);
    // 清除之前的分析结果
    setAnalysisResult(null);
  };

  // 点击问题项，高亮对应文本
  const handleIssueClick = (issue: TextIssue) => {
    setSelectedIssue(issue);
    
    if (textareaRef.current) {
      // 计算行的位置
      const lines = text.split("\n");
      let position = 0;
      
      for (let i = 0; i < issue.line_number - 1; i++) {
        position += lines[i].length + 1; // +1 for newline character
      }
      
      // 设置选择范围
      const start = position + issue.start;
      const end = position + issue.end;
      
      textareaRef.current.focus();
      textareaRef.current.setSelectionRange(start, end);
      
      // 滚动到可见区域
      const lineHeight = 24; // 估计的行高
      const scrollTop = (issue.line_number - 1) * lineHeight - 100;
      if (editorRef.current) {
        editorRef.current.scrollTop = scrollTop > 0 ? scrollTop : 0;
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
        <h1>论文本地校验工具</h1>
        <div>
          <select 
            className="language-selector" 
            value={language}
            onChange={(e) => setLanguage(e.target.value)}
          >
            <option value="zh">中文</option>
            <option value="en">英文</option>
          </select>
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

      {tooltipPosition && selectedIssue && (
        <div 
          className="tooltip" 
          style={{ 
            top: tooltipPosition.top, 
            left: tooltipPosition.left 
          }}
        >
          <div className="tooltip-header">{selectedIssue.issue_type}</div>
          <div>{selectedIssue.message}</div>
          <div className="tooltip-suggestion">{selectedIssue.suggestion}</div>
        </div>
      )}
    </div>
  );
}

export default App;