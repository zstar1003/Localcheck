use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import our grammar checking modules
mod fix_functions;
mod grammar_check;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextIssue {
    line_number: usize,
    start: usize,
    end: usize,
    issue_type: String,
    message: String,
    suggestion: String,
}

// Convert byte index to character index
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

    // Calculate basic statistics
    let total_chars = text.chars().count();
    let total_words = text.split_whitespace().count();
    let total_lines = text.lines().count();

    stats.insert("total_chars".to_string(), total_chars);
    stats.insert("total_words".to_string(), total_words);
    stats.insert("total_lines".to_string(), total_lines);

    // Analyze each line
    for (line_idx, line) in text.lines().enumerate() {
        // Auto-detect language for the current line
        let line_language = detect_language(line);

        // Check for repeated words
        check_repeated_words(line, line_idx, &mut issues);

        // Check punctuation usage
        check_punctuation(line, line_idx, &mut issues);

        // Check passive voice (simplified)
        check_passive_voice(line, line_idx, &mut issues, &line_language);

        // Check redundant expressions
        check_redundant_expressions(line, line_idx, &mut issues, &line_language);

        // Check common typos
        check_common_typos(line, line_idx, &mut issues, &line_language);

        // Check grammar issues
        check_grammar_issues(line, line_idx, &mut issues, &line_language);

        // Check academic writing style
        fix_functions::check_academic_style(line, line_idx, &mut issues, &line_language);

        // Check sentence length
        fix_functions::check_sentence_length(line, line_idx, &mut issues, &line_language);

        // Check citation format
        fix_functions::check_citation_format(line, line_idx, &mut issues);
    }

    AnalysisResult { issues, stats }
}

fn check_repeated_words(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    let words: Vec<&str> = line.split_whitespace().collect();

    for i in 0..words.len().saturating_sub(1) {
        if words[i].len() > 3 && words[i] == words[i + 1] {
            // Find position of first word
            let first_word_pos = match line.find(words[i]) {
                Some(pos) => pos,
                None => continue, // Skip if word not found
            };

            // Find position of second word (starting after first word)
            let second_word_pos = match line[first_word_pos + words[i].len()..].find(words[i]) {
                Some(pos) => first_word_pos + words[i].len() + pos,
                None => continue, // Skip if second word not found
            };

            // Ensure only whitespace between words
            let between_text = &line[first_word_pos + words[i].len()..second_word_pos];
            if !between_text.trim().is_empty() {
                continue; // Skip if non-whitespace between words
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
    // Check for mixed Chinese and English punctuation
    // Use individual character checks instead of regex for Chinese punctuation
    let has_chinese_punct = line.contains('，')
        || line.contains('。')
        || line.contains('！')
        || line.contains('？')
        || line.contains('；')
        || line.contains('：');

    // Use a simpler regex for English punctuation to avoid escaping issues
    let en_punct_regex = match Regex::new(r"[,.!?;:]") {
        Ok(re) => re,
        Err(_) => return,
    };

    let has_english_punct = en_punct_regex.is_match(line);

    if has_chinese_punct && has_english_punct {
        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: 0,
            end: line.len(),
            issue_type: "标点混用".to_string(),
            message: "中英文标点符号混用".to_string(),
            suggestion: "请统一使用中文或英文标点符号".to_string(),
        });
    }

    // Check for consecutive punctuation
    let consecutive_punct_regex = match Regex::new(r"[,.!?;:]{2,}") {
        Ok(re) => re,
        Err(_) => return,
    };

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
        // Chinese passive voice detection (simplified)
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
        // English passive voice detection (simplified)
        let be_verbs = ["is", "are", "was", "were", "be", "been", "being"];
        let past_participles = ["ed", "en", "t"];

        for be_verb in be_verbs {
            if let Some(pos) = line.to_lowercase().find(be_verb) {
                // Simple check for past participle after be verb
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
    // Chinese repeated character detection
    if language == "zh" {
        // Detect consecutive repeated single characters
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len().saturating_sub(1) {
            if chars[i] == chars[i + 1] && chars[i] >= '\u{4e00}' && chars[i] <= '\u{9fff}' {
                // Chinese character repeated consecutively

                // Calculate byte position of character in original string
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

                i += 2; // Skip detected repeated characters
            } else {
                i += 1;
            }
        }
    } else {
        // English common typo detection - simplified list
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
        ]
        .iter()
        .cloned()
        .collect();

        for (typo, correction) in typos {
            // Use regex to match whole word
            let pattern = format!(r"\b{}\b", typo);
            let regex = match Regex::new(&pattern) {
                Ok(re) => re,
                Err(_) => continue, // Skip this pattern if regex creation fails
            };

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
        // Chinese grammar checks

        // Check "的得地" usage
        check_de_usage(line, line_idx, issues);

        // Check common Chinese errors
        check_common_chinese_errors(line, line_idx, issues);

        // Check measure word usage
        check_measure_word_usage(line, line_idx, issues);

        // Check idiom usage
        fix_functions::check_idiom_usage(line, line_idx, issues);

        // Check word order issues
        grammar_check::check_word_order(line, line_idx, issues);

        // Check punctuation usage
        grammar_check::check_chinese_punctuation(line, line_idx, issues);
    } else {
        // English grammar checks

        // Check subject-verb agreement
        check_subject_verb_agreement(line, line_idx, issues);

        // Check article usage
        check_article_usage(line, line_idx, issues);

        // Check tense consistency
        grammar_check::check_tense_consistency(line, line_idx, issues);

        // Check preposition usage
        grammar_check::check_preposition_usage(line, line_idx, issues);

        // Check common English errors
        check_common_english_errors(line, line_idx, issues);
    }
}

// Check Chinese measure word usage
fn check_measure_word_usage(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Incorrect measure word pairs - simplified list
    let measure_word_pairs = [
        (r"一只.{0,2}(桌子|椅子|床|柜子)", "一张"),
        (r"一张.{0,2}(狗|猫|鸟|鱼|老虎)", "一只"),
        (r"一个.{0,2}(报纸|杂志|书|地图)", "一份"),
        (r"一条.{0,2}(裤子|裙子)", "一条"),
        (r"一件.{0,2}(裤子|裙子)", "一条"),
    ];

    for (pattern, suggestion) in measure_word_pairs {
        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(_) => continue, // Skip this pattern if regex creation fails
        };

        if let Some(mat) = regex.find(line) {
            let matched_text = &line[mat.start()..mat.end()];
            let wrong_measure = &matched_text[0..2]; // Extract incorrect measure word

            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, mat.start()),
                end: byte_to_char_index(line, mat.start() + 2),
                issue_type: "量词搭配".to_string(),
                message: format!("量词搭配不当: '{}'", matched_text),
                suggestion: format!("建议使用: '{}' 替换 '{}'", suggestion, wrong_measure),
            });
        }
    }
}

// Check Chinese "的得地" usage
fn check_de_usage(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Adjective + "地" + verb, like "快地跑"
    let de_di_regex =
        match Regex::new(r"[快慢高低大小好坏强弱深浅厚薄粗细长短宽窄][的][跑走看听说读写做想吃喝]")
        {
            Ok(re) => re,
            Err(_) => return, // Return early if regex creation fails
        };

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

    // Verb + "得" + adjective, like "跑得快"
    let de_de_regex =
        match Regex::new(r"[跑走看听说读写做想吃喝][地][快慢高低大小好坏强弱深浅厚薄粗细长短宽窄]")
        {
            Ok(re) => re,
            Err(_) => return,
        };

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

    // Noun + "的" + noun, like "我的书"
    let de_de_regex = match Regex::new(r"[我你他她它们][得地][书包车房子]") {
        Ok(re) => re,
        Err(_) => return,
    };

    for mat in de_de_regex.find_iter(line) {
        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: byte_to_char_index(line, mat.start() + 1),
            end: byte_to_char_index(line, mat.start() + 2),
            issue_type: "语法错误".to_string(),
            message: "名词之间的所属关系应使用'的'而非'得'或'地'".to_string(),
            suggestion: "将'得'或'地'改为'的'".to_string(),
        });
    }
}

// Check common Chinese errors
fn check_common_chinese_errors(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Check "把" sentence missing object
    if line.contains("把") {
        let ba_regex = match Regex::new(r"把[^，。！？；：]*$") {
            Ok(re) => re,
            Err(_) => return, // Return early if regex creation fails
        };

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

    // Check improper "是...的" structure
    let shi_de_regex = match Regex::new(r"是.*的") {
        Ok(re) => re,
        Err(_) => return,
    };

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

// Check English subject-verb agreement
fn check_subject_verb_agreement(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Simple subject-verb agreement check
    let singular_subjects = ["it", "he", "she", "this", "that"];
    let plural_verbs = ["are", "were", "have", "do"];

    for subject in singular_subjects.iter() {
        for verb in plural_verbs.iter() {
            let pattern = format!(r"\b{}\s+{}\b", subject, verb);
            let regex = match Regex::new(&pattern) {
                Ok(re) => re,
                Err(_) => continue, // Skip this pattern if regex creation fails
            };

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
            let regex = match Regex::new(&pattern) {
                Ok(re) => re,
                Err(_) => continue, // Skip this pattern if regex creation fails
            };

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

// Check English article usage
fn check_article_usage(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Check article before vowel-starting words
    let a_vowel_regex = match Regex::new(r"\ba\s+[aeiouAEIOU]\w+\b") {
        Ok(re) => re,
        Err(_) => return, // Return early if regex creation fails
    };

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

    // Check article before consonant-starting words
    let an_consonant_regex =
        match Regex::new(r"\ban\s+[bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ]\w+\b") {
            Ok(re) => re,
            Err(_) => return,
        };

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

// Check common English grammar errors
fn check_common_english_errors(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Check double negatives
    let double_negatives = [
        (r"\bdon't\s+have\s+no\b", "don't have any"),
        (r"\bcan't\s+hardly\b", "can hardly"),
        (r"\bwon't\s+be\s+no\b", "won't be any"),
    ];

    for (pattern, suggestion) in double_negatives {
        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(_) => continue, // Skip this pattern if regex creation fails
        };

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

    // Check common preposition errors
    let preposition_errors = [
        (r"\bdifferent\s+than\b", "different from"),
        (r"\bin\s+regards\s+to\b", "regarding"),
        (r"\bregardless\s+to\b", "regardless of"),
    ];

    for (pattern, suggestion) in preposition_errors {
        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(_) => continue, // Skip this pattern if regex creation fails
        };

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

// Auto-detect text language
fn detect_language(text: &str) -> String {
    // Count Chinese and English characters
    let mut chinese_count = 0;
    let mut english_count = 0;

    for c in text.chars() {
        if c >= '\u{4e00}' && c <= '\u{9fff}' {
            // Chinese character range
            chinese_count += 1;
        } else if c.is_ascii_alphabetic() {
            // English letters
            english_count += 1;
        }
    }

    // Determine language based on character count
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
