use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextIssue {
    line_number: usize,
    start: usize,
    end: usize,
    issue_type: String,
    message: String,
    suggestion: String,
}

// 将字节索引转换为字符索引
fn byte_to_char_index(s: &str, byte_idx: usize) -> usize {
    s[..byte_idx.min(s.len())].chars().count()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisResult {
    issues: Vec<TextIssue>,
    stats: HashMap<String, usize>,
}

#[tauri::command]
fn analyze_text(text: &str) -> AnalysisResult {
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
        // 自动检测当前行的语言
        let line_language = detect_language(line);

        // 检查重复词
        check_repeated_words(line, line_idx, &mut issues);

        // 检查标点符号使用
        check_punctuation(line, line_idx, &mut issues);

        // 检查被动语态 (简化版)
        check_passive_voice(line, line_idx, &mut issues, &line_language);

        // 检查冗余表达
        check_redundant_expressions(line, line_idx, &mut issues, &line_language);

        // 检查常见错别字
        check_common_typos(line, line_idx, &mut issues, &line_language);

        // 检查语法问题
        check_grammar_issues(line, line_idx, &mut issues, &line_language);
    }

    AnalysisResult { issues, stats }
}

fn check_repeated_words(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    let words: Vec<&str> = line.split_whitespace().collect();

    for i in 0..words.len().saturating_sub(1) {
        if words[i].len() > 3 && words[i] == words[i + 1] {
            // 查找第一个单词的位置
            let first_word_pos = match line.find(words[i]) {
                Some(pos) => pos,
                None => continue, // 如果找不到单词，跳过这个检查
            };

            // 查找第二个单词的位置（从第一个单词之后开始查找）
            let second_word_pos = match line[first_word_pos + words[i].len()..].find(words[i]) {
                Some(pos) => first_word_pos + words[i].len() + pos,
                None => continue, // 如果找不到第二个单词，跳过这个检查
            };

            // 确保两个单词之间只有空白字符
            let between_text = &line[first_word_pos + words[i].len()..second_word_pos];
            if !between_text.trim().is_empty() {
                continue; // 如果两个单词之间有非空白字符，跳过这个检查
            }

            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, first_word_pos),
                end: byte_to_char_index(line, second_word_pos + words[i].len()),
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
            start: byte_to_char_index(line, mat.start()),
            end: byte_to_char_index(line, mat.end()),
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
                    start: byte_to_char_index(line, pos),
                    end: byte_to_char_index(line, pos + marker.len()),
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
                            let end_pos = pos
                                + be_verb.len()
                                + after_be.find(next_word).unwrap_or(0)
                                + next_word.len();
                            issues.push(TextIssue {
                                line_number: line_idx + 1,
                                start: byte_to_char_index(line, pos),
                                end: byte_to_char_index(line, end_pos),
                                issue_type: "被动语态".to_string(),
                                message: "检测到被动语态".to_string(),
                                suggestion: "考虑使用主动语态以增强表达力".to_string(),
                            });
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn check_redundant_expressions(
    line: &str,
    line_idx: usize,
    issues: &mut Vec<TextIssue>,
    language: &str,
) {
    let redundant_expressions: HashMap<&str, &str> = if language == "zh" {
        [
            ("事实上", "可以直接陈述事实"),
            ("总的来说", "可以省略"),
            ("基本上", "可以省略"),
            ("实际上", "可以直接陈述事实"),
            ("从某种程度上讲", "可以更明确地表达"),
            ("可以说是", "可以省略"),
        ]
        .iter()
        .cloned()
        .collect()
    } else {
        [
            ("in order to", "use 'to' instead"),
            ("due to the fact that", "use 'because' instead"),
            ("in spite of the fact that", "use 'although' instead"),
            ("it is important to note that", "omit this phrase"),
            ("for all intents and purposes", "use 'essentially' or omit"),
        ]
        .iter()
        .cloned()
        .collect()
    };

    for (phrase, suggestion) in redundant_expressions {
        if let Some(pos) = line.to_lowercase().find(&phrase.to_lowercase()) {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, pos),
                end: byte_to_char_index(line, pos + phrase.len()),
                issue_type: "冗余表达".to_string(),
                message: format!("冗余表达: '{}'", phrase),
                suggestion: suggestion.to_string(),
            });
        }
    }
}

fn check_common_typos(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>, language: &str) {
    // 中文重复字符检测
    if language == "zh" {
        // 检测连续重复的单字符
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len().saturating_sub(1) {
            if chars[i] == chars[i + 1] && chars[i] >= '\u{4e00}' && chars[i] <= '\u{9fff}' {
                // 是中文字符且连续重复

                // 计算字符在原始字符串中的字节位置
                let start_byte_pos = line.char_indices().nth(i).map(|(pos, _)| pos).unwrap_or(0);

                let end_byte_pos = line
                    .char_indices()
                    .nth(i + 2)
                    .map(|(pos, _)| pos)
                    .unwrap_or_else(|| line.len());

                issues.push(TextIssue {
                    line_number: line_idx + 1,
                    start: byte_to_char_index(line, start_byte_pos),
                    end: byte_to_char_index(line, end_byte_pos),
                    issue_type: "重复字符".to_string(),
                    message: format!("重复字符: '{}{}'", chars[i], chars[i]),
                    suggestion: format!("删除重复的 '{}'", chars[i]),
                });

                i += 2; // 跳过已检测的重复字符
            } else {
                i += 1;
            }
        }
    } else {
        // 英文常见错别字检测
        let typos: HashMap<&str, &str> = [
            ("teh", "the"),
            ("recieve", "receive"),
            ("wierd", "weird"),
            ("alot", "a lot"),
            ("definately", "definitely"),
            ("seperate", "separate"),
            ("occured", "occurred"),
            ("untill", "until"),
            ("wich", "which"),
            ("recieved", "received"),
            ("accomodate", "accommodate"),
            ("adress", "address"),
            ("beleive", "believe"),
            ("concious", "conscious"),
            ("existance", "existence"),
            ("goverment", "government"),
            ("independant", "independent"),
            ("liason", "liaison"),
            ("neccessary", "necessary"),
            ("occassion", "occasion"),
            ("occassionally", "occasionally"),
            ("persistant", "persistent"),
            ("posession", "possession"),
            ("publically", "publicly"),
            ("reccomend", "recommend"),
            ("relevent", "relevant"),
            ("religous", "religious"),
            ("rythm", "rhythm"),
            ("sieze", "seize"),
            ("sincerly", "sincerely"),
            ("supercede", "supersede"),
            ("tommorrow", "tomorrow"),
            ("twelth", "twelfth"),
            ("tyrany", "tyranny"),
            ("underate", "underrate"),
            ("untill", "until"),
            ("wether", "whether"),
            ("withhold", "withhold"),
            ("writting", "writing"),
        ]
        .iter()
        .cloned()
        .collect();

        for (typo, correction) in typos {
            // 使用正则表达式匹配整个单词
            let pattern = format!(r"\b{}\b", typo);
            let regex = Regex::new(&pattern).unwrap();

            for mat in regex.find_iter(line) {
                issues.push(TextIssue {
                    line_number: line_idx + 1,
                    start: byte_to_char_index(line, mat.start()),
                    end: byte_to_char_index(line, mat.end()),
                    issue_type: "拼写错误".to_string(),
                    message: format!("可能的拼写错误: '{}'", typo),
                    suggestion: format!("建议修改为: '{}'", correction),
                });
            }
        }
    }
}

fn check_grammar_issues(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>, language: &str) {
    if language == "zh" {
        // 中文语法检查

        // 检查"的得地"用法
        check_de_usage(line, line_idx, issues);

        // 检查常见病句
        check_common_chinese_errors(line, line_idx, issues);
    } else {
        // 英文语法检查

        // 检查主谓一致
        check_subject_verb_agreement(line, line_idx, issues);

        // 检查冠词使用
        check_article_usage(line, line_idx, issues);

        // 检查常见语法错误
        check_common_english_errors(line, line_idx, issues);
    }
}

// 检查中文"的得地"用法
fn check_de_usage(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // 形容词+地+动词，如"快地跑"
    let de_di_regex =
        Regex::new(r"[快慢高低大小好坏强弱深浅厚薄粗细长短宽窄][的][跑走看听说读写做想吃喝]")
            .unwrap();
    for mat in de_di_regex.find_iter(line) {
        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: byte_to_char_index(line, mat.start() + 1),
            end: byte_to_char_index(line, mat.start() + 2),
            issue_type: "语法错误".to_string(),
            message: "形容词后接动词应使用'地'而非'的'".to_string(),
            suggestion: "将'的'改为'地'".to_string(),
        });
    }

    // 动词+得+形容词，如"跑得快"
    let de_de_regex =
        Regex::new(r"[跑走看听说读写做想吃喝][地][快慢高低大小好坏强弱深浅厚薄粗细长短宽窄]")
            .unwrap();
    for mat in de_de_regex.find_iter(line) {
        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: byte_to_char_index(line, mat.start() + 1),
            end: byte_to_char_index(line, mat.start() + 2),
            issue_type: "语法错误".to_string(),
            message: "动词后接形容词应使用'得'而非'地'".to_string(),
            suggestion: "将'地'改为'得'".to_string(),
        });
    }
}

// 检查中文常见病句
fn check_common_chinese_errors(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // 检查"把"字句缺少宾语
    if line.contains("把") {
        let ba_regex = Regex::new(r"把[^，。！？；：]*$").unwrap();
        if let Some(mat) = ba_regex.find(line) {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, mat.start()),
                end: byte_to_char_index(line, mat.end()),
                issue_type: "语法错误".to_string(),
                message: "'把'字句可能缺少宾语".to_string(),
                suggestion: "检查句子结构，确保'把'字后有完整的宾语和动作".to_string(),
            });
        }
    }

    // 检查"是...的"结构不当
    let shi_de_regex = Regex::new(r"是.*的").unwrap();
    if let Some(mat) = shi_de_regex.find(line) {
        let content = &line[mat.start()..mat.end()];
        if content.len() > 20 && !content.contains("，") && !content.contains("。") {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, mat.start()),
                end: byte_to_char_index(line, mat.end()),
                issue_type: "语法建议".to_string(),
                message: "'是...的'结构过长，可能影响阅读流畅度".to_string(),
                suggestion: "考虑拆分句子或重新组织句子结构".to_string(),
            });
        }
    }
}

// 检查英文主谓一致
fn check_subject_verb_agreement(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // 简单的主谓一致检查
    let singular_subjects = ["it", "he", "she", "this", "that"];
    let plural_verbs = ["are", "were", "have", "do"];

    for subject in singular_subjects.iter() {
        for verb in plural_verbs.iter() {
            let pattern = format!(r"\b{}\s+{}\b", subject, verb);
            let regex = Regex::new(&pattern).unwrap();

            if let Some(mat) = regex.find(line) {
                issues.push(TextIssue {
                    line_number: line_idx + 1,
                    start: byte_to_char_index(line, mat.start()),
                    end: byte_to_char_index(line, mat.end()),
                    issue_type: "语法错误".to_string(),
                    message: format!("主谓一致性错误: '{}' 与 '{}'", subject, verb),
                    suggestion: format!("对于单数主语 '{}' 应使用单数动词形式", subject),
                });
            }
        }
    }

    let plural_subjects = ["they", "we", "these", "those"];
    let singular_verbs = ["is", "was", "has", "does"];

    for subject in plural_subjects.iter() {
        for verb in singular_verbs.iter() {
            let pattern = format!(r"\b{}\s+{}\b", subject, verb);
            let regex = Regex::new(&pattern).unwrap();

            if let Some(mat) = regex.find(line) {
                issues.push(TextIssue {
                    line_number: line_idx + 1,
                    start: byte_to_char_index(line, mat.start()),
                    end: byte_to_char_index(line, mat.end()),
                    issue_type: "语法错误".to_string(),
                    message: format!("主谓一致性错误: '{}' 与 '{}'", subject, verb),
                    suggestion: format!("对于复数主语 '{}' 应使用复数动词形式", subject),
                });
            }
        }
    }
}

// 检查英文冠词使用
fn check_article_usage(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // 检查元音开头单词前的冠词
    let a_vowel_regex = Regex::new(r"\ba\s+[aeiouAEIOU]\w+\b").unwrap();
    if let Some(mat) = a_vowel_regex.find(line) {
        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: byte_to_char_index(line, mat.start()),
            end: byte_to_char_index(line, mat.start() + 1),
            issue_type: "冠词错误".to_string(),
            message: "元音开头的单词前应使用'an'而非'a'".to_string(),
            suggestion: "将'a'替换为'an'".to_string(),
        });
    }

    // 检查辅音开头单词前的冠词
    let an_consonant_regex =
        Regex::new(r"\ban\s+[bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ]\w+\b").unwrap();
    if let Some(mat) = an_consonant_regex.find(line) {
        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: byte_to_char_index(line, mat.start()),
            end: byte_to_char_index(line, mat.start() + 2),
            issue_type: "冠词错误".to_string(),
            message: "辅音开头的单词前应使用'a'而非'an'".to_string(),
            suggestion: "将'an'替换为'a'".to_string(),
        });
    }
}

// 检查英文常见语法错误
fn check_common_english_errors(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // 检查双重否定
    let double_negatives = [
        (r"\bdon't\s+have\s+no\b", "don't have any"),
        (r"\bcan't\s+hardly\b", "can hardly"),
        (r"\bwon't\s+be\s+no\b", "won't be any"),
        (r"\bdidn't\s+have\s+no\b", "didn't have any"),
        (r"\bwouldn't\s+never\b", "wouldn't ever"),
        (r"\bcouldn't\s+barely\b", "could barely"),
    ];

    for (pattern, suggestion) in double_negatives {
        let regex = Regex::new(pattern).unwrap();
        if let Some(mat) = regex.find(line) {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, mat.start()),
                end: byte_to_char_index(line, mat.end()),
                issue_type: "语法错误".to_string(),
                message: "检测到双重否定".to_string(),
                suggestion: format!("建议使用 '{}'", suggestion),
            });
        }
    }

    // 检查常见介词搭配错误
    let preposition_errors = [
        (r"\bdifferent\s+than\b", "different from"),
        (r"\bin\s+regards\s+to\b", "regarding"),
        (r"\bin\s+the\s+year\s+of\b", "in the year"),
        (r"\bregardless\s+to\b", "regardless of"),
        (r"\bsimilar\s+than\b", "similar to"),
        (r"\bsuperior\s+than\b", "superior to"),
    ];

    for (pattern, suggestion) in preposition_errors {
        let regex = Regex::new(pattern).unwrap();
        if let Some(mat) = regex.find(line) {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, mat.start()),
                end: byte_to_char_index(line, mat.end()),
                issue_type: "介词错误".to_string(),
                message: "不正确的介词用法".to_string(),
                suggestion: format!("建议使用 '{}'", suggestion),
            });
        }
    }
}

#[tauri::command]
fn read_file_content(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

// 自动检测文本语言
fn detect_language(text: &str) -> String {
    // 计算中文字符和英文字符的数量
    let mut chinese_count = 0;
    let mut english_count = 0;

    for c in text.chars() {
        if c >= '\u{4e00}' && c <= '\u{9fff}' {
            // 中文字符范围
            chinese_count += 1;
        } else if c.is_ascii_alphabetic() {
            // 英文字母
            english_count += 1;
        }
    }

    // 根据字符数量判断语言
    if chinese_count > english_count {
        "zh".to_string()
    } else {
        "en".to_string()
    }
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![analyze_text, read_file_content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
