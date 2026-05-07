use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Read a file into memory and return `None` when reading fails.
fn get_file_content(fpath: &Path) -> Option<Vec<u8>> {
    fs::read(fpath).ok()
}

/// Remove leading characters that appear in `del` from `s`.
fn remove_beginning(s: &str, del: &str) -> String {
    s.trim_start_matches(|c: char| del.contains(c)).to_string()
}

/// Split raw file bytes by `<` and `>`, trimming leading spaces from each segment.
fn separate_tags(input: &[u8]) -> Vec<String> {
    let mut result = Vec::new();
    let mut current: Vec<u8> = Vec::new();
    let mut inside_tag = false;

    for &byte in input {
        let c = byte as char;
        if c == '<' && !inside_tag {
            result.push(String::from_utf8_lossy(&current).into_owned());
            current.clear();
            inside_tag = true;
        } else if c == '>' && inside_tag {
            result.push(String::from_utf8_lossy(&current).into_owned());
            current.clear();
            inside_tag = false;
        } else {
            current.push(byte);
        }
    }

    result
        .into_iter()
        .map(|s| remove_beginning(&s, " "))
        .collect()
}

/// Split `s` by `delimiter`, but treat `quote`-enclosed regions as unsplittable.
/// Faithfully replicates the C++ original, including its quirk at the last character.
fn split_quote(s: &str, delimiter: char, quote: char) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut tokens: Vec<String> = Vec::new();
    let mut is = 0usize;
    let mut is_in = false;

    for ie in 0..n {
        if ie == n - 1 {
            let token: String = chars[is..=ie].iter().collect();
            tokens.push(token);
        }
        if chars[ie] == quote {
            is_in = !is_in;
        }
        if chars[ie] == delimiter && !is_in {
            if chars[is] != delimiter {
                let token: String = chars[is..ie].iter().collect();
                tokens.push(token);
            }
            is = ie + 1;
        }
    }
    tokens
}

/// Strip the outermost pair of `quot` characters from `s`.
fn remove_quote(s: &str, quot: char) -> String {
    let chars: Vec<char> = s.chars().collect();
    let count = chars.iter().filter(|&&c| c == quot).count();
    if count < 2 {
        return s.to_string();
    }
    let istat = chars.iter().position(|&c| c == quot).unwrap();
    let iend = chars.iter().rposition(|&c| c == quot).unwrap();
    chars[istat + 1..iend].iter().collect()
}

/// Parse a tag's attribute string (e.g. `svg width="100" height="200"`) into a map.
fn parse_tag_contents(input: &str) -> HashMap<String, String> {
    let tokens = split_quote(input, ' ', '"');
    let mut map = HashMap::new();
    for token in &tokens {
        let parts: Vec<&str> = token.splitn(2, '=').collect();
        if parts.len() != 2 {
            continue;
        }
        let val = remove_quote(parts[1], '"');
        map.insert(parts[0].to_string(), val);
    }
    map
}

/// Parse the SVG file at `file_path` and return (width, height, path-data string).
/// Returns (0, 0, "") on failure.
pub fn get_image_size_and_shape(file_path: &Path) -> (usize, usize, String) {
    let content = match get_file_content(file_path) {
        Some(c) => c,
        None => return (0, 0, String::new()),
    };

    let tags = separate_tags(&content);
    let mut width = 0usize;
    let mut height = 0usize;
    let mut shape = String::new();

    for tag in &tags {
        if tag.starts_with("svg ") {
            let attrs = parse_tag_contents(tag);
            if let Some(h) = attrs.get("height") {
                height = h.parse().unwrap_or(0);
            }
            if let Some(w) = attrs.get("width") {
                width = w.parse().unwrap_or(0);
            }
        }
        if tag.starts_with("path ") {
            let attrs = parse_tag_contents(tag);
            if let Some(d) = attrs.get("d") {
                shape = d.clone();
            }
        }
    }

    (width, height, shape)
}

/// Tokenise an SVG path data string (the `d` attribute) into a flat list of
/// command letters and numeric strings.
pub fn outline_path_from_shape(s0: &str) -> Vec<String> {
    let chars: Vec<char> = s0.chars().collect();
    let n = chars.len();
    let mut result: Vec<String> = Vec::new();
    let mut imark = 0usize;

    for i in 0..n {
        let c = chars[i];
        if c.is_ascii_digit() || c == '.' {
            continue;
        }
        if c == ',' {
            let token: String = chars[imark..i].iter().collect();
            result.push(token);
            imark = i + 1;
        } else if c == ' ' {
            if i > imark {
                let token: String = chars[imark..i].iter().collect();
                result.push(token);
            }
            imark = i + 1;
        } else if c == '-' {
            if i > imark {
                let token: String = chars[imark..i].iter().collect();
                result.push(token);
            }
            imark = i; // keep '-' as part of the next token
        } else if c.is_alphabetic() {
            if i > imark {
                let token: String = chars[imark..i].iter().collect();
                result.push(token);
            }
            result.push(c.to_string());
            imark = i + 1;
        }
    }
    // Push any remaining numeric token after the last delimiter
    if imark < n {
        let token: String = chars[imark..n].iter().collect();
        if !token.is_empty() {
            result.push(token);
        }
    }
    result
}

/// A line-segment or quadratic Bézier edge.
#[derive(Debug, Clone)]
pub struct Edge {
    pub ps: [f32; 2], // start point
    pub pe: [f32; 2], // end point
    pub pc: [f32; 2], // control point (only meaningful when is_bezier == true)
    pub is_bezier: bool,
}

impl Edge {
    /// Construct a straight line edge from `ps` to `pe`.
    pub fn new_line(ps: [f32; 2], pe: [f32; 2]) -> Self {
        Edge {
            ps,
            pe,
            pc: [0.0, 0.0],
            is_bezier: false,
        }
    }

    /// Construct a quadratic Bezier edge from `ps` through `pc` to `pe`.
    pub fn new_bezier(ps: [f32; 2], pc: [f32; 2], pe: [f32; 2]) -> Self {
        Edge {
            ps,
            pe,
            pc,
            is_bezier: true,
        }
    }
}

/// Return true when `s` begins with an ASCII alphabetic command token.
fn is_alpha_token(s: &str) -> bool {
    s.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false)
}

/// Parse two consecutive float values from `strs` starting at index `i`.
fn parse2(strs: &[String], i: usize) -> (f32, f32) {
    assert!(
        i + 1 < strs.len(),
        "parse2: index out of bounds (i={}, len={})",
        i,
        strs.len()
    );
    (strs[i].parse().unwrap(), strs[i + 1].parse().unwrap())
}

/// Convert the flat token list produced by `svg_outline_path_from_shape` into
/// closed loops of edges.
pub fn loops_from_outline_path(strs: &[String]) -> Vec<Vec<Edge>> {
    assert!(strs[0] == "M" || strs[0] == "m");
    assert!(strs[strs.len() - 1] == "Z" || strs[strs.len() - 1] == "z");

    let mut loops: Vec<Vec<Edge>> = Vec::new();
    let mut edges_buffer: Vec<Edge> = Vec::new();
    let mut pos_cur = [0f32, 0f32];
    let mut is = 0usize;

    loop {
        let cmd = strs[is].as_str();
        match cmd {
            "M" => {
                // absolute move-to, then optional implicit line-to pairs
                is += 1;
                let (x, y) = parse2(strs, is);
                pos_cur = [x, y];
                is += 2;
                while !is_alpha_token(&strs[is]) {
                    let (x, y) = parse2(strs, is);
                    let p1 = [x, y];
                    edges_buffer.push(Edge::new_line(pos_cur, p1));
                    pos_cur = p1;
                    is += 2;
                }
            }
            "m" => {
                // relative move-to
                is += 1;
                let (dx, dy) = parse2(strs, is);
                pos_cur = [pos_cur[0] + dx, pos_cur[1] + dy];
                is += 2;
                while !is_alpha_token(&strs[is]) {
                    let (dx, dy) = parse2(strs, is);
                    let p1 = [pos_cur[0] + dx, pos_cur[1] + dy];
                    edges_buffer.push(Edge::new_line(pos_cur, p1));
                    pos_cur = p1;
                    is += 2;
                }
            }
            "l" => {
                // relative line-to
                is += 1;
                loop {
                    let (dx, dy) = parse2(strs, is);
                    let p1 = [pos_cur[0] + dx, pos_cur[1] + dy];
                    edges_buffer.push(Edge::new_line(pos_cur, p1));
                    pos_cur = p1;
                    is += 2;
                    if is_alpha_token(&strs[is]) {
                        break;
                    }
                }
            }
            "L" => {
                // absolute line-to
                is += 1;
                loop {
                    let (x, y) = parse2(strs, is);
                    let p1 = [x, y];
                    edges_buffer.push(Edge::new_line(pos_cur, p1));
                    pos_cur = p1;
                    is += 2;
                    if is_alpha_token(&strs[is]) {
                        break;
                    }
                }
            }
            "v" => {
                // relative vertical line-to
                let dy: f32 = strs[is + 1].parse().unwrap();
                let p1 = [pos_cur[0], pos_cur[1] + dy];
                edges_buffer.push(Edge::new_line(pos_cur, p1));
                pos_cur = p1;
                is += 2;
            }
            "V" => {
                // absolute vertical line-to
                let y: f32 = strs[is + 1].parse().unwrap();
                let p1 = [pos_cur[0], y];
                edges_buffer.push(Edge::new_line(pos_cur, p1));
                pos_cur = p1;
                is += 2;
            }
            "H" => {
                // absolute horizontal line-to
                let x: f32 = strs[is + 1].parse().unwrap();
                let p1 = [x, pos_cur[1]];
                edges_buffer.push(Edge::new_line(pos_cur, p1));
                pos_cur = p1;
                is += 2;
            }
            "h" => {
                // relative horizontal line-to
                let dh: f32 = strs[is + 1].parse().unwrap();
                let p1 = [pos_cur[0] + dh, pos_cur[1]];
                edges_buffer.push(Edge::new_line(pos_cur, p1));
                pos_cur = p1;
                is += 2;
            }
            "q" => {
                // relative quadratic Bézier
                is += 1;
                loop {
                    let (dcx, dcy) = parse2(strs, is);
                    let (dx, dy) = parse2(strs, is + 2);
                    let pm0 = [pos_cur[0] + dcx, pos_cur[1] + dcy];
                    let p1 = [pos_cur[0] + dx, pos_cur[1] + dy];
                    edges_buffer.push(Edge::new_bezier(pos_cur, pm0, p1));
                    pos_cur = p1;
                    is += 4;
                    if is_alpha_token(&strs[is]) {
                        break;
                    }
                }
            }
            "Q" => {
                // absolute quadratic Bézier
                is += 1;
                loop {
                    let (cx, cy) = parse2(strs, is);
                    let (x, y) = parse2(strs, is + 2);
                    let pm0 = [cx, cy];
                    let p1 = [x, y];
                    edges_buffer.push(Edge::new_bezier(pos_cur, pm0, p1));
                    pos_cur = p1;
                    is += 4;
                    if is_alpha_token(&strs[is]) {
                        break;
                    }
                }
            }
            "z" | "Z" => {
                // close path
                let pe = edges_buffer[0].ps;
                let ps = edges_buffer[edges_buffer.len() - 1].pe;
                let dist = ((ps[0] - pe[0]).powi(2) + (ps[1] - pe[1]).powi(2)).sqrt();
                if dist > 1.0e-9 {
                    edges_buffer.push(Edge::new_line(ps, pe));
                }
                loops.push(edges_buffer.clone());
                edges_buffer.clear();
                if is == strs.len() - 1 {
                    break;
                }
                is += 1;
            }
            other => {
                eprintln!("error!--> {}", other);
                break;
            }
        }
    }

    loops
}
