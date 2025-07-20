use encoding_rs::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use zip::ZipArchive;

/// 解析不同格式的文档文件
pub fn parse_document(file_path: &str) -> Result<String, String> {
    let path = Path::new(file_path);

    // 获取文件扩展名
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "docx" => parse_docx(file_path),
        "doc" => parse_doc(file_path),
        "txt" | "md" => parse_text_file(file_path),
        _ => parse_text_file(file_path), // 默认尝试作为文本文件解析
    }
}

/// 解析DOCX文件（Office Open XML格式）
fn parse_docx(file_path: &str) -> Result<String, String> {
    let file = File::open(file_path).map_err(|e| format!("无法打开文件: {}", e))?;

    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader).map_err(|e| format!("无法解析DOCX文件: {}", e))?;

    // 查找document.xml文件
    let mut document_xml = archive
        .by_name("word/document.xml")
        .map_err(|e| format!("无法找到文档内容: {}", e))?;

    let mut xml_content = String::new();
    document_xml
        .read_to_string(&mut xml_content)
        .map_err(|e| format!("无法读取文档内容: {}", e))?;

    // 解析XML并提取文本
    extract_text_from_docx_xml(&xml_content)
}

/// 从DOCX的XML内容中提取纯文本
fn extract_text_from_docx_xml(xml_content: &str) -> Result<String, String> {
    let mut reader = Reader::from_str(xml_content);
    reader.trim_text(true);

    let mut text_content = String::new();
    let mut buf = Vec::new();
    let mut in_text_element = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"w:t" => in_text_element = true,
                    b"w:p" => {
                        // 段落开始，添加换行（如果不是第一段）
                        if !text_content.is_empty() {
                            text_content.push('\n');
                        }
                    }
                    b"w:br" => {
                        // 换行符
                        text_content.push('\n');
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name().as_ref() == b"w:t" {
                    in_text_element = false;
                }
            }
            Ok(Event::Text(e)) => {
                if in_text_element {
                    let text = e.unescape().map_err(|e| format!("XML解析错误: {}", e))?;
                    text_content.push_str(&text);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML解析错误: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(text_content)
}

/// 解析DOC文件（旧版Word格式）
fn parse_doc(file_path: &str) -> Result<String, String> {
    // DOC文件是复杂的二进制格式，这里提供一个简单的实现
    // 实际应用中可能需要更专业的库如python-docx的Rust等价物

    let mut file = File::open(file_path).map_err(|e| format!("无法打开DOC文件: {}", e))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("无法读取DOC文件: {}", e))?;

    // 尝试检测编码并提取可读文本
    // 这是一个简化的实现，可能不能处理所有DOC文件
    let text = extract_text_from_binary(&buffer);

    if text.trim().is_empty() {
        return Err("无法从DOC文件中提取文本内容。建议将文件另存为DOCX格式或TXT格式。".to_string());
    }

    Ok(text)
}

/// 从二进制数据中提取可能的文本内容
fn extract_text_from_binary(data: &[u8]) -> String {
    let mut text = String::new();
    let mut current_word = Vec::new();

    for &byte in data {
        if byte.is_ascii_graphic()
            || byte == b' '
            || byte == b'\n'
            || byte == b'\r'
            || byte == b'\t'
        {
            current_word.push(byte);
        } else {
            if !current_word.is_empty() {
                if let Ok(word) = String::from_utf8(current_word.clone()) {
                    // 只保留看起来像有意义的文本片段
                    if word.len() > 2 && word.chars().any(|c| c.is_alphabetic()) {
                        text.push_str(&word);
                        text.push(' ');
                    }
                }
                current_word.clear();
            }
        }
    }

    // 处理最后的单词
    if !current_word.is_empty() {
        if let Ok(word) = String::from_utf8(current_word) {
            if word.len() > 2 && word.chars().any(|c| c.is_alphabetic()) {
                text.push_str(&word);
            }
        }
    }

    // 清理文本：移除多余的空格和换行
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// 解析纯文本文件，支持多种编码
fn parse_text_file(file_path: &str) -> Result<String, String> {
    let mut file = File::open(file_path).map_err(|e| format!("无法打开文件: {}", e))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("无法读取文件: {}", e))?;

    // 尝试不同的编码
    let encodings = [UTF_8, GBK, GB18030, UTF_16LE, UTF_16BE];

    for encoding in &encodings {
        let (decoded, _, had_errors) = encoding.decode(&buffer);
        if !had_errors {
            return Ok(decoded.into_owned());
        }
    }

    // 如果所有编码都失败，尝试UTF-8并忽略错误
    let (decoded, _, _) = UTF_8.decode(&buffer);
    Ok(decoded.into_owned())
}

/// 检测文件类型
pub fn detect_file_type(file_path: &str) -> String {
    let path = Path::new(file_path);
    path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_lowercase()
}
