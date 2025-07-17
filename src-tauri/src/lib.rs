use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextIssue {
    line_number: usize,
    start: usize,
    end: usize,
    issue_type: String,
    message: String,
    suggestion: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisResult {
    issues: Vec<TextIssue>,
    stats: HashMap<String, usize>,
}

#[tauri::command]
fn analyze_text(text: &str, language: &str) -> AnalysisResult {
    let mut issues = Vec::new();
    let mut stats = HashMap::new();
    
    // 计算基本统计信息
    let total_chars = text.chars().count();
    let total_words = text.split_whitespace().count();
    let total_lines = text.lines().count();
    
    stats.insert("total_chars".to_string(), total_chars);
    stats.insert("total_words".to_string(), total_words);
    stats.insert("total_lines".to_string(), total_lines);
    
    // 分析每一行
    for (line_idx, line) in text.lines().enumerate() {
        // 检查行长度
        if line.len() > 100 {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: 0,
                end: line.len(),
                issue_type: "长句".to_string(),
                message: "这一行过长，可能影响阅读流畅度".to_string(),
                suggestion: "考虑将长句拆分为多个短句".to_string(),
            });
        }
        
        // 检查重复词
        check_repeated_words(line, line_idx, &mut issues);
        
        // 检查标点符号使用
        check_punctuation(line, line_idx, &mut issues);
        
        // 检查被动语态 (简化版)
        check_passive_voice(line, line_idx, &mut issues, language);
        
        // 检查冗余表达
        check_redundant_expressions(line, line_idx, &mut issues, language);
    }
    
    AnalysisResult { issues, stats }
}

fn check_repeated_words(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    let words: Vec<&str> = line.split_whitespace().collect();
    
    for i in 0..words.len().saturating_sub(1) {
        if words[i].len() > 3 && words[i] == words[i + 1] {
            let start_pos = line.find(words[i]).unwrap_or(0);
            let end_pos = start_pos + words[i].len() * 2 + 1; // 包括空格
            
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: start_pos,
                end: end_pos,
                issue_type: "重复词".to_string(),
                message: format!("重复使用词语 '{}'", words[i]),
                suggestion: format!("删除重复的 '{}'", words[i]),
            });
        }
    }
}

fn check_punctuation(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // 检查中英文标点混用
    let cn_punct_regex = Regex::new(r#"[，。！？；：""''（）【】「」『』〈〉《》]"#).unwrap();
    let en_punct_regex = Regex::new(r#"[,.!?;:"'()\[\]<>]"#).unwrap();
    
    if cn_punct_regex.is_match(line) && en_punct_regex.is_match(line) {
        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: 0,
            end: line.len(),
            issue_type: "标点混用".to_string(),
            message: "中英文标点符号混用".to_string(),
            suggestion: "请统一使用中文或英文标点符号".to_string(),
        });
    }
    
    // 检查连续标点
    let consecutive_punct_regex = Regex::new(r"[,.!?;:]{2,}").unwrap();
    if let Some(mat) = consecutive_punct_regex.find(line) {
        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: mat.start(),
            end: mat.end(),
            issue_type: "连续标点".to_string(),
            message: "连续使用多个标点符号".to_string(),
            suggestion: "使用单个适当的标点符号".to_string(),
        });
    }
}

fn check_passive_voice(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>, language: &str) {
    if language == "zh" {
        // 中文被动语态检测 (简化版)
        let passive_markers = ["被", "受到", "遭到", "遭受"];
        
        for marker in passive_markers {
            if let Some(pos) = line.find(marker) {
                issues.push(TextIssue {
                    line_number: line_idx + 1,
                    start: pos,
                    end: pos + marker.len(),
                    issue_type: "被动语态".to_string(),
                    message: "使用了被动语态".to_string(),
                    suggestion: "考虑使用主动语态以增强表达力".to_string(),
                });
            }
        }
    } else {
        // 英文被动语态检测 (简化版)
        let be_verbs = ["is", "are", "was", "were", "be", "been", "being"];
        let past_participles = ["ed", "en", "t"];
        
        for be_verb in be_verbs {
            if let Some(pos) = line.to_lowercase().find(be_verb) {
                // 简单检查后面是否跟着过去分词
                let after_be = &line[pos + be_verb.len()..];
                let words_after: Vec<&str> = after_be.split_whitespace().collect();
                
                if let Some(next_word) = words_after.first() {
                    for suffix in past_participles {
                        if next_word.to_lowercase().ends_with(suffix) {
                            issues.push(TextIssue {
                                line_number: line_idx + 1,
                                start: pos,
                                end: pos + be_verb.len() + after_be.find(next_word).unwrap_or(0) + next_word.len(),
                                issue_type: "Passive Voice".to_string(),
                                message: "Passive voice detected".to_string(),
                                suggestion: "Consider using active voice for stronger writing".to_string(),
                            });
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn check_redundant_expressions(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>, language: &str) {
    let redundant_expressions: HashMap<&str, &str> = if language == "zh" {
        [
            ("事实上", "可以直接陈述事实"),
            ("总的来说", "可以省略"),
            ("基本上", "可以省略"),
            ("实际上", "可以直接陈述事实"),
            ("从某种程度上讲", "可以更明确地表达"),
            ("可以说是", "可以省略"),
        ].iter().cloned().collect()
    } else {
        [
            ("in order to", "use 'to' instead"),
            ("due to the fact that", "use 'because' instead"),
            ("in spite of the fact that", "use 'although' instead"),
            ("it is important to note that", "omit this phrase"),
            ("for all intents and purposes", "use 'essentially' or omit"),
        ].iter().cloned().collect()
    };
    
    for (phrase, suggestion) in redundant_expressions {
        if let Some(pos) = line.to_lowercase().find(&phrase.to_lowercase()) {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: pos,
                end: pos + phrase.len(),
                issue_type: "冗余表达".to_string(),
                message: format!("冗余表达: '{}'", phrase),
                suggestion: suggestion.to_string(),
            });
        }
    }
}

#[tauri::command]
fn read_file_content(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            analyze_text,
            read_file_content
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
