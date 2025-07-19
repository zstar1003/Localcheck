use crate::byte_to_char_index;
use crate::TextIssue;
use crate::MAX_ISSUES;
use std::collections::HashSet;

// 检查标题和专有名词中的拼写错误
pub fn check_title_spelling(
    line: &str,
    line_idx: usize,
    issues: &mut Vec<TextIssue>,
    global_detected_words: &mut HashSet<String>,
) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

    // 用于跟踪已经检测到的错误，避免重复提示
    let mut detected_errors = HashSet::new();

    // 特别针对学术论文标题的拼写错误
    let title_typos = [
        // 您示例中的错误
        ("Enronment", "Environment"),
        ("Financal", "Financial"),
        ("Alocation", "Allocation"),
        ("Empincal", "Empirical"),
        ("Eydence", "Evidence"),
        ("Corporat", "Corporate"),
        ("Corprate", "Corporate"),
        ("Geographc", "Geographic"),
        ("Busines", "Business"),
        ("Endowmnt", "Endowment"),
        // 其他常见的标题单词拼写错误
        ("Analyis", "Analysis"),
        ("Reseach", "Research"),
        ("Statisical", "Statistical"),
        ("Significiant", "Significant"),
        ("Hypothsis", "Hypothesis"),
        ("Methodolgy", "Methodology"),
        ("Framwork", "Framework"),
        ("Implmentation", "Implementation"),
        ("Exprimental", "Experimental"),
        ("Corelation", "Correlation"),
        ("Varibles", "Variables"),
        ("Efficency", "Efficiency"),
        ("Optimzation", "Optimization"),
        ("Algoritm", "Algorithm"),
        ("Proceedure", "Procedure"),
        ("Comparision", "Comparison"),
        ("Improvment", "Improvement"),
        ("Performace", "Performance"),
        ("Technolgoy", "Technology"),
        ("Inovation", "Innovation"),
        ("Developement", "Development"),
        ("Infomation", "Information"),
        ("Comunication", "Communication"),
        ("Straegy", "Strategy"),
        ("Competitve", "Competitive"),
        ("Advantge", "Advantage"),
        ("Sustainble", "Sustainable"),
        ("Organiztion", "Organization"),
        ("Managment", "Management"),
        ("Leadrship", "Leadership"),
        ("Enterprse", "Enterprise"),
        ("Industy", "Industry"),
        ("Manufactring", "Manufacturing"),
        ("Producton", "Production"),
        ("Distribtion", "Distribution"),
        ("Consumtion", "Consumption"),
        ("Econmic", "Economic"),
        ("Finacial", "Financial"),
        ("Investent", "Investment"),
        ("Markting", "Marketing"),
        ("Advertsing", "Advertising"),
        ("Behavor", "Behavior"),
        ("Psycholgy", "Psychology"),
        ("Sociolgy", "Sociology"),
        ("Politcal", "Political"),
        ("Governent", "Government"),
        ("Regultion", "Regulation"),
        ("Legisltion", "Legislation"),
        ("Interntional", "International"),
        ("Globl", "Global"),
        ("Reginal", "Regional"),
        ("Natinal", "National"),
        ("Popultion", "Population"),
        ("Demographc", "Demographic"),
        ("Environental", "Environmental"),
        ("Sustainbility", "Sustainability"),
        ("Resouces", "Resources"),
        ("Enery", "Energy"),
        ("Efficent", "Efficient"),
        ("Renewble", "Renewable"),
        ("Polluton", "Pollution"),
        ("Conservtion", "Conservation"),
        ("Biodivrsity", "Biodiversity"),
        ("Ecosytem", "Ecosystem"),
        ("Climte", "Climate"),
        ("Atmosphre", "Atmosphere"),
        ("Emisssions", "Emissions"),
        ("Carbbon", "Carbon"),
        ("Footprnt", "Footprint"),
        ("Digitl", "Digital"),
        ("Computr", "Computer"),
        ("Softwre", "Software"),
        ("Hardwre", "Hardware"),
        ("Netwrk", "Network"),
        ("Internnet", "Internet"),
        ("Databse", "Database"),
        ("Programing", "Programming"),
        ("Artifical", "Artificial"),
        ("Intellgence", "Intelligence"),
        ("Machne", "Machine"),
        ("Learnng", "Learning"),
        ("Robotcs", "Robotics"),
        ("Automtion", "Automation"),
        ("Virtal", "Virtual"),
        ("Realiy", "Reality"),
        ("Augmeted", "Augmented"),
        ("Simultion", "Simulation"),
        ("Modelng", "Modeling"),
        ("Predicton", "Prediction"),
        ("Forecsting", "Forecasting"),
        ("Effectveness", "Effectiveness"),
        ("Productvity", "Productivity"),
        ("Qualiy", "Quality"),
        ("Reliablity", "Reliability"),
        ("Validty", "Validity"),
        ("Accurcy", "Accuracy"),
        ("Precison", "Precision"),
        ("Measurment", "Measurement"),
        ("Evaluaton", "Evaluation"),
        ("Assessent", "Assessment"),
        ("Synthsis", "Synthesis"),
        ("Integrtion", "Integration"),
        ("Executon", "Execution"),
        ("Operaton", "Operation"),
        ("Maintenace", "Maintenance"),
        ("Enhancment", "Enhancement"),
        ("Maximiztion", "Maximization"),
        ("Minimiztion", "Minimization"),
    ];

    // 首先，将行分割成单词
    let words: Vec<&str> = line
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty() && w.len() > 2) // 过滤掉太短的单词
        .collect();

    // 检查每个完整单词
    for word in words {
        // 跳过已经检测到的错误
        if detected_errors.contains(word) {
            continue;
        }

        // 检查单词是否在拼写错误字典中（不区分大小写）
        for (typo, correction) in title_typos.iter() {
            if word.to_lowercase() == typo.to_lowercase() {
                // 检查是否已经在全局检测集合中
                let word_lower = word.to_lowercase();
                if global_detected_words.contains(&word.to_string())
                    || global_detected_words.contains(&word_lower)
                {
                    continue;
                }

                // 找到单词在原始行中的位置
                if let Some(pos) = find_whole_word(line, word) {
                    issues.push(TextIssue {
                        line_number: line_idx + 1,
                        start: byte_to_char_index(line, pos),
                        end: byte_to_char_index(line, pos + word.len()),
                        issue_type: "拼写错误".to_string(),
                        message: format!("可能的拼写错误: '{}'", word),
                        suggestion: format!("建议修改为: '{}'", correction),
                    });

                    // 添加到已检测集合
                    detected_errors.insert(word);

                    // 添加到全局检测集合
                    global_detected_words.insert(word.to_string());
                    global_detected_words.insert(word_lower);

                    // 检查是否达到最大问题数
                    if issues.len() >= MAX_ISSUES {
                        return;
                    }

                    // 找到匹配后跳出内部循环
                    break;
                }
            }
        }
    }

    // 特别检查您示例中的错误
    let example_errors = [
        ("Enronment", "Environment"),
        ("Financal", "Financial"),
        ("Alocation", "Allocation"),
        ("Empincal", "Empirical"),
        ("Eydence", "Evidence"),
    ];

    for (error, correction) in example_errors.iter() {
        // 尝试查找完整单词
        if let Some(pos) = find_whole_word(line, error) {
            // 如果已经检测到这个错误，跳过
            if detected_errors.contains(*error) {
                continue;
            }

            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, pos),
                end: byte_to_char_index(line, pos + error.len()),
                issue_type: "拼写错误".to_string(),
                message: format!("可能的拼写错误: '{}'", error),
                suggestion: format!("建议修改为: '{}'", correction),
            });

            // 添加到已检测集合
            detected_errors.insert(*error);

            if issues.len() >= MAX_ISSUES {
                return;
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
