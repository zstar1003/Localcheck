import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import { listen } from "@tauri-apps/api/event";
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
  truncated: boolean;
}

interface AnalysisProgress {
  progress: number;
  current_line: number;
  total_lines: number;
  issues_found: number;
  message: string;
}

interface AsyncAnalysisResult {
  completed: boolean;
  progress?: AnalysisProgress;
  result?: AnalysisResult;
  error?: string;
}

function App() {
  const [text, setText] = useState<string>("");
  const [fileName, setFileName] = useState<string>("");
  const [filePath, setFilePath] = useState<string>("");
  const [analysisResult, setAnalysisResult] = useState<AnalysisResult | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [isLargeFile, setIsLargeFile] = useState<boolean>(false);
  const [ignoredIssues, setIgnoredIssues] = useState<Set<number>>(new Set());
  const [selectedFilter, setSelectedFilter] = useState<string>("all");
  const [showAboutDialog, setShowAboutDialog] = useState<boolean>(false);
  const [analysisProgress, setAnalysisProgress] = useState<AnalysisProgress | null>(null);
  const [currentAnalysisId, setCurrentAnalysisId] = useState<string | null>(null);
  const editorRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  // 添加调试信息和事件监听器
  useEffect(() => {
    console.log("App component mounted");
    console.log("Tauri API available:", !!invoke);

    // 检查DOM元素是否正确渲染
    setTimeout(() => {
      console.log("Editor container:", document.querySelector('.editor-container'));
      console.log("Results container:", document.querySelector('.results-container'));
      console.log("Main content:", document.querySelector('.main-content'));
    }, 1000);

    // 设置异步分析事件监听器
    const setupAsyncListeners = async () => {
      // 监听分析进度
      await listen<AsyncAnalysisResult>('analysis_progress', (event) => {
        if (event.payload.progress) {
          setAnalysisProgress(event.payload.progress);
        }
      });

      // 监听分析完成
      await listen<AsyncAnalysisResult>('analysis_complete', (event) => {
        setIsAnalyzing(false);
        setAnalysisProgress(null);
        setCurrentAnalysisId(null);

        if (event.payload.result) {
          setAnalysisResult(event.payload.result);
          setIgnoredIssues(new Set());
          setSelectedFilter("all");
        } else if (event.payload.error) {
          setError(`分析失败: ${event.payload.error}`);
        }
      });
    };

    setupAsyncListeners().catch(console.error);
  }, []);

  // 分析文本（支持异步和同步模式）
  const analyzeText = async () => {
    if (!text.trim() && !filePath) return;

    setIsAnalyzing(true);
    setError(null);
    setAnalysisProgress(null);

    try {
      // 检查文本长度，决定使用同步还是异步分析
      const shouldUseAsync = text.length > 10000 || (isLargeFile && filePath);

      if (shouldUseAsync && !isLargeFile) {
        // 使用异步分析处理大文本
        console.log("Using async analysis for large text:", text.substring(0, 50) + "...");
        const analysisId = await invoke<string>("analyze_text_async", { text });
        setCurrentAnalysisId(analysisId);
        // 异步分析的结果会通过事件监听器处理
      } else if (isLargeFile && filePath) {
        // 使用文件路径分析大文件（保持原有逻辑）
        console.log("Analyzing large file:", filePath);
        const result = await invoke<AnalysisResult>("analyze_large_file", { path: filePath });
        setAnalysisResult(result);
        setIgnoredIssues(new Set());
        setSelectedFilter("all");
        setIsAnalyzing(false);
      } else {
        // 使用同步分析处理小文本
        console.log("Using sync analysis for small text:", text.substring(0, 50) + "...");
        const result = await invoke<AnalysisResult>("analyze_text", { text });
        setAnalysisResult(result);
        setIgnoredIssues(new Set());
        setSelectedFilter("all");
        setIsAnalyzing(false);
      }
    } catch (error) {
      console.error("分析文本时出错:", error);
      setError(`分析失败: ${error}`);
      setIsAnalyzing(false);
      setAnalysisProgress(null);
      setCurrentAnalysisId(null);
    }
  };

  // 取消分析
  const cancelAnalysis = () => {
    setIsAnalyzing(false);
    setAnalysisProgress(null);
    setCurrentAnalysisId(null);
    setError("分析已取消");
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
        setFilePath(selected);
        
        try {
          const content = await invoke<string>("read_file_content", { path: selected });
          console.log("File content length:", content.length);
          
          // 提取文件名
          const pathParts = selected.split(/[/\\]/);
          const fileName = pathParts[pathParts.length - 1];
          setFileName(fileName);
          
          // 检查是否为大文件
          const isLarge = content.length > 50000;
          setIsLargeFile(isLarge);
          
          if (isLarge) {
            setText("文件过大，仅显示部分内容...\n\n" + content);
          } else {
            setText(content);
          }
          
          // 自动分析
          setTimeout(() => {
            analyzeText();
          }, 100);
        } catch (error) {
          console.error("读取文件内容时出错:", error);
          setError(`无法读取文件: ${error}`);
        }
      }
    } catch (error) {
      console.error("打开文件时出错:", error);
      setError(`打开文件对话框时出错: ${error}`);
    }
  };

  // 处理文本变化
  const handleTextChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setText(e.target.value);
    setIsLargeFile(false); // 用户编辑了文本，不再是大文件模式
  };

  // 点击问题项，高亮对应文本
  const handleIssueClick = (issue: TextIssue) => {
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

  // 接受建议，自动修改文本
  const handleAcceptSuggestion = (issue: TextIssue, index: number) => {
    if (textareaRef.current) {
      try {
        // 计算行的位置
        const lines = text.split("\n");
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
        const originalText = text.substring(start, end);

        // 提取建议的修改文本
        let replacement = "";

        // 处理不同类型的建议
        if (issue.suggestion.includes("建议修改为:")) {
          // 拼写错误修正
          const match = issue.suggestion.match(/建议修改为:\s*['"]([^'"]+)['"]/);
          if (match) {
            replacement = match[1];
          }
        } else if (issue.suggestion.includes("应使用:")) {
          // 成语用法修正
          const match = issue.suggestion.match(/应使用:\s*['"]([^'"]+)['"]/);
          if (match) {
            replacement = match[1];
          }
        } else if (issue.suggestion.includes("删除重复的")) {
          // 重复词删除
          if (issue.issue_type === "重复词") {
            // 对于重复词，删除后面的重复部分
            const words = originalText.split(/\s+/);
            if (words.length >= 2 && words[0] === words[1]) {
              replacement = words[0];
            }
          } else if (issue.issue_type === "重复字符") {
            // 对于重复字符，删除一个
            replacement = originalText.charAt(0);
          }
        } else if (issue.suggestion.includes("建议使用")) {
          // 非正式代词替换
          if (issue.suggestion.includes("建议使用 '我们'")) {
            replacement = "我们";
          }
        } else if (issue.suggestion.includes("删除多余的标点")) {
          // 标点符号问题
          replacement = originalText.replace(/[，。！？；：""''（）【】《》〈〉「」『』〔〕［］｛｝〖〗]+$/, "");
        }

        if (replacement !== "" && replacement !== originalText) {
          // 执行文本替换
          const newText = text.substring(0, start) + replacement + text.substring(end);
          setText(newText);

          // 将问题标记为已忽略（因为已经修复）
          setIgnoredIssues(prev => new Set([...prev, index]));

          console.log(`Applied suggestion: "${originalText}" -> "${replacement}"`);
        } else {
          // 如果无法自动修复，只是忽略问题
          setIgnoredIssues(prev => new Set([...prev, index]));
          console.log(`Cannot auto-fix, ignoring issue: ${issue.message}`);
        }
      } catch (error) {
        console.error("应用建议时出错:", error);
        // 出错时也忽略问题
        setIgnoredIssues(prev => new Set([...prev, index]));
      }
    }
  };

  // 忽略问题
  const handleIgnoreIssue = (index: number) => {
    setIgnoredIssues(prev => new Set([...prev, index]));
  };

  // 清除所有忽略的问题
  const handleClearIgnored = () => {
    setIgnoredIssues(new Set());
  };

  // 获取所有唯一的错误类型
  const getUniqueIssueTypes = () => {
    if (!analysisResult) return [];
    const types = new Set(analysisResult.issues.map(issue => issue.issue_type));
    return Array.from(types).sort();
  };

  // 根据筛选条件过滤问题
  const getFilteredIssues = () => {
    if (!analysisResult) return [];

    return analysisResult.issues
      .map((issue, index) => ({ issue, index }))
      .filter(({ index }) => !ignoredIssues.has(index))
      .filter(({ issue }) => selectedFilter === "all" || issue.issue_type === selectedFilter);
  };

  // 处理筛选器变化
  const handleFilterChange = (filterType: string) => {
    setSelectedFilter(filterType);
  };

  return (
    <div className="app-container">
      <div className="header">
        <div>
          <button className="button" onClick={openFile}>打开文件</button>
          <button
            className="button"
            onClick={analyzeText}
            disabled={(!text.trim() && !filePath) || isAnalyzing}
            style={{ marginLeft: '10px' }}
          >
            {isAnalyzing ? (analysisProgress ? "异步分析中..." : "分析中...") : "分析文本"}
          </button>
          {isAnalyzing && currentAnalysisId && (
            <button
              className="button button-secondary"
              onClick={cancelAnalysis}
              style={{ marginLeft: '10px' }}
            >
              取消分析
            </button>
          )}
        </div>
        <div>
          <button
            className="button button-info"
            onClick={() => setShowAboutDialog(true)}
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
              {isLargeFile && " (大文件模式)"}
            </div>
          </div>
          <div className="editor-content" ref={editorRef}>
            <textarea
              ref={textareaRef}
              className="editor-textarea"
              value={text}
              onChange={handleTextChange}
              placeholder="在此输入或粘贴文本，或者点击'打开文件'按钮导入文件..."
            />
          </div>
        </div>

        <div className="results-container">
          <div className="results-header">
            <span>分析结果</span>
            {analysisResult && analysisResult.issues.length > 0 && (
              <div className="filter-container">
                <select
                  className="filter-select"
                  value={selectedFilter}
                  onChange={(e) => handleFilterChange(e.target.value)}
                >
                  <option value="all">全部类型</option>
                  {getUniqueIssueTypes().map(type => (
                    <option key={type} value={type}>{type}</option>
                  ))}
                </select>
              </div>
            )}
          </div>
          <div className="results-content">
            {error ? (
              <div className="error-message">
                {error}
              </div>
            ) : analysisResult ? (
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
                    <span>{analysisResult.issues.length}{analysisResult.truncated ? "+" : ""}</span>
                  </div>
                  {selectedFilter !== "all" && (
                    <div className="stats-item">
                      <span>当前筛选:</span>
                      <span>{getFilteredIssues().length} 个 {selectedFilter}</span>
                    </div>
                  )}
                </div>

                {analysisResult.truncated && (
                  <div className="warning-message">
                    注意: 文本过长或问题过多，仅显示部分分析结果。
                  </div>
                )}

                {isLargeFile && (
                  <div className="info-message">
                    大文件模式: 文件较大，仅显示部分内容和分析结果。
                  </div>
                )}

                {analysisResult.issues.length > 0 ? (
                  <>
                    <div className="filter-info">
                      {ignoredIssues.size > 0 && (
                        <div className="ignored-info">
                          <span>已忽略 {ignoredIssues.size} 个问题</span>
                          <button
                            className="button button-small button-secondary"
                            onClick={handleClearIgnored}
                          >
                            显示全部
                          </button>
                        </div>
                      )}
                      {selectedFilter !== "all" && (
                        <div className="filter-active-info">
                          <span>筛选: {selectedFilter}</span>
                          <button
                            className="button button-small button-secondary"
                            onClick={() => handleFilterChange("all")}
                          >
                            清除筛选
                          </button>
                        </div>
                      )}
                    </div>
                    {getFilteredIssues().map(({ issue, index }) => (
                      <div
                        key={index}
                        className="issue-item"
                      >
                        <div
                          className="issue-content"
                          onClick={() => handleIssueClick(issue)}
                        >
                          <div className="issue-header">
                            <span className="issue-type">{issue.issue_type}</span>
                            <span className="issue-location">行 {issue.line_number}</span>
                          </div>
                          <div className="issue-message">{issue.message}</div>
                          <div className="issue-suggestion">{issue.suggestion}</div>
                        </div>
                        <div className="issue-actions">
                          <button
                            className="button button-small button-accept"
                            onClick={(e) => {
                              e.stopPropagation();
                              handleAcceptSuggestion(issue, index);
                            }}
                            title="接受建议并自动修改"
                          >
                            接受
                          </button>
                          <button
                            className="button button-small button-ignore"
                            onClick={(e) => {
                              e.stopPropagation();
                              handleIgnoreIssue(index);
                            }}
                            title="忽略此问题"
                          >
                            忽略
                          </button>
                        </div>
                      </div>
                    ))}
                    {getFilteredIssues().length === 0 && (
                      <div style={{ padding: '1rem', textAlign: 'center' }}>
                        {selectedFilter === "all" ? "所有问题都已处理！" : `没有 "${selectedFilter}" 类型的问题`}
                      </div>
                    )}
                  </>
                ) : (
                  <div style={{ padding: '1rem', textAlign: 'center' }}>
                    没有检测到问题，文本质量良好！
                  </div>
                )}
              </>
            ) : (
              <div style={{ padding: '1rem', textAlign: 'center' }}>
                {text.trim() || filePath ? (
                  isAnalyzing ? (
                    analysisProgress ? (
                      <div className="analysis-progress">
                        <div className="progress-message">{analysisProgress.message}</div>
                        <div className="progress-bar-container">
                          <div
                            className="progress-bar"
                            style={{ width: `${analysisProgress.progress}%` }}
                          ></div>
                        </div>
                        <div className="progress-stats">
                          进度: {Math.round(analysisProgress.progress)}% |
                          行数: {analysisProgress.current_line}/{analysisProgress.total_lines} |
                          发现问题: {analysisProgress.issues_found}
                        </div>
                      </div>
                    ) : (
                      "正在分析文本..."
                    )
                  ) : (
                    "点击'分析文本'按钮开始检查"
                  )
                ) : (
                  "请输入或导入文本进行分析"
                )}
              </div>
            )}
          </div>
        </div>
      </div>

      {/* 关于对话框 */}
      {showAboutDialog && (
        <div className="modal-overlay" onClick={() => setShowAboutDialog(false)}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h2>关于软件</h2>
              <button
                className="modal-close"
                onClick={() => setShowAboutDialog(false)}
              >
                ×
              </button>
            </div>
            <div className="modal-body">
              <div className="about-info">
                <h3>论文本地校验工具 (LocalCheck)</h3>
                <p className="version">版本: v0.1.0</p>

                <div className="about-section">
                  <h4>软件介绍</h4>
                  <p>一个基于Tauri的论文本地校验工具，可以对导入的文本进行逐行校验，检测出文本中的错误并给出优化建议。支持中英文混合文本分析，自动识别语言。</p>
                </div>

                <div className="about-section">
                  <h4>主要功能</h4>
                  <ul>
                    <li>拼写错误检测与修正建议</li>
                    <li>语法错误识别</li>
                    <li>重复词语检测</li>
                    <li>学术写作风格检查</li>
                    <li>标点符号规范检查</li>
                    <li>中英文混合文本支持</li>
                  </ul>
                </div>

                <div className="about-section">
                  <h4>开发信息</h4>
                  <div className="dev-info">
                    <p><strong>作者:</strong> zstar</p>
                    <p><strong>开源仓库:</strong>
                      <a
                        href="https://github.com/zstar1003/Localcheck"
                        target="_blank"
                        rel="noopener noreferrer"
                        className="link"
                      >
                        https://github.com/zstar1003/Localcheck
                      </a>
                    </p>
                    <p><strong>微信公众号:</strong> 我有一计</p>
                  </div>
                </div>
              </div>
            </div>
            <div className="modal-footer">
              <button
                className="button button-primary"
                onClick={() => setShowAboutDialog(false)}
              >
                确定
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;