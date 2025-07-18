use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// 导入拼写检查模块
mod dictionary;
mod fix_functions;
mod grammar_check;
mod improved_checker;
mod spelling_dict;
mod title_checker;

// Import our gr text processing limits
const MAX_TEXT_LENGTH: usize = 50_000; // Maximum text length to process at once
const MAX_LINE_LENGTH: usize = 500; // Maximum line length to process
const MAX_ISSUES: usize = 500; // Maximum number of issues to return
const MAX_FILE_SIZE: u64 = 5_000_000; // Maximum file size (5MB)

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
    truncated: bool,
}

#[tauri::command]
fn analyze_text(text: &str) -> AnalysisResult {
    let mut issues = Vec::new();
    let mut stats = HashMap::new();
    let mut truncated = false;

    // Limit text size to prevent crashes
    let text = if text.len() > MAX_TEXT_LENGTH {
        truncated = true;
        &text[0..MAX_TEXT_LENGTH]
    } else {
        text
    };

    // Calculate basic statistics
    let total_chars = text.chars().count();
    let total_words = text.split_whitespace().count();
    let total_lines = text.lines().count();

    stats.insert("total_chars".to_string(), total_chars);
    stats.insert("total_words".to_string(), total_words);
    stats.insert("total_lines".to_string(), total_lines);

    // Process text in smaller chunks to avoid memory issues
    process_text_chunk(text, 0, &mut issues, &mut truncated);

    // Limit the number of issues returned
    if issues.len() > MAX_ISSUES {
        issues.truncate(MAX_ISSUES);
        truncated = true;
    }

    AnalysisResult {
        issues,
        stats,
        truncated,
    }
}

// 批量拼写检查命令
#[tauri::command]
fn batch_spell_check(text: &str) -> AnalysisResult {
    let mut issues = Vec::new();
    let mut stats = HashMap::new();
    let mut truncated = false;

    // Limit text size to prevent crashes
    let text = if text.len() > MAX_TEXT_LENGTH {
        truncated = true;
        &text[0..MAX_TEXT_LENGTH]
    } else {
        text
    };

    // Calculate basic statistics
    let total_chars = text.chars().count();
    let total_words = text.split_whitespace().count();
    let total_lines = text.lines().count();

    stats.insert("total_chars".to_string(), total_chars);
    stats.insert("total_words".to_string(), total_words);
    stats.insert("total_lines".to_string(), total_lines);

    // 使用批量拼写检查函数
    let spelling_errors = spelling_dict::check_text_spelling(text);

    // 将拼写错误转换为TextIssue格式
    for (wrong_word, correction, line_idx, pos) in spelling_errors {
        if issues.len() >= MAX_ISSUES {
            truncated = true;
            break;
        }

        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: pos,
            end: pos + wrong_word.len(),
            issue_type: "拼写错误".to_string(),
            message: format!("可能的拼写错误: '{}'", wrong_word),
            suggestion: format!("建议修改为: '{}'", correction),
        });
    }

    AnalysisResult {
        issues,
        stats,
        truncated,
    }
}

// Process a chunk of text
fn process_text_chunk(
    text: &str,
    start_line: usize,
    issues: &mut Vec<TextIssue>,
    truncated: &mut bool,
) {
    // Analyze each line
    for (rel_line_idx, line) in text.lines().enumerate() {
        let line_idx = start_line + rel_line_idx;

        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Limit line length to prevent excessive processing
        let line = if line.len() > MAX_LINE_LENGTH {
            *truncated = true;
            &line[0..MAX_LINE_LENGTH]
        } else {
            line
        };

        // Stop if we've found too many issues
        if issues.len() >= MAX_ISSUES {
            *truncated = true;
            break;
        }

        // Auto-detect language for the current line
        let line_language = detect_language(line);

        // Check for repeated words
        check_repeated_words(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        // Check punctuation usage
        check_punctuation(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        // 被动语态检查已禁用
        // check_passive_voice(line, line_idx, issues, &line_language);
        // if issues.len() >= MAX_ISSUES {
        //     break;
        // }

        // Check redundant expressions
        check_redundant_expressions(line, line_idx, issues, &line_language);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        // 使用改进的拼写检查器，解决单词切分不当和重复提示的问题
        improved_checker::check_spelling(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        // 检查常见拼写错误
        check_common_typos(line, line_idx, issues, &line_language);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        // 使用标题检查器检查标题中的拼写错误
        title_checker::check_title_spelling(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        // Check grammar issues
        check_grammar_issues(line, line_idx, issues, &line_language);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        // 使用语法检查模块
        grammar_check::check_word_order(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        grammar_check::check_chinese_punctuation(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        grammar_check::check_tense_consistency(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        grammar_check::check_preposition_usage(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        // 使用修复函数模块
        fix_functions::check_idiom_usage(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        fix_functions::check_academic_style(line, line_idx, issues, &line_language);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        fix_functions::check_sentence_length(line, line_idx, issues, &line_language);
        if issues.len() >= MAX_ISSUES {
            break;
        }

        fix_functions::check_citation_format(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            break;
        }
    }
}

fn check_repeated_words(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

    // 使用更简单的方法检测重复词
    let words: Vec<&str> = line.split_whitespace().collect();

    // 跟踪已经检测到的重复词，避免重复报告
    let mut detected_positions = Vec::new();

    for i in 0..words.len().saturating_sub(1) {
        // 跳过太短的词（少于4个字母的英文词或1个汉字）
        let min_length = if words[i].chars().any(|c| c >= '\u{4e00}' && c <= '\u{9fff}') {
            1 // 中文词至少1个字
        } else {
            4 // 英文词至少4个字母
        };

        if words[i].chars().count() < min_length {
            continue;
        }

        // 检查是否与下一个词相同
        if words[i] == words[i + 1] {
            // 找到第一个词的位置
            if let Some(first_word_pos) = find_whole_word(line, words[i]) {
                // 找到第二个词的位置（从第一个词之后开始查找）
                let after_first = &line[first_word_pos + words[i].len()..];
                if let Some(second_pos) = find_whole_word(after_first, words[i]) {
                    let second_word_pos = first_word_pos + words[i].len() + second_pos;

                    // 确保两个词之间只有空白字符
                    let between_text = &line[first_word_pos + words[i].len()..second_word_pos];
                    if between_text.trim().is_empty() {
                        // 检查是否已经检测到这个位置的重复词
                        let already_detected = detected_positions.iter().any(|&(start, end)| {
                            (first_word_pos >= start && first_word_pos < end)
                                || (second_word_pos >= start && second_word_pos < end)
                        });

                        if !already_detected {
                            issues.push(TextIssue {
                                line_number: line_idx + 1,
                                start: byte_to_char_index(line, first_word_pos),
                                end: byte_to_char_index(line, second_word_pos + words[i].len()),
                                issue_type: "重复词".to_string(),
                                message: format!("重复使用词语 '{}'", words[i]),
                                suggestion: format!("删除重复的 '{}'", words[i]),
                            });

                            // 记录已检测的位置
                            detected_positions
                                .push((first_word_pos, second_word_pos + words[i].len()));

                            // Stop if we've found too many issues
                            if issues.len() >= MAX_ISSUES {
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}

// 查找完整单词的位置，确保不会匹配到单词的一部分
fn find_whole_word(text: &str, word: &str) -> Option<usize> {
    let mut start_idx = 0;

    while let Some(pos) = text[start_idx..].find(word) {
        let actual_pos = start_idx + pos;

        // 检查单词前后是否是单词边界（空格、标点符号等）
        let is_start_boundary = actual_pos == 0
            || !text
                .chars()
                .nth(actual_pos - 1)
                .map_or(false, |c| c.is_alphanumeric());

        let is_end_boundary = actual_pos + word.len() >= text.len()
            || !text
                .chars()
                .nth(actual_pos + word.len())
                .map_or(false, |c| c.is_alphanumeric());

        if is_start_boundary && is_end_boundary {
            return Some(actual_pos);
        }

        // 继续查找下一个匹配
        start_idx = actual_pos + 1;
    }

    None
}

fn check_punctuation(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

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

        // Stop if we've found too many issues
        if issues.len() >= MAX_ISSUES {
            return;
        }
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

#[allow(dead_code)]
fn check_passive_voice(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>, language: &str) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

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

                // Stop if we've found too many issues
                if issues.len() >= MAX_ISSUES {
                    return;
                }
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

                            // Stop if we've found too many issues
                            if issues.len() >= MAX_ISSUES {
                                return;
                            }
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
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

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

            // Stop if we've found too many issues
            if issues.len() >= MAX_ISSUES {
                return;
            }
        }
    }
}

fn check_common_typos(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>, language: &str) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

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

                // Stop if we've found too many issues
                if issues.len() >= MAX_ISSUES {
                    return;
                }
            } else {
                i += 1;
            }
        }
    } else {
        // 使用我们的拼写检查字典进行更全面的拼写检查
        // 将行分割成单词并进行处理
        let words: Vec<&str> = line
            .split(|c: char| !c.is_alphanumeric() && c != '\'')
            .map(|w| w.trim())
            .filter(|w| !w.is_empty())
            .collect();

        for word in words {
            // 跳过太短的单词和纯数字
            if word.len() <= 2 || word.chars().all(|c| c.is_numeric()) {
                continue;
            }

            // 清理单词，去除可能的标点符号
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'');
            if clean_word.is_empty() {
                continue;
            }

            // 检查单词是否在拼写错误字典中
            if let Some(correction) = spelling_dict::check_word_spelling(clean_word) {
                // 找到单词在原始行中的位置
                if let Some(pos) = line.find(clean_word) {
                    issues.push(TextIssue {
                        line_number: line_idx + 1,
                        start: byte_to_char_index(line, pos),
                        end: byte_to_char_index(line, pos + clean_word.len()),
                        issue_type: "拼写错误".to_string(),
                        message: format!("可能的拼写错误: '{}'", clean_word),
                        suggestion: format!("建议修改为: '{}'", correction),
                    });

                    // Stop if we've found too many issues
                    if issues.len() >= MAX_ISSUES {
                        return;
                    }
                }
            }
        }

        // 特别检查标题和专有名词中的拼写错误
        // 这对于检测您提供的示例特别有用
        let typos: HashMap<&str, &str> = [
            // 常见拼写错误
            ("teh", "the"),
            ("recieve", "receive"),
            ("wierd", "weird"),
            ("alot", "a lot"),
            ("definately", "definitely"),
            ("seperate", "separate"),
            ("occured", "occurred"),
            ("accomodate", "accommodate"),
            ("adress", "address"),
            ("advertisment", "advertisement"),
            ("agressive", "aggressive"),
            ("apparant", "apparent"),
            ("appearence", "appearance"),
            ("arguement", "argument"),
            ("assasination", "assassination"),
            ("basicly", "basically"),
            ("begining", "beginning"),
            ("beleive", "believe"),
            ("belive", "believe"),
            ("buisness", "business"),
            ("calender", "calendar"),
            ("catagory", "category"),
            ("cemetary", "cemetery"),
            ("changable", "changeable"),
            ("cheif", "chief"),
            ("collegue", "colleague"),
            ("comming", "coming"),
            ("commitee", "committee"),
            ("completly", "completely"),
            ("concious", "conscious"),
            ("curiousity", "curiosity"),
            ("decieve", "deceive"),
            ("definate", "definite"),
            ("definitly", "definitely"),
            ("dissapoint", "disappoint"),
            ("embarass", "embarrass"),
            ("enviroment", "environment"),
            ("existance", "existence"),
            ("experiance", "experience"),
            ("familliar", "familiar"),
            ("finaly", "finally"),
            ("foriegn", "foreign"),
            ("freind", "friend"),
            ("goverment", "government"),
            ("gaurd", "guard"),
            ("happend", "happened"),
            ("harrass", "harass"),
            ("hieght", "height"),
            ("immediatly", "immediately"),
            ("independant", "independent"),
            ("interupt", "interrupt"),
            ("irrelevent", "irrelevant"),
            ("knowlege", "knowledge"),
            ("liason", "liaison"),
            ("libary", "library"),
            ("lisence", "license"),
            ("maintainance", "maintenance"),
            ("managment", "management"),
            ("medecine", "medicine"),
            ("millenium", "millennium"),
            ("miniscule", "minuscule"),
            ("mispell", "misspell"),
            ("neccessary", "necessary"),
            ("negociate", "negotiate"),
            ("nieghbor", "neighbor"),
            ("noticable", "noticeable"),
            ("occassion", "occasion"),
            ("occassionally", "occasionally"),
            ("occurance", "occurrence"),
            ("ocurrance", "occurrence"),
            ("oppurtunity", "opportunity"),
            ("persistant", "persistent"),
            ("posession", "possession"),
            ("prefered", "preferred"),
            ("presance", "presence"),
            ("propoganda", "propaganda"),
            ("publically", "publicly"),
            ("realy", "really"),
            ("reccomend", "recommend"),
            ("recieve", "receive"),
            ("refered", "referred"),
            ("relevent", "relevant"),
            ("religous", "religious"),
            ("remeber", "remember"),
            ("repitition", "repetition"),
            ("rythm", "rhythm"),
            ("secratary", "secretary"),
            ("sieze", "seize"),
            ("similer", "similar"),
            ("sincerely", "sincerely"),
            ("speach", "speech"),
            ("succesful", "successful"),
            ("supercede", "supersede"),
            ("supress", "suppress"),
            ("suprise", "surprise"),
            ("temperture", "temperature"),
            ("tendancy", "tendency"),
            ("therefor", "therefore"),
            ("threshhold", "threshold"),
            ("tommorrow", "tomorrow"),
            ("tounge", "tongue"),
            ("truely", "truly"),
            ("twelth", "twelfth"),
            ("tyrany", "tyranny"),
            ("underate", "underrate"),
            ("untill", "until"),
            ("usally", "usually"),
            ("vaccuum", "vacuum"),
            ("vegtable", "vegetable"),
            ("vehical", "vehicle"),
            ("visable", "visible"),
            ("wether", "whether"),
            ("withhold", "withhold"),
            ("writting", "writing"),
            // 学术论文中常见错误
            ("enronment", "environment"),
            ("financal", "financial"),
            ("alocation", "allocation"),
            ("empincal", "empirical"),
            ("eydence", "evidence"),
            ("analyis", "analysis"),
            ("reseach", "research"),
            ("statisical", "statistical"),
            ("significiant", "significant"),
            ("hypothsis", "hypothesis"),
            ("methodolgy", "methodology"),
            ("framwork", "framework"),
            ("implmentation", "implementation"),
            ("exprimental", "experimental"),
            ("corelation", "correlation"),
            ("varibles", "variables"),
            ("efficency", "efficiency"),
            ("optimzation", "optimization"),
            ("algoritm", "algorithm"),
            ("proceedure", "procedure"),
            ("comparision", "comparison"),
            ("improvment", "improvement"),
            ("performace", "performance"),
            ("technolgoy", "technology"),
            ("inovation", "innovation"),
            ("developement", "development"),
            ("infomation", "information"),
            ("comunication", "communication"),
            ("straegy", "strategy"),
            ("competitve", "competitive"),
            ("advantge", "advantage"),
            ("sustainble", "sustainable"),
            ("organiztion", "organization"),
            ("managment", "management"),
            ("leadrship", "leadership"),
            ("corprate", "corporate"),
            ("enterprse", "enterprise"),
            ("industy", "industry"),
            ("manufactring", "manufacturing"),
            ("producton", "production"),
            ("distribtion", "distribution"),
            ("consumtion", "consumption"),
            ("econmic", "economic"),
            ("finacial", "financial"),
            ("investent", "investment"),
            ("markting", "marketing"),
            ("advertsing", "advertising"),
            ("behavor", "behavior"),
            ("psycholgy", "psychology"),
            ("sociolgy", "sociology"),
            ("politcal", "political"),
            ("governent", "government"),
            ("regultion", "regulation"),
            ("legisltion", "legislation"),
            ("interntional", "international"),
            ("globl", "global"),
            ("reginal", "regional"),
            ("natinal", "national"),
            ("popultion", "population"),
            ("demographc", "demographic"),
            ("geographc", "geographic"),
            ("environental", "environmental"),
            ("sustainbility", "sustainability"),
            ("resouces", "resources"),
            ("enery", "energy"),
            ("efficent", "efficient"),
            ("renewble", "renewable"),
            ("polluton", "pollution"),
            ("conservtion", "conservation"),
            ("biodivrsity", "biodiversity"),
            ("ecosytem", "ecosystem"),
            ("climte", "climate"),
            ("temperture", "temperature"),
            ("atmosphre", "atmosphere"),
            ("emisssions", "emissions"),
            ("carbbon", "carbon"),
            ("footprnt", "footprint"),
            ("sustainble", "sustainable"),
            ("developent", "development"),
            ("innovtion", "innovation"),
            ("technolgy", "technology"),
            ("digitl", "digital"),
            ("computr", "computer"),
            ("softwre", "software"),
            ("hardwre", "hardware"),
            ("netwrk", "network"),
            ("internnet", "internet"),
            ("databse", "database"),
            ("algoritm", "algorithm"),
            ("programing", "programming"),
            ("artifical", "artificial"),
            ("intellgence", "intelligence"),
            ("machne", "machine"),
            ("learnng", "learning"),
            ("robotcs", "robotics"),
            ("automtion", "automation"),
            ("virtal", "virtual"),
            ("realiy", "reality"),
            ("augmeted", "augmented"),
            ("simultion", "simulation"),
            ("modelng", "modeling"),
            ("predicton", "prediction"),
            ("forecsting", "forecasting"),
            ("optimzation", "optimization"),
            ("efficincy", "efficiency"),
            ("effectveness", "effectiveness"),
            ("performnce", "performance"),
            ("productvity", "productivity"),
            ("qualiy", "quality"),
            ("reliablity", "reliability"),
            ("validty", "validity"),
            ("accurcy", "accuracy"),
            ("precison", "precision"),
            ("measurment", "measurement"),
            ("evaluaton", "evaluation"),
            ("assessent", "assessment"),
            ("analyis", "analysis"),
            ("synthsis", "synthesis"),
            ("integrtion", "integration"),
            ("implementtion", "implementation"),
            ("executon", "execution"),
            ("operaton", "operation"),
            ("maintenace", "maintenance"),
            ("improvment", "improvement"),
            ("enhancment", "enhancement"),
            ("optimiztion", "optimization"),
            ("maximiztion", "maximization"),
            ("minimiztion", "minimization"),
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

                // Stop if we've found too many issues
                if issues.len() >= MAX_ISSUES {
                    return;
                }
            }
        }
    }
}

fn check_grammar_issues(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>, language: &str) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

    if language == "zh" {
        // Chinese grammar checks - simplified for performance
        // Only check the most important rules

        // Check "的得地" usage
        check_de_usage(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            return;
        }

        // Check common Chinese errors
        check_common_chinese_errors(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            return;
        }
    } else {
        // English grammar checks - simplified for performance
        // Only check the most important rules

        // Check subject-verb agreement
        check_subject_verb_agreement(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            return;
        }

        // Check article usage
        check_article_usage(line, line_idx, issues);
        if issues.len() >= MAX_ISSUES {
            return;
        }
    }
}

// Check Chinese "的得地" usage
fn check_de_usage(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

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

        // Stop if we've found too many issues
        if issues.len() >= MAX_ISSUES {
            return;
        }
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

        // Stop if we've found too many issues
        if issues.len() >= MAX_ISSUES {
            return;
        }
    }
}

// Check common Chinese errors
fn check_common_chinese_errors(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

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
}

// Check English subject-verb agreement
fn check_subject_verb_agreement(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

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

                // Stop if we've found too many issues
                if issues.len() >= MAX_ISSUES {
                    return;
                }
            }
        }
    }
}

// Check English article usage
fn check_article_usage(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

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
}

// Read file content with streaming approach for large files
#[tauri::command]
fn read_file_content(path: &str) -> Result<String, String> {
    // Check if file exists
    let path = Path::new(path);
    if !path.exists() {
        return Err(format!("文件不存在: {}", path.display()));
    }

    // Check file size
    let metadata = match std::fs::metadata(path) {
        Ok(meta) => meta,
        Err(e) => return Err(format!("无法读取文件元数据: {}", e)),
    };

    // Check if file is too large
    if metadata.len() > MAX_FILE_SIZE {
        return Err(format!(
            "文件过大，请选择小于{}MB的文件",
            MAX_FILE_SIZE / 1_000_000
        ));
    }

    // Read file content
    match std::fs::read_to_string(path) {
        Ok(content) => {
            // If content is too large, truncate it
            if content.len() > MAX_TEXT_LENGTH {
                let truncated = content[0..MAX_TEXT_LENGTH].to_string();
                Ok(truncated)
            } else {
                Ok(content)
            }
        }
        Err(e) => Err(format!("读取文件失败: {}", e)),
    }
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

// Process large file in chunks
#[tauri::command]
fn analyze_large_file(path: &str) -> Result<AnalysisResult, String> {
    // Check if file exists
    let path = Path::new(path);
    if !path.exists() {
        return Err(format!("文件不存在: {}", path.display()));
    }

    // Check file size
    let metadata = match std::fs::metadata(path) {
        Ok(meta) => meta,
        Err(e) => return Err(format!("无法读取文件元数据: {}", e)),
    };

    // Check if file is too large
    if metadata.len() > MAX_FILE_SIZE {
        return Err(format!(
            "文件过大，请选择小于{}MB的文件",
            MAX_FILE_SIZE / 1_000_000
        ));
    }

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("无法打开文件: {}", e)),
    };

    let reader = BufReader::new(file);
    let mut issues = Vec::new();
    let mut stats = HashMap::new();
    let mut truncated = false;

    // Count statistics
    let mut total_chars = 0;
    let mut total_words = 0;
    let mut total_lines = 0;

    // Process file in chunks
    let mut line_idx = 0;
    let mut chunk = String::new();
    let mut chunk_size = 0;

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                total_lines += 1;
                total_chars += line.chars().count();
                total_words += line.split_whitespace().count();

                chunk.push_str(&line);
                chunk.push('\n');
                chunk_size += line.len() + 1;

                // Process chunk when it reaches the limit
                if chunk_size >= MAX_TEXT_LENGTH / 10 || issues.len() >= MAX_ISSUES {
                    process_text_chunk(&chunk, line_idx, &mut issues, &mut truncated);
                    line_idx += chunk.lines().count();
                    chunk.clear();
                    chunk_size = 0;

                    // Stop if we've found too many issues
                    if issues.len() >= MAX_ISSUES {
                        truncated = true;
                        break;
                    }
                }
            }
            Err(e) => return Err(format!("读取文件行时出错: {}", e)),
        }
    }

    // Process remaining chunk
    if !chunk.is_empty() && issues.len() < MAX_ISSUES {
        process_text_chunk(&chunk, line_idx, &mut issues, &mut truncated);
    }

    // Update statistics
    stats.insert("total_chars".to_string(), total_chars);
    stats.insert("total_words".to_string(), total_words);
    stats.insert("total_lines".to_string(), total_lines);

    // Limit the number of issues returned
    if issues.len() > MAX_ISSUES {
        issues.truncate(MAX_ISSUES);
        truncated = true;
    }

    Ok(AnalysisResult {
        issues,
        stats,
        truncated,
    })
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            analyze_text,
            read_file_content,
            analyze_large_file,
            batch_spell_check
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
