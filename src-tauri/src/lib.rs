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
        // 检查行长度 - 只检查超过150个字符的行
        if line.len() > 150 {
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

        // 检查常见错别字
        check_common_typos(line, line_idx, &mut issues, language);

        // 检查语法问题
        check_grammar_issues(line, line_idx, &mut issues, language);
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
                                end: pos
                                    + be_verb.len()
                                    + after_be.find(next_word).unwrap_or(0)
                                    + next_word.len(),
                                issue_type: "Passive Voice".to_string(),
                                message: "Passive voice detected".to_string(),
                                suggestion: "Consider using active voice for stronger writing"
                                    .to_string(),
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
                start: pos,
                end: pos + phrase.len(),
                issue_type: "冗余表达".to_string(),
                message: format!("冗余表达: '{}'", phrase),
                suggestion: suggestion.to_string(),
            });
        }
    }
}

fn check_common_typos(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>, language: &str) {
    let typos: HashMap<&str, &str> = if language == "zh" {
        [
            ("的的", "的"),
            ("了了", "了"),
            ("是是", "是"),
            ("地地", "地"),
            ("得得", "得"),
            ("不不", "不"),
            ("和和", "和"),
            ("中中", "中"),
            ("有有", "有"),
            ("这这", "这"),
            ("那那", "那"),
            ("就就", "就"),
            ("在在", "在"),
            ("我我", "我"),
            ("你你", "你"),
            ("他他", "他"),
            ("她她", "她"),
            ("它它", "它"),
            ("们们", "们"),
            ("为为", "为"),
            ("以以", "以"),
            ("于于", "于"),
            ("对对", "对"),
            ("能能", "能"),
            ("会会", "会"),
            ("年年", "年"),
            ("月月", "月"),
            ("日日", "日"),
            ("时时", "时"),
            ("分分", "分"),
            ("秒秒", "秒"),
            ("天天", "天"),
            ("地地", "地"),
            ("人人", "人"),
            ("大大", "大"),
            ("小小", "小"),
            ("多多", "多"),
            ("少少", "少"),
            ("好好", "好"),
            ("坏坏", "坏"),
            ("高高", "高"),
            ("低低", "低"),
            ("长长", "长"),
            ("短短", "短"),
            ("粗粗", "粗"),
            ("细细", "细"),
            ("快快", "快"),
            ("慢慢", "慢"),
            ("新新", "新"),
            ("旧旧", "旧"),
            ("前前", "前"),
            ("后后", "后"),
            ("上上", "上"),
            ("下下", "下"),
            ("左左", "左"),
            ("右右", "右"),
            ("内内", "内"),
            ("外外", "外"),
            ("中中", "中"),
            ("东东", "东"),
            ("西西", "西"),
            ("南南", "南"),
            ("北北", "北"),
            ("春春", "春"),
            ("夏夏", "夏"),
            ("秋秋", "秋"),
            ("冬冬", "冬"),
            ("红红", "红"),
            ("黄黄", "黄"),
            ("蓝蓝", "蓝"),
            ("绿绿", "绿"),
            ("黑黑", "黑"),
            ("白白", "白"),
            ("灰灰", "灰"),
            ("紫紫", "紫"),
            ("金金", "金"),
            ("银银", "银"),
            ("铜铜", "铜"),
            ("铁铁", "铁"),
            ("水水", "水"),
            ("火火", "火"),
            ("木木", "木"),
            ("土土", "土"),
            ("金金", "金"),
            ("一一", "一"),
            ("二二", "二"),
            ("三三", "三"),
            ("四四", "四"),
            ("五五", "五"),
            ("六六", "六"),
            ("七七", "七"),
            ("八八", "八"),
            ("九九", "九"),
            ("十十", "十"),
            ("百百", "百"),
            ("千千", "千"),
            ("万万", "万"),
            ("亿亿", "亿"),
            ("零零", "零"),
            ("壹壹", "壹"),
            ("贰贰", "贰"),
            ("叁叁", "叁"),
            ("肆肆", "肆"),
            ("伍伍", "伍"),
            ("陆陆", "陆"),
            ("柒柒", "柒"),
            ("捌捌", "捌"),
            ("玖玖", "玖"),
            ("拾拾", "拾"),
            ("佰佰", "佰"),
            ("仟仟", "仟"),
            ("萬萬", "萬"),
            ("億億", "億"),
            ("零零", "零"),
        ]
        .iter()
        .cloned()
        .collect()
    } else {
        [
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
            ("acheive", "achieve"),
            ("aquire", "acquire"),
            ("apparant", "apparent"),
            ("arguement", "argument"),
            ("assasination", "assassination"),
            ("basicly", "basically"),
            ("begining", "beginning"),
            ("beleive", "believe"),
            ("buisness", "business"),
            ("calender", "calendar"),
            ("camoflage", "camouflage"),
            ("catagory", "category"),
            ("cemetary", "cemetery"),
            ("changable", "changeable"),
            ("cheif", "chief"),
            ("collegue", "colleague"),
            ("comming", "coming"),
            ("commitee", "committee"),
            ("completly", "completely"),
            ("conceed", "concede"),
            ("congradulate", "congratulate"),
            ("consciencious", "conscientious"),
            ("consious", "conscious"),
            ("curiousity", "curiosity"),
            ("definately", "definitely"),
            ("desireable", "desirable"),
            ("dissapear", "disappear"),
            ("dissapoint", "disappoint"),
            ("ecstacy", "ecstasy"),
            ("embarass", "embarrass"),
            ("enviroment", "environment"),
            ("equiped", "equipped"),
            ("exagerate", "exaggerate"),
            ("excede", "exceed"),
            ("existance", "existence"),
            ("experiance", "experience"),
            ("facinating", "fascinating"),
            ("familar", "familiar"),
            ("finaly", "finally"),
            ("flourescent", "fluorescent"),
            ("foriegn", "foreign"),
            ("freind", "friend"),
            ("fullfil", "fulfill"),
            ("guage", "gauge"),
            ("gaurd", "guard"),
            ("happend", "happened"),
            ("harras", "harass"),
            ("heighth", "height"),
            ("heirarchy", "hierarchy"),
            ("humerous", "humorous"),
            ("hygene", "hygiene"),
            ("hypocracy", "hypocrisy"),
            ("ignorence", "ignorance"),
            ("immediatly", "immediately"),
            ("incidently", "incidentally"),
            ("independant", "independent"),
            ("indispensible", "indispensable"),
            ("innoculate", "inoculate"),
            ("intellegent", "intelligent"),
            ("jewelery", "jewelry"),
            ("judgement", "judgment"),
            ("knowlege", "knowledge"),
            ("liason", "liaison"),
            ("libary", "library"),
            ("lisence", "license"),
            ("maintenence", "maintenance"),
            ("medeval", "medieval"),
            ("momento", "memento"),
            ("millenium", "millennium"),
            ("miniscule", "minuscule"),
            ("mischevious", "mischievous"),
            ("misspell", "misspell"),
            ("neccessary", "necessary"),
            ("neice", "niece"),
            ("nieghbor", "neighbor"),
            ("noticable", "noticeable"),
            ("occassion", "occasion"),
            ("occurance", "occurrence"),
            ("occured", "occurred"),
            ("ommision", "omission"),
            ("oppurtunity", "opportunity"),
            ("outragous", "outrageous"),
            ("parrallel", "parallel"),
            ("parliment", "parliament"),
            ("particurly", "particularly"),
            ("pasttime", "pastime"),
            ("percieve", "perceive"),
            ("persistant", "persistent"),
            ("personell", "personnel"),
            ("persue", "pursue"),
            ("posession", "possession"),
            ("potatos", "potatoes"),
            ("preceed", "precede"),
            ("predjudice", "prejudice"),
            ("presance", "presence"),
            ("privelege", "privilege"),
            ("probly", "probably"),
            ("pronounciation", "pronunciation"),
            ("prufe", "proof"),
            ("publically", "publicly"),
            ("quarentine", "quarantine"),
            ("questionaire", "questionnaire"),
            ("readible", "readable"),
            ("realy", "really"),
            ("recieve", "receive"),
            ("recomend", "recommend"),
            ("refered", "referred"),
            ("referance", "reference"),
            ("relevent", "relevant"),
            ("religous", "religious"),
            ("repitition", "repetition"),
            ("restarant", "restaurant"),
            ("rythm", "rhythm"),
            ("sacrafice", "sacrifice"),
            ("saftey", "safety"),
            ("secratary", "secretary"),
            ("seperate", "separate"),
            ("shedule", "schedule"),
            ("shoudl", "should"),
            ("similer", "similar"),
            ("sincerely", "sincerely"),
            ("speach", "speech"),
            ("succesful", "successful"),
            ("supercede", "supersede"),
            ("supposably", "supposedly"),
            ("suprise", "surprise"),
            ("temperture", "temperature"),
            ("tendancy", "tendency"),
            ("therefor", "therefore"),
            ("threshhold", "threshold"),
            ("tommorrow", "tomorrow"),
            ("tounge", "tongue"),
            ("truely", "truly"),
            ("unforseen", "unforeseen"),
            ("unfortunatly", "unfortunately"),
            ("untill", "until"),
            ("vacume", "vacuum"),
            ("vehical", "vehicle"),
            ("visious", "vicious"),
            ("wether", "whether"),
            ("wierd", "weird"),
            ("wellcome", "welcome"),
            ("whereever", "wherever"),
            ("wich", "which"),
            ("writting", "writing"),
        ]
        .iter()
        .cloned()
        .collect()
    };

    for (typo, correction) in typos {
        // 使用正则表达式匹配整个单词
        let pattern = if language == "zh" {
            format!(r"{}", typo)
        } else {
            format!(r"\b{}\b", typo)
        };

        let regex = Regex::new(&pattern).unwrap();

        for mat in regex.find_iter(line) {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: mat.start(),
                end: mat.end(),
                issue_type: "错别字".to_string(),
                message: format!("可能的错别字: '{}'", typo),
                suggestion: format!("建议修改为: '{}'", correction),
            });
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
            start: mat.start() + 1,
            end: mat.start() + 2,
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
            start: mat.start() + 1,
            end: mat.start() + 2,
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
                start: mat.start(),
                end: mat.end(),
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
                start: mat.start(),
                end: mat.end(),
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
                    start: mat.start(),
                    end: mat.end(),
                    issue_type: "Grammar Error".to_string(),
                    message: format!(
                        "Subject-verb agreement error: '{}' with '{}'",
                        subject, verb
                    ),
                    suggestion: format!("Use singular verb form with '{}'", subject),
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
                    start: mat.start(),
                    end: mat.end(),
                    issue_type: "Grammar Error".to_string(),
                    message: format!(
                        "Subject-verb agreement error: '{}' with '{}'",
                        subject, verb
                    ),
                    suggestion: format!("Use plural verb form with '{}'", subject),
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
            start: mat.start(),
            end: mat.start() + 1,
            issue_type: "Grammar Error".to_string(),
            message: "Use 'an' before words starting with vowel sounds".to_string(),
            suggestion: "Replace 'a' with 'an'".to_string(),
        });
    }

    // 检查辅音开头单词前的冠词
    let an_consonant_regex =
        Regex::new(r"\ban\s+[bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ]\w+\b").unwrap();
    if let Some(mat) = an_consonant_regex.find(line) {
        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: mat.start(),
            end: mat.start() + 2,
            issue_type: "Grammar Error".to_string(),
            message: "Use 'a' before words starting with consonant sounds".to_string(),
            suggestion: "Replace 'an' with 'a'".to_string(),
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
                start: mat.start(),
                end: mat.end(),
                issue_type: "Grammar Error".to_string(),
                message: "Double negative detected".to_string(),
                suggestion: format!("Consider using '{}'", suggestion),
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
                start: mat.start(),
                end: mat.end(),
                issue_type: "Grammar Error".to_string(),
                message: "Incorrect preposition usage".to_string(),
                suggestion: format!("Consider using '{}'", suggestion),
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
        .invoke_handler(tauri::generate_handler![analyze_text, read_file_content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
