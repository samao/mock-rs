use rand::{random_range, seq::IndexedRandom};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, ops::Range, sync::LazyLock};

static CLAST: LazyLock<Vec<&str>> = LazyLock::new(|| {
    include_str!("config/clastname.in")
        .split_ascii_whitespace()
        .collect()
});

static CFIRST: LazyLock<Vec<&str>> = LazyLock::new(|| {
    include_str!("config/cfirstname.in")
        .split_ascii_whitespace()
        .collect()
});

static CWORD: LazyLock<Vec<&str>> =
    LazyLock::new(|| include_str!("config/cword.in").split("").collect());

static ADDRESS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let lines = include_str!("config/address.in");
    let reg = regex::Regex::new(r#"(?<zip>\d{6})": "(?<city>.+)""#).unwrap();
    let mut map = HashMap::new();
    for i in lines.lines() {
        let caps = match reg.captures(i) {
            Some(caps) => caps,
            None => continue,
        };
        let zip = caps.name("zip").unwrap().as_str();
        let city = caps.name("city").unwrap().as_str();

        let pid = match (&zip[2..6], &zip[4..6]) {
            ("0000", _) => None,
            (_, "00") => Some(format!("{}0000", &zip[..2])),
            _ => Some(format!("{}00", &zip[..4])),
        };
        map.insert(
            zip.to_owned(),
            Node {
                zip: zip.to_owned(),
                name: city.to_owned(),
                pcode: pid,
                children: RefCell::new(vec![]),
            },
        );
    }

    map.values()
        .into_iter()
        .filter(|node| !node.zip.ends_with("00"))
        .map(|node| {
            let mut current_node = node;
            let mut right_name = format!("{}", current_node.name);
            loop {
                match &current_node.pcode {
                    None => {
                        break right_name;
                    }
                    Some(code) => {
                        if let Some(node) = map.get(code) {
                            current_node = node;
                            right_name = format!("{} {}", current_node.name, right_name);
                            continue;
                        }
                        break right_name;
                    }
                }
            }
        })
        .collect()
});

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Node {
    zip: String,
    name: String,
    pcode: Option<String>,
    children: RefCell<Vec<Node>>,
}

pub fn ip4() -> String {
    format!(
        "{}.{}.{}.{}",
        integer(10..255),
        integer(0..255),
        integer(0..255),
        integer(0..255)
    )
}

pub fn address() -> String {
    ADDRESS.choose(&mut rand::rng()).unwrap().to_string()
}

pub fn cname() -> String {
    format!(
        "{}{}",
        CFIRST.choose(&mut rand::rng()).unwrap().to_string(),
        CLAST.choose(&mut rand::rng()).unwrap().to_string()
    )
}

pub fn csentence(range: Range<usize>) -> String {
    let total = rand::random_range(range);
    let mut result = vec![];
    for _ in 0..total {
        result.push(cword());
    }
    result.join("")
}

fn cword() -> String {
    CWORD.choose(&mut rand::rng()).unwrap().to_string()
}

pub fn boolean(tpart: u32, all: u32) -> bool {
    rand::random_ratio(tpart, all)
}

pub fn integer(range: Range<usize>) -> usize {
    rand::random_range(range)
}

pub fn color() -> String {
    format!("#{:06x?}", random_range(0..0xFFFFFF))
}

pub fn image(
    (width, height): (usize, usize),
    background_color: &str,
    foreground_color: &str,
    format: Option<&str>,
    text: Option<&str>,
) -> String {
    //http://dummyimage.com/600x400/cc00cc/470047.png&text=hello
    format!(
        "http://dummyimage.com/{}x{}/{}/{}{}{}",
        width,
        height,
        match &background_color[0..1] {
            "#" => &background_color[1..],
            _ => background_color,
        },
        match &foreground_color[0..1] {
            "#" => &foreground_color[1..],
            _ => foreground_color,
        },
        match format {
            Some(format) => format!(".{}", format),
            _ => "".to_owned(),
        },
        match text {
            Some(text) => format!("&text={}", text),
            _ => "".to_owned(),
        }
    )
}
