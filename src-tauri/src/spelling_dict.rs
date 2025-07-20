use std::collections::HashMap;

// 创建一个包含常见学术英文拼写错误的字典
pub fn get_academic_spelling_dict() -> HashMap<&'static str, &'static str> {
    let mut dict = HashMap::new();

    // 基础常见拼写错误
    dict.insert("teh", "the");
    dict.insert("recieve", "receive");
    dict.insert("wierd", "weird");
    dict.insert("alot", "a lot");
    dict.insert("definately", "definitely");
    dict.insert("seperate", "separate");
    dict.insert("occured", "occurred");
    dict.insert("accomodate", "accommodate");
    dict.insert("adress", "address");
    dict.insert("advertisment", "advertisement");
    dict.insert("agressive", "aggressive");
    dict.insert("apparant", "apparent");
    dict.insert("appearence", "appearance");
    dict.insert("arguement", "argument");
    dict.insert("assasination", "assassination");
    dict.insert("basicly", "basically");
    dict.insert("begining", "beginning");
    dict.insert("beleive", "believe");
    dict.insert("belive", "believe");
    dict.insert("buisness", "business");
    dict.insert("calender", "calendar");
    dict.insert("catagory", "category");
    dict.insert("cemetary", "cemetery");
    dict.insert("changable", "changeable");
    dict.insert("cheif", "chief");
    dict.insert("collegue", "colleague");
    dict.insert("comming", "coming");
    dict.insert("commitee", "committee");
    dict.insert("completly", "completely");
    dict.insert("concious", "conscious");
    dict.insert("curiousity", "curiosity");
    dict.insert("decieve", "deceive");
    dict.insert("definate", "definite");
    dict.insert("definitly", "definitely");
    dict.insert("dissapoint", "disappoint");
    dict.insert("embarass", "embarrass");
    dict.insert("enviroment", "environment");
    dict.insert("existance", "existence");
    dict.insert("experiance", "experience");
    dict.insert("familliar", "familiar");
    dict.insert("finaly", "finally");
    dict.insert("foriegn", "foreign");
    dict.insert("freind", "friend");
    dict.insert("goverment", "government");
    dict.insert("gaurd", "guard");
    dict.insert("happend", "happened");
    dict.insert("harrass", "harass");
    dict.insert("hieght", "height");
    dict.insert("immediatly", "immediately");
    dict.insert("independant", "independent");
    dict.insert("interupt", "interrupt");
    dict.insert("irrelevent", "irrelevant");
    dict.insert("knowlege", "knowledge");
    dict.insert("liason", "liaison");
    dict.insert("libary", "library");
    dict.insert("lisence", "license");
    dict.insert("maintainance", "maintenance");
    dict.insert("managment", "management");
    dict.insert("medecine", "medicine");
    dict.insert("millenium", "millennium");
    dict.insert("miniscule", "minuscule");
    dict.insert("mispell", "misspell");
    dict.insert("neccessary", "necessary");
    dict.insert("negociate", "negotiate");
    dict.insert("nieghbor", "neighbor");
    dict.insert("noticable", "noticeable");
    dict.insert("occassion", "occasion");
    dict.insert("occassionally", "occasionally");
    dict.insert("occurance", "occurrence");
    dict.insert("ocurrance", "occurrence");
    dict.insert("oppurtunity", "opportunity");
    dict.insert("persistant", "persistent");
    dict.insert("posession", "possession");
    dict.insert("prefered", "preferred");
    dict.insert("presance", "presence");
    dict.insert("propoganda", "propaganda");
    dict.insert("publically", "publicly");
    dict.insert("realy", "really");
    dict.insert("reccomend", "recommend");
    dict.insert("refered", "referred");
    dict.insert("relevent", "relevant");
    dict.insert("religous", "religious");
    dict.insert("remeber", "remember");
    dict.insert("repitition", "repetition");
    dict.insert("rythm", "rhythm");
    dict.insert("secratary", "secretary");
    dict.insert("sieze", "seize");
    dict.insert("similer", "similar");
    dict.insert("sincerly", "sincerely");
    dict.insert("speach", "speech");
    dict.insert("succesful", "successful");
    dict.insert("supercede", "supersede");
    dict.insert("supress", "suppress");
    dict.insert("suprise", "surprise");
    dict.insert("temperture", "temperature");
    dict.insert("tendancy", "tendency");
    dict.insert("therefor", "therefore");
    dict.insert("threshhold", "threshold");
    dict.insert("tommorrow", "tomorrow");
    dict.insert("tounge", "tongue");
    dict.insert("truely", "truly");
    dict.insert("twelth", "twelfth");
    dict.insert("tyrany", "tyranny");
    dict.insert("underate", "underrate");
    dict.insert("untill", "until");
    dict.insert("usally", "usually");
    dict.insert("vaccuum", "vacuum");
    dict.insert("vegtable", "vegetable");
    dict.insert("vehical", "vehicle");
    dict.insert("visable", "visible");
    dict.insert("wether", "whether");
    dict.insert("withhold", "withhold");
    dict.insert("writting", "writing");

    // 学术论文中常见错误
    dict.insert("enronment", "environment");
    dict.insert("financal", "financial");
    dict.insert("alocation", "allocation");
    dict.insert("empincal", "empirical");
    dict.insert("eydence", "evidence");
    dict.insert("analyis", "analysis");
    dict.insert("reseach", "research");
    dict.insert("statisical", "statistical");
    dict.insert("significiant", "significant");
    dict.insert("hypothsis", "hypothesis");
    dict.insert("methodolgy", "methodology");
    dict.insert("framwork", "framework");
    dict.insert("implmentation", "implementation");
    dict.insert("exprimental", "experimental");
    dict.insert("corelation", "correlation");
    dict.insert("varibles", "variables");
    dict.insert("efficency", "efficiency");
    dict.insert("optimzation", "optimization");
    dict.insert("algoritm", "algorithm");
    dict.insert("proceedure", "procedure");
    dict.insert("comparision", "comparison");
    dict.insert("improvment", "improvement");
    dict.insert("performace", "performance");
    dict.insert("technolgoy", "technology");
    dict.insert("inovation", "innovation");
    dict.insert("developement", "development");
    dict.insert("infomation", "information");
    dict.insert("comunication", "communication");
    dict.insert("straegy", "strategy");
    dict.insert("competitve", "competitive");
    dict.insert("advantge", "advantage");
    dict.insert("sustainble", "sustainable");
    dict.insert("organiztion", "organization");
    dict.insert("leadrship", "leadership");
    dict.insert("corprate", "corporate");
    dict.insert("enterprse", "enterprise");
    dict.insert("industy", "industry");
    dict.insert("manufactring", "manufacturing");
    dict.insert("producton", "production");
    dict.insert("distribtion", "distribution");
    dict.insert("consumtion", "consumption");
    dict.insert("econmic", "economic");
    dict.insert("finacial", "financial");
    dict.insert("investent", "investment");
    dict.insert("markting", "marketing");
    dict.insert("advertsing", "advertising");
    dict.insert("behavor", "behavior");
    dict.insert("psycholgy", "psychology");
    dict.insert("sociolgy", "sociology");
    dict.insert("politcal", "political");
    dict.insert("governent", "government");
    dict.insert("regultion", "regulation");
    dict.insert("legisltion", "legislation");
    dict.insert("interntional", "international");
    dict.insert("globl", "global");
    dict.insert("reginal", "regional");
    dict.insert("natinal", "national");
    dict.insert("popultion", "population");
    dict.insert("demographc", "demographic");
    dict.insert("geographc", "geographic");
    dict.insert("environental", "environmental");
    dict.insert("sustainbility", "sustainability");
    dict.insert("resouces", "resources");
    dict.insert("enery", "energy");
    dict.insert("efficent", "efficient");
    dict.insert("renewble", "renewable");
    dict.insert("polluton", "pollution");
    dict.insert("conservtion", "conservation");
    dict.insert("biodivrsity", "biodiversity");
    dict.insert("ecosytem", "ecosystem");
    dict.insert("climte", "climate");
    dict.insert("atmosphre", "atmosphere");
    dict.insert("emisssions", "emissions");
    dict.insert("carbbon", "carbon");
    dict.insert("footprnt", "footprint");
    dict.insert("digitl", "digital");
    dict.insert("computr", "computer");
    dict.insert("softwre", "software");
    dict.insert("hardwre", "hardware");
    dict.insert("netwrk", "network");
    dict.insert("internnet", "internet");
    dict.insert("databse", "database");
    dict.insert("programing", "programming");
    dict.insert("artifical", "artificial");
    dict.insert("intellgence", "intelligence");
    dict.insert("machne", "machine");
    dict.insert("learnng", "learning");
    dict.insert("robotcs", "robotics");
    dict.insert("automtion", "automation");
    dict.insert("virtal", "virtual");
    dict.insert("realiy", "reality");
    dict.insert("augmeted", "augmented");
    dict.insert("simultion", "simulation");
    dict.insert("modelng", "modeling");
    dict.insert("predicton", "prediction");
    dict.insert("forecsting", "forecasting");
    dict.insert("effectveness", "effectiveness");
    dict.insert("productvity", "productivity");
    dict.insert("qualiy", "quality");
    dict.insert("reliablity", "reliability");
    dict.insert("validty", "validity");
    dict.insert("accurcy", "accuracy");
    dict.insert("precison", "precision");
    dict.insert("measurment", "measurement");
    dict.insert("evaluaton", "evaluation");
    dict.insert("assessent", "assessment");
    dict.insert("synthsis", "synthesis");
    dict.insert("integrtion", "integration");
    dict.insert("executon", "execution");
    dict.insert("operaton", "operation");
    dict.insert("maintenace", "maintenance");
    dict.insert("enhancment", "enhancement");
    dict.insert("maximiztion", "maximization");
    dict.insert("minimiztion", "minimization");

    // 针对示例中的特定错误
    dict.insert("endowment", "endowment");
    dict.insert("enronment", "environment");
    dict.insert("financal", "financial");
    dict.insert("alocation", "allocation");
    dict.insert("empincal", "empirical");
    dict.insert("eydence", "evidence");
    dict.insert("corporat", "corporate");
    dict.insert("corprate", "corporate");
    dict.insert("geographc", "geographic");
    dict.insert("geographi", "geographic");
    dict.insert("busines", "business");
    dict.insert("asset", "asset");

    // 添加更多学术词汇的常见拼写错误
    dict.insert("academc", "academic");
    dict.insert("achievment", "achievement");
    dict.insert("aquisition", "acquisition");
    dict.insert("adminstration", "administration");
    dict.insert("aggreement", "agreement");
    dict.insert("aproximate", "approximate");
    dict.insert("arguement", "argument");
    dict.insert("assesment", "assessment");
    dict.insert("benifit", "benefit");
    dict.insert("catagory", "category");
    dict.insert("challange", "challenge");
    dict.insert("committe", "committee");
    dict.insert("competetive", "competitive");
    dict.insert("concensus", "consensus");
    dict.insert("contigency", "contingency");
    dict.insert("controversal", "controversial");
    dict.insert("conveniance", "convenience");
    dict.insert("coorporation", "corporation");
    dict.insert("criterias", "criteria");
    dict.insert("decison", "decision");
    dict.insert("deficiet", "deficit");
    dict.insert("definiton", "definition");
    dict.insert("disipline", "discipline");
    dict.insert("disscussion", "discussion");
    dict.insert("ecconomic", "economic");
    dict.insert("efficency", "efficiency");
    dict.insert("emphsis", "emphasis");
    dict.insert("enviorment", "environment");
    dict.insert("equiptment", "equipment");
    dict.insert("exagerate", "exaggerate");
    dict.insert("excercise", "exercise");
    dict.insert("explaination", "explanation");
    dict.insert("explicity", "explicitly");
    dict.insert("expresion", "expression");
    dict.insert("faciliate", "facilitate");
    dict.insert("facinated", "fascinated");
    dict.insert("foriegn", "foreign");
    dict.insert("fourty", "forty");
    dict.insert("freqently", "frequently");
    dict.insert("guage", "gauge");
    dict.insert("garantee", "guarantee");
    dict.insert("guidlines", "guidelines");
    dict.insert("heirarchy", "hierarchy");
    dict.insert("homogenous", "homogeneous");
    dict.insert("hypocracy", "hypocrisy");
    dict.insert("hipothesis", "hypothesis");
    dict.insert("identiy", "identity");
    dict.insert("immediatly", "immediately");
    dict.insert("impliment", "implement");
    dict.insert("improvment", "improvement");
    dict.insert("incidently", "incidentally");
    dict.insert("independant", "independent");
    dict.insert("indispensible", "indispensable");
    dict.insert("inefficent", "inefficient");
    dict.insert("infered", "inferred");
    dict.insert("influencial", "influential");
    dict.insert("inteligence", "intelligence");
    dict.insert("intergrated", "integrated");
    dict.insert("interpretted", "interpreted");
    dict.insert("interuption", "interruption");
    dict.insert("irrelevent", "irrelevant");
    dict.insert("knowlege", "knowledge");
    dict.insert("liesure", "leisure");
    dict.insert("liason", "liaison");
    dict.insert("libraray", "library");
    dict.insert("liscense", "license");
    dict.insert("maintenence", "maintenance");
    dict.insert("managment", "management");
    dict.insert("manditory", "mandatory");
    dict.insert("mathmatics", "mathematics");
    dict.insert("medcine", "medicine");
    dict.insert("millenium", "millennium");
    dict.insert("miscelaneous", "miscellaneous");
    dict.insert("morgage", "mortgage");
    dict.insert("necesary", "necessary");
    dict.insert("negotation", "negotiation");
    dict.insert("nieghbor", "neighbor");
    dict.insert("noticable", "noticeable");
    dict.insert("occassion", "occasion");
    dict.insert("occurance", "occurrence");
    dict.insert("occurence", "occurrence");
    dict.insert("ommision", "omission");
    dict.insert("oppurtunity", "opportunity");
    dict.insert("orignal", "original");
    dict.insert("outragous", "outrageous");
    dict.insert("parrallel", "parallel");
    dict.insert("parliment", "parliament");
    dict.insert("particpant", "participant");
    dict.insert("persistant", "persistent");
    dict.insert("personel", "personnel");
    dict.insert("phenomina", "phenomena");
    dict.insert("posession", "possession");
    dict.insert("potentialy", "potentially");
    dict.insert("practicle", "practical");
    dict.insert("preceed", "precede");
    dict.insert("predjudice", "prejudice");
    dict.insert("presance", "presence");
    dict.insert("privelege", "privilege");
    dict.insert("probaly", "probably");
    dict.insert("proceedure", "procedure");
    dict.insert("proffesional", "professional");
    dict.insert("promiss", "promise");
    dict.insert("pronounciation", "pronunciation");
    dict.insert("prupose", "purpose");
    dict.insert("psuedo", "pseudo");
    dict.insert("psychicly", "psychically");
    dict.insert("publically", "publicly");
    dict.insert("quarentine", "quarantine");
    dict.insert("questionaire", "questionnaire");
    dict.insert("readible", "readable");
    dict.insert("realy", "really");
    dict.insert("reccomend", "recommend");
    dict.insert("recieve", "receive");
    dict.insert("reconize", "recognize");
    dict.insert("refered", "referred");
    dict.insert("referance", "reference");
    dict.insert("relevent", "relevant");
    dict.insert("religous", "religious");
    dict.insert("reluctent", "reluctant");
    dict.insert("remeber", "remember");
    dict.insert("repatition", "repetition");
    dict.insert("restaraunt", "restaurant");
    dict.insert("rythm", "rhythm");
    dict.insert("scedule", "schedule");
    dict.insert("secratary", "secretary");
    dict.insert("seperate", "separate");
    dict.insert("sieze", "seize");
    dict.insert("similer", "similar");
    dict.insert("sincerity", "sincerity");
    dict.insert("sophmore", "sophomore");
    dict.insert("specifc", "specific");
    dict.insert("strenght", "strength");
    dict.insert("succesful", "successful");
    dict.insert("supercede", "supersede");
    dict.insert("surpress", "suppress");
    dict.insert("suprise", "surprise");
    dict.insert("temperture", "temperature");
    dict.insert("tendancy", "tendency");
    dict.insert("therefor", "therefore");
    dict.insert("threshhold", "threshold");
    dict.insert("tommorrow", "tomorrow");
    dict.insert("tounge", "tongue");
    dict.insert("truely", "truly");
    dict.insert("twelth", "twelfth");
    dict.insert("tyrany", "tyranny");
    dict.insert("underate", "underrate");
    dict.insert("untill", "until");
    dict.insert("unuseual", "unusual");
    dict.insert("usally", "usually");
    dict.insert("vaccuum", "vacuum");
    dict.insert("vegatarian", "vegetarian");
    dict.insert("vehical", "vehicle");
    dict.insert("visable", "visible");
    dict.insert("volenteer", "volunteer");
    dict.insert("warrenty", "warranty");
    dict.insert("wether", "whether");
    dict.insert("wierd", "weird");
    dict.insert("wellfare", "welfare");
    dict.insert("welfair", "welfare");
    dict.insert("wilfull", "willful");
    dict.insert("withold", "withhold");
    dict.insert("writting", "writing");

    dict
}

// 检查单词是否是拼写错误，如果是则返回正确的拼写
pub fn check_word_spelling(word: &str) -> Option<&'static str> {
    let dict = get_academic_spelling_dict();
    dict.get(word.to_lowercase().as_str()).copied()
}

// 检查文本中的拼写错误
pub fn check_text_spelling(text: &str) -> Vec<(String, String, usize, usize)> {
    let mut errors = Vec::new();
    let dict = get_academic_spelling_dict();

    // 将文本分割成单词
    for (line_idx, line) in text.lines().enumerate() {
        let words: Vec<&str> = line.split_whitespace().collect();

        let mut pos = 0;
        for word in words {
            // 跳过空白字符（字符安全）
            while pos < line.len() {
                // 确保pos在字符边界上
                if let Some(remaining) = line.get(pos..) {
                    if remaining.starts_with(|c: char| c.is_whitespace()) {
                        // 安全地移动到下一个字符
                        if let Some(ch) = remaining.chars().next() {
                            pos += ch.len_utf8();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // 找到单词的位置（字符安全）
            let word_pos = if pos < line.len() {
                match line.get(pos..).and_then(|remaining| remaining.find(word)) {
                    Some(p) => pos + p,
                    None => {
                        pos += word.len();
                        continue;
                    }
                }
            } else {
                break;
            };

            // 清理单词，去除标点符号
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
            if clean_word.is_empty() {
                pos = word_pos + word.len();
                continue;
            }

            // 检查单词拼写
            if let Some(correction) = dict.get(clean_word.to_lowercase().as_str()) {
                errors.push((
                    clean_word.to_string(),
                    correction.to_string(),
                    line_idx,
                    word_pos,
                ));
            }

            pos = word_pos + word.len();
        }
    }

    errors
}
