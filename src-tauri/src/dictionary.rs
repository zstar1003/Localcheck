use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::OnceLock;

// 使用 OnceLock 来实现单例模式，确保词典只被加载一次
static DICTIONARY: OnceLock<HashSet<String>> = OnceLock::new();

// 加载词典文件
pub fn load_dictionary() -> &'static HashSet<String> {
    DICTIONARY.get_or_init(|| {
        let mut words = HashSet::new();

        // 尝试从不同位置加载词典文件
        let paths = [
            "English.dic",             // 当前目录
            "./English.dic",           // 当前目录（显式）
            "../English.dic",          // 上级目录
            "../../English.dic",       // 上上级目录
            "./src-tauri/English.dic", // src-tauri 目录
            "./resources/English.dic", // resources 目录
            "./_up_/English.dic", // _up_目录
            "_up_/English.dic", // _up_目录
        ];

        for path in paths {
            if let Ok(dict) = read_dictionary_file(path) {
                words = dict;
                println!("成功加载词典文件: {}", path);
                break;
            }
        }

        // 如果没有找到词典文件，使用内置的常见单词列表
        if words.is_empty() {
            println!("未找到词典文件，使用内置的常见单词列表");
            for word in COMMON_WORDS {
                words.insert(word.to_lowercase());
            }
        }

        words
    })
}

// 从文件中读取词典
fn read_dictionary_file(path: &str) -> io::Result<HashSet<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut words = HashSet::new();

    // 跳过第一行（词条数量）
    let mut lines = reader.lines();
    let _ = lines.next();

    // 读取每一行，提取单词（去除词性标记）
    for line in lines {
        if let Ok(line) = line {
            // 提取单词部分（去除词性标记）
            if let Some(idx) = line.find('/') {
                let word = line[..idx].to_string();
                if !word.is_empty() {
                    // 添加原始单词
                    words.insert(word.to_lowercase());

                    // 如果单词是数字，跳过
                    if word.chars().all(|c| c.is_numeric()) {
                        continue;
                    }

                    // 添加常见的词形变化
                    let word_lower = word.to_lowercase();

                    // 添加复数形式
                    if !word_lower.ends_with('s') {
                        words.insert(format!("{}s", word_lower));
                    }

                    // 添加过去式和过去分词
                    if word_lower.ends_with('e') {
                        words.insert(format!("{}d", word_lower));
                    } else {
                        words.insert(format!("{}ed", word_lower));
                    }

                    // 添加现在分词
                    if word_lower.ends_with('e') {
                        words.insert(format!("{}ing", &word_lower[..word_lower.len() - 1]));
                    } else {
                        words.insert(format!("{}ing", word_lower));
                    }

                    // 添加形容词形式
                    if !word_lower.ends_with("al") {
                        words.insert(format!("{}al", word_lower));
                    }

                    // 添加副词形式
                    if !word_lower.ends_with("ly") {
                        words.insert(format!("{}ly", word_lower));
                    }
                }
            } else {
                // 如果没有词性标记，直接添加整行
                if !line.is_empty() {
                    words.insert(line.to_lowercase());
                }
            }
        }
    }

    // 添加特定的常见词形变化
    let common_words = [
        // 常见的带连字符的技术术语和图论术语
        "out-degree",
        "in-degree",
        "out-degrees",
        "in-degrees",
        "degree-centrality",
        "betweenness-centrality",
        "closeness-centrality",
        "eigenvector-centrality",
        "graph-based",
        "node-based",
        "edge-based",
        "path-based",
        "network-based",
        "directed-graph",
        "undirected-graph",
        "weighted-graph",
        "unweighted-graph",
        "strongly-connected",
        "weakly-connected",
        "fully-connected",
        "shortest-path",
        "longest-path",
        "critical-path",
        "minimum-spanning-tree",
        "maximum-flow",
        "minimum-cut",
        "breadth-first",
        "depth-first",
        "greedy-algorithm",
        "time-complexity",
        "space-complexity",
        "worst-case",
        "best-case",
        "average-case",
        "big-O",
        "big-Theta",
        "big-Omega",
        "data-structure",
        "data-structures",
        "data-type",
        "data-types",
        "hash-table",
        "hash-map",
        "linked-list",
        "binary-tree",
        "binary-search-tree",
        "red-black-tree",
        "b-tree",
        "heap-structure",
        "priority-queue",
        "dynamic-programming",
        "divide-and-conquer",
        "branch-and-bound",
        "machine-learning",
        "deep-learning",
        "neural-network",
        "decision-tree",
        "random-forest",
        "support-vector-machine",
        "k-means",
        "k-nearest-neighbors",
        "natural-language-processing",
        "computer-vision",
        "image-processing",
        "feature-extraction",
        "feature-selection",
        "feature-engineering",
        "cross-validation",
        "over-fitting",
        "under-fitting",
        "hyper-parameter",
        "gradient-descent",
        "back-propagation",
        "forward-propagation",
        "supervised-learning",
        "unsupervised-learning",
        "reinforcement-learning",
        "semi-supervised",
        "transfer-learning",
        "meta-learning",
        "in-memory",
        "on-disk",
        "in-place",
        "out-of-place",
        "pre-processing",
        "post-processing",
        "real-time-processing",
        "batch-processing",
        "stream-processing",
        "parallel-processing",
        "distributed-computing",
        "cloud-computing",
        "edge-computing",
        "fog-computing",
        "micro-service",
        "service-oriented",
        "event-driven",
        "message-driven",
        "fault-tolerant",
        "highly-available",
        "load-balanced",
        "auto-scaling",
        "version-control",
        "continuous-integration",
        "continuous-deployment",
        "test-driven",
        "behavior-driven",
        "domain-driven",
        "object-relational",
        "document-oriented",
        "key-value",
        "column-family",
        "time-series",
        "graph-database",
        "in-memory-database",
        "relational-database",
        "non-relational-database",
        "nosql-database",
        "sql-query",
        "no-sql",
        "new-sql",
        "cross-reference",
        "cross-platform",
        "cross-site",
        "self-contained",
        "self-reference",
        "self-organizing",
        "self-service",
        "well-known",
        "well-defined",
        "well-formed",
        "well-structured",
        "high-level",
        "low-level",
        "high-performance",
        "high-availability",
        "real-time",
        "run-time",
        "compile-time",
        "design-time",
        "build-time",
        "client-side",
        "server-side",
        "front-end",
        "back-end",
        "full-stack",
        "object-oriented",
        "service-oriented",
        "event-driven",
        "data-driven",
        "user-friendly",
        "mobile-friendly",
        "search-engine-friendly",
        "open-source",
        "closed-source",
        "multi-threaded",
        "single-threaded",
        "multi-core",
        "multi-process",
        "multi-user",
        "multi-tenant",
        "end-to-end",
        "peer-to-peer",
        "business-to-business",
        "business-to-consumer",
        "point-to-point",
        "one-to-many",
        "many-to-many",
        "one-to-one",
        "first-class",
        "second-class",
        "third-party",
        "first-party",
        "read-only",
        "write-only",
        "read-write",
        "non-blocking",
        "state-of-the-art",
        "cutting-edge",
        "mission-critical",
        // 金融术语
        "Asset",
        "ASSET",
        "Assets",
        "ASSETS",
        "asset",
        "assets",
        "Fund",
        "FUND",
        "Funds",
        "FUNDS",
        "fund",
        "funds",
        "Stock",
        "STOCK",
        "Stocks",
        "STOCKS",
        "stock",
        "stocks",
        "Bond",
        "BOND",
        "Bonds",
        "BONDS",
        "bond",
        "bonds",
        "Share",
        "SHARE",
        "Shares",
        "SHARES",
        "share",
        "shares",
        "Market",
        "MARKET",
        "Markets",
        "MARKETS",
        "market",
        "markets",
        "Investment",
        "INVESTMENT",
        "Investments",
        "INVESTMENTS",
        "investment",
        "investments",
        "Portfolio",
        "PORTFOLIO",
        "Portfolios",
        "PORTFOLIOS",
        "portfolio",
        "portfolios",
        "Capital",
        "CAPITAL",
        "Capitals",
        "CAPITALS",
        "capital",
        "capitals",
        "Equity",
        "EQUITY",
        "Equities",
        "EQUITIES",
        "equity",
        "equities",
        "Dividend",
        "DIVIDEND",
        "Dividends",
        "DIVIDENDS",
        "dividend",
        "dividends",
        "Revenue",
        "REVENUE",
        "Revenues",
        "REVENUES",
        "revenue",
        "revenues",
        "Profit",
        "PROFIT",
        "Profits",
        "PROFITS",
        "profit",
        "profits",
        "Loss",
        "LOSS",
        "Losses",
        "LOSSES",
        "loss",
        "losses",
        "Balance",
        "BALANCE",
        "Balances",
        "BALANCES",
        "balance",
        "balances",
        "Account",
        "ACCOUNT",
        "Accounts",
        "ACCOUNTS",
        "account",
        "accounts",
        "Transaction",
        "TRANSACTION",
        "Transactions",
        "TRANSACTIONS",
        "transaction",
        "transactions",
        "Payment",
        "PAYMENT",
        "Payments",
        "PAYMENTS",
        "payment",
        "payments",
        "Credit",
        "CREDIT",
        "Credits",
        "CREDITS",
        "credit",
        "credits",
        "Debit",
        "DEBIT",
        "Debits",
        "DEBITS",
        "debit",
        "debits",
        "Cash",
        "CASH",
        "cash",
        "Currency",
        "CURRENCY",
        "Currencies",
        "CURRENCIES",
        "currency",
        "currencies",
        "Exchange",
        "EXCHANGE",
        "Exchanges",
        "EXCHANGES",
        "exchange",
        "exchanges",
        "Rate",
        "RATE",
        "Rates",
        "RATES",
        "rate",
        "rates",
        "Interest",
        "INTEREST",
        "Interests",
        "INTERESTS",
        "interest",
        "interests",
        "Tax",
        "TAX",
        "Taxes",
        "TAXES",
        "tax",
        "taxes",
        "Budget",
        "BUDGET",
        "Budgets",
        "BUDGETS",
        "budget",
        "budgets",
        "Expense",
        "EXPENSE",
        "Expenses",
        "EXPENSES",
        "expense",
        "expenses",
        "Cost",
        "COST",
        "Costs",
        "COSTS",
        "cost",
        "costs",
        "Price",
        "PRICE",
        "Prices",
        "PRICES",
        "price",
        "prices",
        "Value",
        "VALUE",
        "Values",
        "VALUES",
        "value",
        "values",
        "Risk",
        "RISK",
        "Risks",
        "RISKS",
        "risk",
        "risks",
        "Return",
        "RETURN",
        "Returns",
        "RETURNS",
        "return",
        "returns",
        "Yield",
        "YIELD",
        "Yields",
        "YIELDS",
        "yield",
        "yields",
        "Volatility",
        "VOLATILITY",
        "volatility",
        "Liquidity",
        "LIQUIDITY",
        "liquidity",
        "Solvency",
        "SOLVENCY",
        "solvency",
        "Leverage",
        "LEVERAGE",
        "leverage",
        "Debt",
        "DEBT",
        "Debts",
        "DEBTS",
        "debt",
        "debts",
        "Liability",
        "LIABILITY",
        "Liabilities",
        "LIABILITIES",
        "liability",
        "liabilities",
        // 其他常见词形变化
        "relate",
        "related",
        "relation",
        "relations",
        "relationship",
        "relationships",
        "associate",
        "associated",
        "association",
        "associations",
        "connect",
        "connected",
        "connection",
        "connections",
        "integrate",
        "integrated",
        "integration",
        "automate",
        "automated",
        "automation",
        "dedicate",
        "dedicated",
        "dedication",
        "educate",
        "educated",
        "education",
        "complicate",
        "complicated",
        "complication",
        "motivate",
        "motivated",
        "motivation",
        "isolate",
        "isolated",
        "isolation",
        "locate",
        "located",
        "location",
        "estimate",
        "estimated",
        "estimation",
        "evaluate",
        "evaluated",
        "evaluation",
        "calculate",
        "calculated",
        "calculation",
        "illustrate",
        "illustrated",
        "illustration",
        "demonstrate",
        "demonstrated",
        "demonstration",
        "indicate",
        "indicated",
        "indication",
        "validate",
        "validated",
        "validation",
        "regulate",
        "regulated",
        "regulation",
        "simulate",
        "simulated",
        "simulation",
        "formulate",
        "formulated",
        "formulation",
        "populate",
        "populated",
        "population",
        "elevate",
        "elevated",
        "elevation",
        "cultivate",
        "cultivated",
        "cultivation",
        "initiate",
        "initiated",
        "initiation",
        "negotiate",
        "negotiated",
        "negotiation",
        "operate",
        "operated",
        "operation",
        "generate",
        "generated",
        "generation",
        "translate",
        "translated",
        "translation",
        "update",
        "updated",
        "updating",
        "create",
        "created",
        "creation",
        "limit",
        "limited",
        "limitation",
        "unite",
        "united",
        "unity",
        "excite",
        "excited",
        "excitement",
        "detail",
        "detailed",
        "details",
        "advance",
        "advanced",
        "advancement",
    ];

    for word in common_words {
        words.insert(word.to_string());
    }

    Ok(words)
}

// 检查单词是否在词典中，考虑常见的单词变形
pub fn is_word_in_dictionary(word: &str) -> bool {
    let dict = load_dictionary();

    // 保留原始大小写检查
    if dict.contains(word) {
        return true;
    }

    // 特殊处理常见的大写单词
    if word == "Asset" || word == "ASSET" || word == "Assets" || word == "ASSETS" {
        return true;
    }

    let word_lower = word.to_lowercase();

    // 直接检查单词是否在词典中
    if dict.contains(&word_lower) {
        return true;
    }

    // 检查单词的基本形式
    // 1. 去掉结尾的 's'（复数形式）
    if word_lower.ends_with('s') && word_lower.len() > 2 {
        let singular = &word_lower[..word_lower.len() - 1];
        if dict.contains(singular) {
            return true;
        }
    }

    // 2. 去掉结尾的 'es'（复数形式）
    if word_lower.ends_with("es") && word_lower.len() > 3 {
        let singular = &word_lower[..word_lower.len() - 2];
        if dict.contains(singular) {
            return true;
        }
    }

    // 3. 去掉结尾的 'ed'（过去式）
    if word_lower.ends_with("ed") && word_lower.len() > 3 {
        let base = &word_lower[..word_lower.len() - 2];
        if dict.contains(base) {
            return true;
        }

        // 处理双写辅音字母的情况，如 'stopped' -> 'stop'
        if word_lower.len() > 4 && base.ends_with(base.chars().last().unwrap()) {
            let base_single = &base[..base.len() - 1];
            if dict.contains(base_single) {
                return true;
            }
        }

        // 处理以 'e' 结尾的动词，如 'related' -> 'relate'
        let base_e = format!("{}e", base);
        if dict.contains(&base_e) {
            return true;
        }
    }

    // 4. 去掉结尾的 'ing'（现在分词）
    if word_lower.ends_with("ing") && word_lower.len() > 4 {
        let base = &word_lower[..word_lower.len() - 3];
        if dict.contains(base) {
            return true;
        }

        // 处理去掉 'e' 的情况，如 'making' -> 'make'
        let base_e = format!("{}e", base);
        if dict.contains(&base_e) {
            return true;
        }

        // 处理双写辅音字母的情况，如 'running' -> 'run'
        if word_lower.len() > 5 && base.ends_with(base.chars().last().unwrap()) {
            let base_single = &base[..base.len() - 1];
            if dict.contains(base_single) {
                return true;
            }
        }
    }

    // 5. 去掉结尾的 'ly'（副词）
    if word_lower.ends_with("ly") && word_lower.len() > 3 {
        let base = &word_lower[..word_lower.len() - 2];
        if dict.contains(base) {
            return true;
        }
    }

    // 6. 去掉结尾的 'er'（比较级）
    if word_lower.ends_with("er") && word_lower.len() > 3 {
        let base = &word_lower[..word_lower.len() - 2];
        if dict.contains(base) {
            return true;
        }
    }

    // 7. 去掉结尾的 'est'（最高级）
    if word_lower.ends_with("est") && word_lower.len() > 4 {
        let base = &word_lower[..word_lower.len() - 3];
        if dict.contains(base) {
            return true;
        }
    }

    // 8. 处理 'tion' 结尾的名词，如 'relation' -> 'relate'
    if word_lower.ends_with("tion") && word_lower.len() > 5 {
        let base = &word_lower[..word_lower.len() - 4];
        let base_e = format!("{}e", base);
        if dict.contains(&base_e) {
            return true;
        }
    }

    // 9. 处理 'ment' 结尾的名词，如 'development' -> 'develop'
    if word_lower.ends_with("ment") && word_lower.len() > 6 {
        let base = &word_lower[..word_lower.len() - 4];
        if dict.contains(base) {
            return true;
        }
    }

    // 10. 处理 'able'/'ible' 结尾的形容词，如 'readable' -> 'read'
    if (word_lower.ends_with("able") || word_lower.ends_with("ible")) && word_lower.len() > 5 {
        let base = &word_lower[..word_lower.len() - 4];
        if dict.contains(base) {
            return true;
        }

        // 处理去掉 'e' 的情况，如 'reliable' -> 'rely'
        if base.len() > 1 && base.ends_with('i') {
            let base_y = format!("{}y", &base[..base.len() - 1]);
            if dict.contains(&base_y) {
                return true;
            }
        }

        // 处理以 'e' 结尾的动词，如 'usable' -> 'use'
        let base_e = format!("{}e", base);
        if dict.contains(&base_e) {
            return true;
        }
    }

    // 11. 处理 'al' 结尾的形容词，如 'related' -> 'relate', 'functional' -> 'function'
    if word_lower.ends_with("al") && word_lower.len() > 4 {
        // 检查基本形式
        let base = &word_lower[..word_lower.len() - 2];
        if dict.contains(base) {
            return true;
        }

        // 处理 'tion' -> 'tional'，如 'functional' -> 'function'
        if base.ends_with("tion") {
            let function_base = &base[..base.len() - 3]; // 去掉 'ion'
            if dict.contains(function_base) {
                return true;
            }
        }

        // 处理以 'e' 结尾的词，如 'cultural' -> 'culture'
        let base_e = format!("{}e", base);
        if dict.contains(&base_e) {
            return true;
        }
    }

    // 12. 处理 'ive' 结尾的形容词，如 'productive' -> 'product'
    if word_lower.ends_with("ive") && word_lower.len() > 4 {
        let base = &word_lower[..word_lower.len() - 3];
        if dict.contains(base) {
            return true;
        }

        // 处理 'duct' -> 'ductive'，如 'productive' -> 'product'
        let base_e = format!("{}e", base);
        if dict.contains(&base_e) {
            return true;
        }

        // 处理 't' -> 'tive'，如 'relative' -> 'relate'
        if base.ends_with('t') {
            let alt_base = &base[..base.len() - 1];
            let alt_base_e = format!("{}e", alt_base);
            if dict.contains(&alt_base_e) {
                return true;
            }
        }
    }

    // 13. 处理 'ize'/'ise' 结尾的动词，如 'optimize' -> 'optimal'
    if (word_lower.ends_with("ize") || word_lower.ends_with("ise")) && word_lower.len() > 4 {
        let base = &word_lower[..word_lower.len() - 3];
        if dict.contains(base) {
            return true;
        }

        // 处理 'al' -> 'alize'，如 'formalize' -> 'formal'
        let base_al = format!("{}al", base);
        if dict.contains(&base_al) {
            return true;
        }
    }

    // 14. 处理 'ful' 结尾的形容词，如 'helpful' -> 'help'
    if word_lower.ends_with("ful") && word_lower.len() > 4 {
        let base = &word_lower[..word_lower.len() - 3];
        if dict.contains(base) {
            return true;
        }
    }

    // 15. 处理 'ity' 结尾的名词，如 'activity' -> 'active'
    if word_lower.ends_with("ity") && word_lower.len() > 4 {
        let base = &word_lower[..word_lower.len() - 3];

        // 处理 'ive' -> 'ivity'，如 'activity' -> 'active'
        let base_ive = format!("{}ive", base);
        if dict.contains(&base_ive) {
            return true;
        }

        // 处理 'al' -> 'ality'，如 'reality' -> 'real'
        let base_al = format!("{}al", base);
        if dict.contains(&base_al) {
            return true;
        }
    }

    // 16. 处理特定的常见形容词，如 "related"
    let common_adjectives = [
        ("related", "relate"),
        ("associated", "associate"),
        ("connected", "connect"),
        ("integrated", "integrate"),
        ("automated", "automate"),
        ("dedicated", "dedicate"),
        ("educated", "educate"),
        ("complicated", "complicate"),
        ("motivated", "motivate"),
        ("isolated", "isolate"),
        ("located", "locate"),
        ("estimated", "estimate"),
        ("evaluated", "evaluate"),
        ("calculated", "calculate"),
        ("illustrated", "illustrate"),
        ("demonstrated", "demonstrate"),
        ("indicated", "indicate"),
        ("validated", "validate"),
        ("regulated", "regulate"),
        ("simulated", "simulate"),
        ("formulated", "formulate"),
        ("populated", "populate"),
        ("elevated", "elevate"),
        ("cultivated", "cultivate"),
        ("initiated", "initiate"),
        ("negotiated", "negotiate"),
        ("operated", "operate"),
        ("generated", "generate"),
        ("translated", "translate"),
        ("updated", "update"),
        ("created", "create"),
        ("limited", "limit"),
        ("united", "unite"),
        ("excited", "excite"),
        ("detailed", "detail"),
        ("advanced", "advance"),
    ];

    for &(adj, base) in &common_adjectives {
        if word_lower == adj && dict.contains(base) {
            return true;
        }
    }

    // 17. 检查常见的不规则变化
    match word_lower.as_str() {
        "am" | "are" | "is" | "was" | "were" => return dict.contains("be"),
        "has" | "have" | "had" | "having" => return dict.contains("have"),
        "does" | "did" | "done" | "doing" => return dict.contains("do"),
        "goes" | "went" | "gone" | "going" => return dict.contains("go"),
        "makes" | "made" | "making" => return dict.contains("make"),
        "takes" | "took" | "taken" | "taking" => return dict.contains("take"),
        "comes" | "came" | "coming" => return dict.contains("come"),
        "sees" | "saw" | "seen" | "seeing" => return dict.contains("see"),
        "knows" | "knew" | "known" | "knowing" => return dict.contains("know"),
        "gets" | "got" | "gotten" | "getting" => return dict.contains("get"),
        "gives" | "gave" | "given" | "giving" => return dict.contains("give"),
        "finds" | "found" | "finding" => return dict.contains("find"),
        "thinks" | "thought" | "thinking" => return dict.contains("think"),
        "tells" | "told" | "telling" => return dict.contains("tell"),
        "becomes" | "became" | "becoming" => return dict.contains("become"),
        "shows" | "showed" | "shown" | "showing" => return dict.contains("show"),
        "leaves" | "left" | "leaving" => return dict.contains("leave"),
        "feels" | "felt" | "feeling" => return dict.contains("feel"),
        "puts" | "putting" => return dict.contains("put"),
        "means" | "meant" | "meaning" => return dict.contains("mean"),
        "keeps" | "kept" | "keeping" => return dict.contains("keep"),
        "lets" | "letting" => return dict.contains("let"),
        "begins" | "began" | "begun" | "beginning" => return dict.contains("begin"),
        "seems" | "seemed" | "seeming" => return dict.contains("seem"),
        "helps" | "helped" | "helping" => return dict.contains("help"),
        "talks" | "talked" | "talking" => return dict.contains("talk"),
        "turns" | "turned" | "turning" => return dict.contains("turn"),
        "starts" | "started" | "starting" => return dict.contains("start"),
        "hears" | "heard" | "hearing" => return dict.contains("hear"),
        "plays" | "played" | "playing" => return dict.contains("play"),
        "runs" | "ran" | "running" => return dict.contains("run"),
        "moves" | "moved" | "moving" => return dict.contains("move"),
        "lives" | "lived" | "living" => return dict.contains("live"),
        "believes" | "believed" | "believing" => return dict.contains("believe"),
        "says" | "said" | "saying" => return dict.contains("say"),
        "sits" | "sat" | "sitting" => return dict.contains("sit"),
        "stands" | "stood" | "standing" => return dict.contains("stand"),
        "loses" | "lost" | "losing" => return dict.contains("lose"),
        "pays" | "paid" | "paying" => return dict.contains("pay"),
        "meets" | "met" | "meeting" => return dict.contains("meet"),
        "includes" | "included" | "including" => return dict.contains("include"),
        "continues" | "continued" | "continuing" => return dict.contains("continue"),
        "sets" | "setting" => return dict.contains("set"),
        "learns" | "learned" | "learnt" | "learning" => return dict.contains("learn"),
        "changes" | "changed" | "changing" => return dict.contains("change"),
        "leads" | "led" | "leading" => return dict.contains("lead"),
        "understands" | "understood" | "understanding" => return dict.contains("understand"),
        "watches" | "watched" | "watching" => return dict.contains("watch"),
        "follows" | "followed" | "following" => return dict.contains("follow"),
        "stops" | "stopped" | "stopping" => return dict.contains("stop"),
        "creates" | "created" | "creating" => return dict.contains("create"),
        "speaks" | "spoke" | "spoken" | "speaking" => return dict.contains("speak"),
        "reads" | "read" | "reading" => return dict.contains("read"),
        "spends" | "spent" | "spending" => return dict.contains("spend"),
        "grows" | "grew" | "grown" | "growing" => return dict.contains("grow"),
        "opens" | "opened" | "opening" => return dict.contains("open"),
        "walks" | "walked" | "walking" => return dict.contains("walk"),
        "wins" | "won" | "winning" => return dict.contains("win"),
        "teaches" | "taught" | "teaching" => return dict.contains("teach"),
        "offers" | "offered" | "offering" => return dict.contains("offer"),
        "remembers" | "remembered" | "remembering" => return dict.contains("remember"),
        "considers" | "considered" | "considering" => return dict.contains("consider"),
        "appears" | "appeared" | "appearing" => return dict.contains("appear"),
        "buys" | "bought" | "buying" => return dict.contains("buy"),
        "serves" | "served" | "serving" => return dict.contains("serve"),
        "dies" | "died" | "dying" => return dict.contains("die"),
        "sends" | "sent" | "sending" => return dict.contains("send"),
        "builds" | "built" | "building" => return dict.contains("build"),
        "stays" | "stayed" | "staying" => return dict.contains("stay"),
        "falls" | "fell" | "fallen" | "falling" => return dict.contains("fall"),
        "cuts" | "cutting" => return dict.contains("cut"),
        "reaches" | "reached" | "reaching" => return dict.contains("reach"),
        "kills" | "killed" | "killing" => return dict.contains("kill"),
        "raises" | "raised" | "raising" => return dict.contains("raise"),
        _ => false,
    }
}

// 内置的常见单词列表（如果找不到词典文件时使用）
const COMMON_WORDS: &[&str] = &[
    "the",
    "be",
    "to",
    "of",
    "and",
    "a",
    "in",
    "that",
    "have",
    "I",
    "it",
    "for",
    "not",
    "on",
    "with",
    "he",
    "as",
    "you",
    "do",
    "at",
    "this",
    "but",
    "his",
    "by",
    "from",
    "they",
    "we",
    "say",
    "her",
    "she",
    "or",
    "an",
    "will",
    "my",
    "one",
    "all",
    "would",
    "there",
    "their",
    "what",
    "so",
    "up",
    "out",
    "if",
    "about",
    "who",
    "get",
    "which",
    "go",
    "me",
    "when",
    "make",
    "can",
    "like",
    "time",
    "no",
    "just",
    "him",
    "know",
    "take",
    "people",
    "into",
    "year",
    "your",
    "good",
    "some",
    "could",
    "them",
    "see",
    "other",
    "than",
    "then",
    "now",
    "look",
    "only",
    "come",
    "its",
    "over",
    "think",
    "also",
    "back",
    "after",
    "use",
    "two",
    "how",
    "our",
    "work",
    "first",
    "well",
    "way",
    "even",
    "new",
    "want",
    "because",
    "any",
    "these",
    "give",
    "day",
    "most",
    "us",
    // 学术论文中常用的单词
    "research",
    "analysis",
    "data",
    "method",
    "result",
    "conclusion",
    "study",
    "theory",
    "hypothesis",
    "experiment",
    "variable",
    "correlation",
    "significant",
    "evidence",
    "framework",
    "implementation",
    "development",
    "environment",
    "financial",
    "economic",
    "corporate",
    "business",
    "management",
    "strategy",
    "performance",
    "technology",
    "innovation",
    "sustainable",
    "organization",
    "industry",
    "production",
    "consumption",
    "investment",
    "marketing",
    "behavior",
    "psychology",
    "sociology",
    "political",
    "government",
    "regulation",
    "international",
    "global",
    "regional",
    "national",
    "population",
    "demographic",
    "environmental",
    "sustainability",
    "resources",
    "energy",
    "efficient",
    "renewable",
    "pollution",
    "conservation",
    "biodiversity",
    "ecosystem",
    "climate",
    "temperature",
    "atmosphere",
    "emissions",
    "carbon",
    "footprint",
    "digital",
    "computer",
    "software",
    "hardware",
    "network",
    "internet",
    "database",
    "algorithm",
    "programming",
    "artificial",
    "intelligence",
    "machine",
    "learning",
    "robotics",
    "automation",
    "virtual",
    "reality",
    "augmented",
    "simulation",
    "modeling",
    "prediction",
    "forecasting",
    "optimization",
    "efficiency",
    "effectiveness",
    "productivity",
    "quality",
    "reliability",
    "validity",
    "accuracy",
    "precision",
    "measurement",
    "evaluation",
    "assessment",
    "synthesis",
    "integration",
    "execution",
    "operation",
    "maintenance",
    "improvement",
    "enhancement",
    "maximization",
    "minimization",
    // 您示例中的单词
    "geographic",
    "endowment",
    "environment",
    "business",
    "corporate",
    "financial",
    "asset",
    "allocation",
    "empirical",
    "evidence",
    "share",
    "listed",
    // 常见的单词变形
    "mean",
    "means",
    "meant",
    "meaning",
];
