use regex::Regex;

struct Field {
    name: String,
    target: Target,
    selector: Selector,
}

enum Target {
    Html,
    Text,
    StripText,
    Attr(String),
}

impl TryFrom<&str> for Target {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            "html" => Ok(Self::Html),
            "text" => Ok(Self::Text),
            "strip_text" => Ok(Self::StripText),
            "src" => Ok(Self::Attr("src".to_string())),
            "href" => Ok(Self::Attr("href".to_string())),
            _ => {
                if let Some(v) = value.strip_prefix("attr=") {
                    Ok(Self::Attr(v.to_string()))
                } else {
                    Err(())
                }
            }
        }
    }
}

impl Field {
    fn parse(text: &str) -> Vec<Field> {
        let re = Regex::new(r#"\b([a-zA-Z0-9_]+)\[([a-z_=\-]+)]\s(.+)"#).unwrap();
        let mut res = vec![];
        for cap in re.captures_iter(text) {
            if let Ok(v) = Target::try_from(&cap[2]) {
                res.push(Field {
                    name: cap[1].to_string(),
                    target: v,
                    selector: Selector::parse(cap[3].to_string()),
                });
            } else {
                panic!("Invalid target: {}", &cap[2])
            }
        }
        res
    }
}

#[derive(Debug)]
enum SelectorType {
    Class,
    Id,
    Name,
}

#[derive(Debug)]
struct Selector {
    typ: SelectorType,
    name: String,
    child: Box<Child>,
}

#[derive(Debug)]
enum Child {
    Same(Selector),
    Descendant(Selector),
    BroadDescendant(Selector),
    None,
}

impl Selector {
    fn parse(mut s: String) -> Selector {
        let typ = if let Some(v) = s.strip_prefix("#") {
            s = v.to_string();
            SelectorType::Id
        } else if let Some(v) = s.strip_prefix(".") {
            s = v.to_string();
            SelectorType::Class
        } else {
            SelectorType::Name
        };

        let split_chars = [' ', '.', '#'];

        let (split_char_index, _) = split_chars.iter()
            .filter_map(|&c| s.find(c).map(|pos| (pos, c)))
            .min_by_key(|&(pos, _)| pos)
            .unwrap_or((s.len(), ' '));
        if split_char_index < s.len() {
            let (name, next) = s.split_at(split_char_index);
            let child = if let Some(v) = next.strip_prefix(" ") {
                if let Some(v) = v.strip_prefix("... ") {
                    Child::BroadDescendant(Self::parse(v.to_string()))
                } else {
                    Child::Descendant(Self::parse(v.to_string()))
                }
            } else {
                Child::Same(Self::parse(next.to_string()))
            };
            Selector {
                typ,
                name: name.to_string(),
                child: Box::new(child),
            }
        } else {
            Selector {
                typ,
                name: s,
                child: Box::new(Child::None),
            }
        }
    }
}