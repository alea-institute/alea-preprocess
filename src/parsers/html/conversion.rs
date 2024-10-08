use lazy_static::*;
use regex::*;
use std::collections::{HashMap, HashSet};
use tl;

// list of standard exclude tags
const DEFAULT_EXCLUDE_TAGS: [&str; 2] = ["script", "style"];

// default max depth
const DEFAULT_MAX_DEPTH: usize = 1024;

// regex to clean up 3+ newlines
lazy_static! {
    static ref RE_CLEAN_NEWLINE: Regex = Regex::new(r"\n{3,}").unwrap();
}

/// Normalize the markdown output.
/// Arguments:
/// - markdown: The markdown to clean up.
/// Returns:
/// - The cleaned up markdown.
pub fn normalize_markdown(markdown: &str) -> String {
    RE_CLEAN_NEWLINE
        .replace_all(markdown, "\n\n")
        .replace("\n[\n", "\n[")
        .replace("\n](", "](")
        .to_string()
}

/// Parser configuration struct
/// Fields:
/// - exclude_tags: A list of tags to exclude from the output.
/// - output_links: Whether to output links in the output.
/// - output_images: Whether to output images in the output.
pub struct ParserConfig {
    pub exclude_tags: HashSet<String>,
    pub output_links: bool,
    pub output_images: bool,
}

impl ParserConfig {
    pub fn new(
        exclude_tags: Option<HashSet<String>>,
        output_links: bool,
        output_images: bool,
    ) -> ParserConfig {
        if exclude_tags.is_none() {
            let init_exclude_tags: HashSet<String> =
                DEFAULT_EXCLUDE_TAGS.iter().map(|x| x.to_string()).collect();
            ParserConfig {
                exclude_tags: init_exclude_tags,
                output_links,
                output_images,
            }
        } else {
            ParserConfig {
                exclude_tags: exclude_tags.unwrap(),
                output_links,
                output_images,
            }
        }
    }
}

/// Get the tag name from a node.
/// Arguments:
/// - node: The node to get the tag name from.
/// Returns:
/// - The tag name as a string.
pub fn get_tag_name(node: &tl::Node) -> String {
    // get the tag name
    if let Some(tag) = node.as_tag() {
        return String::from_utf8_lossy(tag.name().as_bytes()).to_string();
    }

    "".to_string()
}

/// Get the tag attributes from a node.
/// Arguments:
/// - node: The node to get the tag attributes from.
/// Returns:
/// - The tag attributes as a hashmap.
pub fn get_tag_attributes(node: &tl::Node) -> HashMap<String, String> {
    // get the tag attributes
    if let Some(tag) = node.as_tag() {
        return tag
            .attributes()
            .iter()
            .map(|(key, value)| {
                (
                    key.to_string(),
                    String::from_utf8_lossy(value.unwrap_or_default().as_bytes()).to_string(),
                )
            })
            .collect();
    }

    HashMap::new()
}

pub struct HtmlToMarkdownParser<'a> {
    config: ParserConfig,
    dom: tl::VDom<'a>,
    max_depth: usize,
}

impl<'a> HtmlToMarkdownParser<'a> {
    pub fn new(config: ParserConfig, html_input: &'a str) -> HtmlToMarkdownParser<'a> {
        // get the vdom by parsing
        dbg!(format!("Parsing HTML: {}", html_input.len()));
        let dom = dbg!(tl::parse(html_input, tl::ParserOptions::default()).unwrap());
        dbg!("Parsed HTML");

        HtmlToMarkdownParser {
            config,
            dom,
            max_depth: DEFAULT_MAX_DEPTH,
        }
    }

    pub fn parser(&self) -> &'a tl::Parser {
        self.dom.parser()
    }

    pub fn get_children(&self, node: &'a tl::Node) -> Vec<&'a tl::Node> {
        // get the children of a node
        let mut results: Vec<&'a tl::Node> = Vec::new();

        if let Some(children) = node.children() {
            for child in children.top().iter() {
                if let Some(child_node) = child.get(self.parser()) {
                    results.push(child_node);
                }
            }
        }

        results
    }

    pub fn get_first_selector(&self, selector: &str) -> Option<&tl::Node> {
        // get the first element by selector
        let result = self
            .dom
            .query_selector(selector)
            .and_then(|mut iter| iter.next());

        if result.is_none() {
            None
        } else {
            result?.get(self.parser())
        }
    }

    pub fn get_top_element(&self) -> Option<tl::Node> {
        // look for elements in this priority order:
        // - main or MAIN
        // - body or BODY
        // - html or HTML
        // set tag vec
        let tags = ["main", "body", "html"];
        for element in tags.iter() {
            // try lowercase
            if let Some(node) = self.get_first_selector(element) {
                return Some(node.clone());
            }

            // try uppercase
            if let Some(node) = self.get_first_selector(&element.to_ascii_uppercase()) {
                return Some(node.clone());
            }
        }

        None
    }

    pub fn parse_div_element(&self, node: &tl::Node, depth: usize) -> String {
        // stack overflow prevention
        if depth >= self.max_depth {
            return "".to_string();
        }

        // output how we are entering this
        let mut elements: Vec<String> = Vec::new();

        for child in self.get_children(&node) {
            let child_tag_name = get_tag_name(&child).to_ascii_lowercase();

            if self.config.exclude_tags.contains(&child_tag_name) {
                continue;
            }

            match child_tag_name.as_str() {
                "section" | "article" | "aside" | "header" | "footer" | "nav" => {
                    elements.push(format!(
                        "\n{}\n",
                        self.parse_block_element(&child, depth + 1)
                    ));
                }
                "div" => {
                    elements.push(format!("\n{}\n", self.parse_div_element(&child, depth + 1)));
                }
                "span" | "time" => {
                    elements.push(self.parse_inline_element(&child, depth + 1));
                }
                "p" => {
                    elements.push(format!("\n{}\n", self.parse_paragraph(&child, depth + 1)));
                }
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    elements.push(format!("\n{}\n", self.parse_heading(&child)));
                }
                "ul" | "ol" | "dl" | "dd" | "dt" => {
                    elements.push(self.parse_list(&child, depth + 1) + "\n");
                }
                "table" => {
                    elements.push(self.parse_table(&child) + "\n");
                }
                "blockquote" => {
                    elements.push(self.parse_blockquote(&child) + "\n");
                }
                "pre" => {
                    elements.push(self.parse_pre(&child) + "\n");
                }
                "code" => {
                    elements.push(self.parse_code(&child));
                }
                "em" => {
                    elements.push(self.parse_emphasis(&child));
                }
                "strong" => {
                    elements.push(self.parse_strong(&child));
                }
                "a" => {
                    elements.push(self.parse_link(&child));
                }
                "img" => {
                    elements.push(self.parse_image(&child));
                }
                "s" | "strike" | "del" => {
                    elements.push(self.parse_strikethrough(&child));
                }
                "br" => {
                    elements.push(self.parse_br(&child));
                }
                "hr" => {
                    elements.push(self.parse_hr(&child));
                }
                _ => {
                    if !child_tag_name.is_empty() {
                        // dbg!(format!("Unknown tag being treated as block: {}", child_tag_name));
                        elements.push(self.parse_block_element(&child, depth + 1) + "\n");
                    }
                }
            }
        }

        // trim empty elements and return joined
        elements
            .iter()
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn parse_pre_inline_element(&self, node: &tl::Node, depth: usize) -> String {
        let mut elements: Vec<String> = Vec::new();

        if let Some(text_node) = node.as_raw() {
            let element_text = text_node.as_utf8_str().to_string();
            elements.push(element_text.to_string());
        }

        for child in self.get_children(node) {
            let child_tag_name = get_tag_name(&child).to_ascii_lowercase();

            if self.config.exclude_tags.contains(&child_tag_name) {
                continue;
            }

            match child_tag_name.as_str() {
                "strong" | "b" => {
                    // parse strong
                    elements.push(format!(
                        "**{}**",
                        self.parse_pre_inline_element(&child, depth + 1).trim()
                    ))
                }
                "em" | "i" => {
                    // parse emphasis
                    elements.push(format!(
                        "*{}*",
                        self.parse_pre_inline_element(&child, depth + 1).trim()
                    ))
                }
                "s" | "strike" | "del" => {
                    // parse strikethrough
                    elements.push(format!(
                        "~~{}~~",
                        self.parse_pre_inline_element(&child, depth + 1).trim()
                    ))
                }
                "a" => elements.push(self.parse_link(&child)),
                "img" => elements.push(self.parse_image(&child)),
                "br" => elements.push(self.parse_br(&child)),
                "ul" | "ol" | "dl" | "dt" | "dd" => {
                    elements.push(self.parse_list(&child, 0) + "\n");
                }
                _ => elements.push(self.parse_pre_inline_element(&child, depth + 1)),
            }
        }

        // filter out empty elements and join
        elements
            .iter()
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn parse_inline_element(&self, node: &tl::Node, depth: usize) -> String {
        // stack overflow prevention
        if depth >= self.max_depth {
            return "".to_string();
        }

        let mut elements: Vec<String> = Vec::new();
        let tag_name = get_tag_name(&node).to_ascii_lowercase();

        if let Some(text_node) = node.as_raw() {
            let element_text = text_node.as_utf8_str().to_string();

            // remove newlines if it's not a pre or code tag
            if tag_name != "pre" && tag_name != "code" {
                elements.push(element_text.replace("\n", "").to_string());
            } else {
                elements.push(element_text.to_string());
            }
        }

        for child in self.get_children(node) {
            let child_tag_name = get_tag_name(&child).to_ascii_lowercase();

            if self.config.exclude_tags.contains(&child_tag_name) {
                continue;
            }

            match child_tag_name.as_str() {
                "p" => {
                    elements.push(format!(
                        "\n{}\n",
                        self.parse_inline_element(&child, depth + 1)
                    ));
                }
                "strong" | "b" => {
                    // parse strong
                    elements.push(format!(
                        "**{}**",
                        self.parse_inline_element(&child, depth + 1).trim()
                    ))
                }
                "em" | "i" => {
                    // parse emphasis
                    elements.push(format!(
                        "*{}*",
                        self.parse_inline_element(&child, depth + 1).trim()
                    ))
                }
                "s" | "strike" | "del" => {
                    // parse strikethrough
                    elements.push(format!(
                        "~~{}~~",
                        self.parse_inline_element(&child, depth + 1).trim()
                    ))
                }
                "a" => elements.push(self.parse_link(&child)),
                "img" => elements.push(self.parse_image(&child)),
                "br" => elements.push(self.parse_br(&child)),
                "ul" | "ol" | "li" | "dl" | "dt" | "dd" => {
                    elements.push(self.parse_list(&child, 0) + "\n");
                }
                _ => elements.push(self.parse_inline_element(&child, depth + 1)),
            }
        }

        // filter out empty elements and join
        elements
            .iter()
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn parse_heading(&self, node: &tl::Node) -> String {
        let tag_name = get_tag_name(&node);
        let tag_text = self.parse_inline_element(node, 0);

        match tag_name.as_str() {
            "h1" => format!("\n\n# {}\n\n", tag_text),
            "h2" => format!("\n\n## {}\n\n", tag_text),
            "h3" => format!("\n\n### {}\n\n", tag_text),
            "h4" => format!("\n\n#### {}\n\n", tag_text),
            "h5" => format!("\n\n##### {}\n\n", tag_text),
            "h6" => format!("\n\n###### {}\n\n", tag_text),
            _ => tag_text,
        }
    }

    pub fn parse_paragraph(&self, node: &tl::Node, depth: usize) -> String {
        // stack overflow prevention
        if depth >= self.max_depth {
            return "".to_string();
        }
        self.parse_inline_element(node, depth + 1)
    }

    pub fn parse_list(&self, node: &tl::Node, level: usize) -> String {
        let mut list_items: Vec<String> = Vec::new();

        for child in self.get_children(node) {
            let child_tag_name = get_tag_name(&child).to_ascii_lowercase();

            if self.config.exclude_tags.contains(&child_tag_name) {
                continue;
            }

            // get indent level based on level
            let list_value;
            let indent = " ".repeat(level * 2);

            if child_tag_name == "li" || child_tag_name == "dt" || child_tag_name == "dd" {
                // parse all child elements but separate ul/ol from inline for others
                let mut list_item_elements: Vec<String> = Vec::new();
                let mut sublist = true;

                for list_child in self.get_children(&child) {
                    let list_child_tag_name = get_tag_name(&list_child);

                    if self.config.exclude_tags.contains(&list_child_tag_name) {
                        continue;
                    }

                    match list_child_tag_name.as_str() {
                        "ul" | "ol" | "dl" | "dt" | "dd" => {
                            list_item_elements.push(
                                self.parse_list(&list_child, level + 1)
                                    .trim_ascii_end()
                                    .to_string(),
                            );
                        }
                        _ => {
                            list_item_elements.push(self.parse_inline_element(&list_child, 0));
                            sublist = false;
                        }
                    }
                }

                if sublist {
                    // indent is handled inside the next level
                    list_value = format!("{}", list_item_elements.join(" "));
                } else {
                    list_value = format!("{}- {}", indent, list_item_elements.join(" "));
                }
            } else {
                list_value = format!("{}{}", indent, self.parse_inline_element(&child, 0));
            }

            // append if not empty
            if !list_value.trim().is_empty() {
                list_items.push(list_value);
            }
        }

        list_items.join("\n")
    }

    pub fn parse_table(&self, node: &tl::Node) -> String {
        let rows: Vec<&tl::Node> = self
            .get_children(node)
            .into_iter()
            .filter(|child| get_tag_name(child) == "tr")
            .collect();

        if rows.is_empty() {
            return String::new();
        }

        let header = rows[0];
        let body = &rows[1..];

        // Convert header and calculate column widths
        let header_cells: Vec<String> = self
            .get_children(header)
            .into_iter()
            .filter(|cell| get_tag_name(cell) == "th" || get_tag_name(cell) == "td")
            .map(|cell| self.parse_inline_element(cell, 0).trim().to_string())
            .collect();

        let mut column_widths: Vec<usize> = header_cells.iter().map(|cell| cell.len()).collect();

        // Update column widths based on body cells
        for row in body {
            let cells: Vec<String> = self
                .get_children(row)
                .into_iter()
                .filter(|cell| get_tag_name(cell) == "td")
                .map(|cell| self.parse_inline_element(cell, 0).trim().to_string())
                .collect();

            for (i, cell) in cells.iter().enumerate() {
                if i < column_widths.len() {
                    column_widths[i] = column_widths[i].max(cell.len());
                }
            }
        }

        // Create header row
        let header_row = format!(
            "|{}|",
            header_cells
                .iter()
                .enumerate()
                .map(|(i, cell)| format!(" {:<width$} ", cell, width = column_widths[i]))
                .collect::<Vec<String>>()
                .join("|")
        );

        // Create separator row
        let separator_row = format!(
            "|{}|",
            column_widths
                .iter()
                .map(|&width| format!("{:-<width$}", "", width = width + 2))
                .collect::<Vec<String>>()
                .join("|")
        );

        // Create body rows
        let body_rows: Vec<String> = body
            .iter()
            .map(|row| {
                let cells: Vec<String> = self
                    .get_children(row)
                    .into_iter()
                    .filter(|cell| get_tag_name(cell) == "td")
                    .map(|cell| self.parse_inline_element(cell, 0).trim().to_string())
                    .collect();

                format!(
                    "|{}|",
                    cells
                        .iter()
                        .enumerate()
                        .map(|(i, cell)| format!(
                            " {:<width$} ",
                            cell,
                            width = column_widths.get(i).unwrap_or(&0)
                        ))
                        .collect::<Vec<String>>()
                        .join("|")
                )
            })
            .collect();

        // Combine all rows
        let mut markdown_table = vec![header_row, separator_row];
        markdown_table.extend(body_rows);

        markdown_table.join("\n")
    }

    pub fn parse_blockquote(&self, node: &tl::Node) -> String {
        format!("> {}\n", self.parse_inline_element(node, 0))
    }

    pub fn parse_pre(&self, node: &tl::Node) -> String {
        let content = self.parse_pre_inline_element(node, 0);
        format!("```\n{}\n```\n", content.trim_end())
    }

    pub fn parse_code(&self, node: &tl::Node) -> String {
        format!("`{}`", self.parse_inline_element(node, 0))
    }

    pub fn parse_emphasis(&self, node: &tl::Node) -> String {
        format!("*{}*", self.parse_inline_element(node, 0))
    }

    pub fn parse_strong(&self, node: &tl::Node) -> String {
        format!("**{}**", self.parse_inline_element(node, 0))
    }

    pub fn parse_link(&self, node: &tl::Node) -> String {
        let tag_text = self.parse_inline_element(node, 0);

        if self.config.output_links {
            let tag_attributes = get_tag_attributes(node);
            if tag_attributes.contains_key("href") {
                format!("[{}]({})", tag_text, tag_attributes.get("href").unwrap())
            } else {
                tag_text
            }
        } else {
            tag_text
        }
    }

    pub fn parse_image(&self, node: &tl::Node) -> String {
        if self.config.output_images {
            let tag_attributes = get_tag_attributes(node);

            if tag_attributes.contains_key("src") {
                format!(
                    "![{}]({})",
                    tag_attributes.get("alt").unwrap_or(&"".to_string()),
                    tag_attributes.get("src").unwrap()
                )
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    }

    pub fn parse_strikethrough(&self, node: &tl::Node) -> String {
        format!("~~{}~~", self.parse_inline_element(node, 0))
    }

    pub fn parse_br(&self, _node: &tl::Node) -> String {
        "\n".to_string()
    }

    pub fn parse_hr(&self, _node: &tl::Node) -> String {
        "\n\n----\n\n".to_string()
    }

    pub fn parse_block_element(&self, node: &tl::Node, depth: usize) -> String {
        // initialize the block markdown strings
        let mut blocks: Vec<String> = Vec::new();

        // iterate through children
        for child_handle in node.children().unwrap().top().iter() {
            if let Some(child) = child_handle.get(self.parser()) {
                let child_tag_name = get_tag_name(&child).to_ascii_lowercase();

                // skip or stop if the tag is in the exclude list
                if self.config.exclude_tags.contains(&child_tag_name) {
                    continue;
                }

                match child_tag_name.as_str() {
                    "body" | "main" | "section" | "article" | "aside" | "header" | "footer"
                    | "nav" => {
                        blocks.push(self.parse_block_element(&child, depth + 1) + "\n");
                    }
                    "div" => {
                        blocks.push(self.parse_div_element(&child, depth + 1) + "\n");
                    }
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                        blocks.push(self.parse_heading(&child) + "\n");
                    }
                    "p" => {
                        blocks.push(format!("\n{}\n", self.parse_paragraph(&child, depth + 1)));
                    }
                    "ul" | "ol" | "dl" | "dd" | "dt" => {
                        blocks.push(self.parse_list(&child, 0) + "\n");
                    }
                    "table" => {
                        blocks.push(self.parse_table(&child) + "\n");
                    }
                    "blockquote" => {
                        blocks.push(self.parse_blockquote(&child) + "\n");
                    }
                    "pre" => {
                        blocks.push(self.parse_pre(&child) + "\n");
                    }
                    "code" => {
                        blocks.push(self.parse_code(&child));
                    }
                    "em" => {
                        blocks.push(self.parse_emphasis(&child));
                    }
                    "strong" => {
                        blocks.push(self.parse_strong(&child));
                    }
                    "center" => {
                        blocks.push(self.parse_strong(&child));
                    }
                    "a" => {
                        blocks.push(self.parse_link(&child));
                    }
                    "img" => {
                        blocks.push(self.parse_image(&child));
                    }
                    "s" | "strike" | "del" => {
                        blocks.push(self.parse_strikethrough(&child));
                    }
                    "br" => {
                        blocks.push(self.parse_br(&child));
                    }
                    "hr" => {
                        blocks.push(self.parse_hr(&child));
                    }
                    _ => {
                        if !child_tag_name.is_empty() {
                            // dbg!(format!("Unknown tag being treated as block: {}", child_tag_name));
                            blocks.push(self.parse_block_element(&child, depth + 1) + "\n");
                        }
                    }
                }
            }
        }

        // remove empty blocks and then join them with double newline
        blocks
            .iter()
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn to_markdown(&self) -> String {
        // get the top element
        if let Some(top_element) = self.get_top_element() {
            // get the block elements
            normalize_markdown(&self.parse_block_element(&top_element, 0).trim().to_string()) + "\n"
        } else {
            "".to_string()
        }
    }
}

/// Normalize the plain text output.
/// Arguments:
/// - text: The text to clean up.
/// Returns:
/// - The cleaned up text.
pub fn normalize_text(text: &str) -> String {
    // handle newlines, &nbsp, and similar issues
    RE_CLEAN_NEWLINE
        .replace_all(text, "\n\n")
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("\u{00A0}", " ")
        .replace("\u{200B}", "")
        .replace("\u{200C}", "")
        .to_string()
}

pub struct HtmlToPlainTextParser<'a> {
    config: ParserConfig,
    dom: tl::VDom<'a>,
    max_depth: usize,
}

impl<'a> HtmlToPlainTextParser<'a> {
    pub fn new(config: ParserConfig, html_input: &'a str) -> HtmlToPlainTextParser<'a> {
        // Parse the HTML input into a virtual DOM
        let dom = tl::parse(html_input, tl::ParserOptions::default()).unwrap();

        HtmlToPlainTextParser {
            config,
            dom,
            max_depth: DEFAULT_MAX_DEPTH,
        }
    }

    pub fn parser(&self) -> &'a tl::Parser {
        self.dom.parser()
    }

    pub fn get_children(&self, node: &'a tl::Node) -> Vec<&'a tl::Node> {
        // Get the children of a node
        let mut results: Vec<&'a tl::Node> = Vec::new();

        if let Some(children) = node.children() {
            for child in children.top().iter() {
                if let Some(child_node) = child.get(self.parser()) {
                    results.push(child_node);
                }
            }
        }

        results
    }

    pub fn get_first_selector(&self, selector: &str) -> Option<&tl::Node> {
        // Get the first element by selector
        let result = self
            .dom
            .query_selector(selector)
            .and_then(|mut iter| iter.next());

        if result.is_none() {
            None
        } else {
            result?.get(self.parser())
        }
    }

    pub fn get_top_element(&self) -> Option<tl::Node> {
        // look for elements in this priority order:
        // - main or MAIN
        // - body or BODY
        // - html or HTML
        // set tag vec
        let tags = ["main", "body", "html"];
        for element in tags.iter() {
            // try lowercase
            if let Some(node) = self.get_first_selector(element) {
                return Some(node.clone());
            }

            // try uppercase
            if let Some(node) = self.get_first_selector(&element.to_ascii_uppercase()) {
                return Some(node.clone());
            }
        }

        None
    }

    pub fn parse_div_element(&self, node: &tl::Node, depth: usize) -> String {
        // stack overflow prevention
        if depth >= self.max_depth {
            return "".to_string();
        }

        let mut elements: Vec<String> = Vec::new();

        for child in self.get_children(&node) {
            let child_tag_name = get_tag_name(&child).to_ascii_lowercase();

            if self.config.exclude_tags.contains(&child_tag_name) {
                continue;
            }

            match child_tag_name.as_str() {
                "section" | "article" | "aside" | "header" | "footer" | "nav" => {
                    elements.push(format!(
                        "\n{}\n",
                        self.parse_block_element(&child, depth + 1)
                    ));
                }
                "div" => {
                    elements.push(format!("\n{}\n", self.parse_div_element(&child, depth + 1)));
                }
                "span" | "time" => {
                    elements.push(self.parse_inline_element(&child, depth + 1));
                }
                "p" => {
                    elements.push(format!("\n{}\n", self.parse_paragraph(&child)));
                }
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    elements.push(self.parse_heading(&child) + "\n");
                }
                "ul" | "ol" | "dl" | "dd" | "dt" => {
                    elements.push(self.parse_list(&child, 0) + "\n");
                }
                "table" => {
                    elements.push(self.parse_table(&child) + "\n");
                }
                "blockquote" => {
                    elements.push(self.parse_blockquote(&child) + "\n");
                }
                "pre" => {
                    elements.push(self.parse_pre(&child) + "\n");
                }
                "code" => {
                    elements.push(self.parse_code(&child));
                }
                "em" => {
                    elements.push(self.parse_emphasis(&child));
                }
                "strong" => {
                    elements.push(self.parse_strong(&child));
                }
                "a" => {
                    elements.push(self.parse_link(&child));
                }
                "img" => {
                    elements.push(self.parse_image(&child));
                }
                "s" | "strike" | "del" => {
                    elements.push(self.parse_strikethrough(&child));
                }
                "br" => {
                    elements.push(self.parse_br(&child));
                }
                "hr" => {
                    elements.push(self.parse_hr(&child));
                }
                _ => {
                    if !child_tag_name.is_empty() {
                        // dbg!(format!("Unknown tag being treated as block: {}", child_tag_name));
                        elements.push(self.parse_block_element(&child, depth + 1) + "\n");
                    }
                }
            }
        }

        // Trim empty elements and return joined
        elements
            .iter()
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn parse_pre_inline_element(&self, node: &tl::Node, depth: usize) -> String {
        // stack overflow prevention
        if depth >= self.max_depth {
            return "".to_string();
        }

        let mut elements: Vec<String> = Vec::new();

        if let Some(text_node) = node.as_raw() {
            let element_text = text_node.as_utf8_str().to_string();
            elements.push(element_text.to_string());
        }

        for child in self.get_children(node) {
            let child_tag_name = get_tag_name(&child).to_ascii_lowercase();

            if self.config.exclude_tags.contains(&child_tag_name) {
                continue;
            }

            match child_tag_name.as_str() {
                "strong" | "b" => {
                    // parse strong
                    elements.push(format!(
                        "{}",
                        self.parse_pre_inline_element(&child, depth + 1).trim()
                    ))
                }
                "em" | "i" => {
                    // parse emphasis
                    elements.push(format!(
                        "{}",
                        self.parse_pre_inline_element(&child, depth + 1).trim()
                    ))
                }
                "s" | "strike" | "del" => {
                    // parse strikethrough
                    elements.push(format!(
                        "{}",
                        self.parse_pre_inline_element(&child, depth + 1).trim()
                    ))
                }
                "a" => elements.push(self.parse_link(&child)),
                "img" => elements.push(self.parse_image(&child)),
                "br" => elements.push(self.parse_br(&child)),
                "ul" | "ol" | "dl" | "dt" | "dd" => {
                    elements.push(self.parse_list(&child, 0) + "\n");
                }
                _ => elements.push(self.parse_pre_inline_element(&child, depth + 1)),
            }
        }

        // filter out empty elements and join
        elements
            .iter()
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn parse_inline_element(&self, node: &tl::Node, depth: usize) -> String {
        // stack overflow prevention
        if depth >= self.max_depth {
            return "".to_string();
        }

        let mut elements: Vec<String> = Vec::new();
        let tag_name = get_tag_name(&node).to_ascii_lowercase();

        if let Some(text_node) = node.as_raw() {
            let element_text = text_node.as_utf8_str().to_string();

            // remove newlines if it's not a pre or code tag
            if tag_name != "pre" && tag_name != "code" {
                elements.push(element_text.replace("\n", "").to_string());
            } else {
                elements.push(element_text.to_string());
            }
        }

        for child in self.get_children(node) {
            let child_tag_name = get_tag_name(&child).to_ascii_lowercase();

            if self.config.exclude_tags.contains(&child_tag_name) {
                continue;
            }

            match child_tag_name.as_str() {
                "p" => {
                    elements.push(format!(
                        "\n{}\n",
                        self.parse_inline_element(&child, depth + 1)
                    ));
                }
                "a" => elements.push(self.parse_link(&child)),
                "img" => elements.push(self.parse_image(&child)),
                "br" => elements.push(self.parse_br(&child)),
                "ul" | "ol" | "li" | "dl" | "dt" | "dd" => {
                    elements.push(self.parse_list(&child, 0));
                    elements.push("\n".to_string());
                }
                _ => elements.push(self.parse_inline_element(&child, depth + 1)),
            }
        }

        // Filter out empty elements and join
        elements
            .iter()
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn parse_heading(&self, node: &tl::Node) -> String {
        let tag_text = self.parse_inline_element(node, 0);
        format!("\n\n{}\n\n", tag_text)
    }

    pub fn parse_paragraph(&self, node: &tl::Node) -> String {
        format!("{}\n", self.parse_inline_element(node, 0))
    }

    pub fn parse_list(&self, node: &tl::Node, level: usize) -> String {
        let mut list_items: Vec<String> = Vec::new();

        for child in self.get_children(node) {
            let child_tag_name = get_tag_name(&child).to_ascii_lowercase();

            if self.config.exclude_tags.contains(&child_tag_name) {
                continue;
            }

            let indent = "  ".repeat(level);

            if child_tag_name == "li" || child_tag_name == "dt" || child_tag_name == "dd" {
                let mut list_item_elements: Vec<String> = Vec::new();
                let mut sublist = false;

                for list_child in self.get_children(&child) {
                    let list_child_tag_name = get_tag_name(&list_child);

                    if self.config.exclude_tags.contains(&list_child_tag_name) {
                        continue;
                    }

                    match list_child_tag_name.as_str() {
                        "ul" | "ol" | "dl" | "dt" | "dd" => {
                            list_item_elements
                                .push(self.parse_list(&list_child, level + 1).trim().to_string());
                            sublist = true;
                        }
                        _ => {
                            list_item_elements.push(self.parse_inline_element(&list_child, 0));
                        }
                    }
                }

                let list_item = if sublist {
                    list_item_elements.join("\n")
                } else {
                    format!("{}- {}", indent, list_item_elements.join(" "))
                };

                if !list_item.trim().is_empty() {
                    list_items.push(list_item);
                }
            }
        }

        list_items.join("\n")
    }

    pub fn parse_table(&self, node: &tl::Node) -> String {
        let mut table_text = String::new();

        for row in self
            .get_children(node)
            .iter()
            .filter(|child| get_tag_name(child) == "tr")
        {
            let mut row_text = String::new();
            for cell in self
                .get_children(row)
                .iter()
                .filter(|cell| get_tag_name(cell) == "th" || get_tag_name(cell) == "td")
            {
                let cell_text = self.parse_inline_element(cell, 0).trim().to_string();
                row_text.push_str(&format!("{}\t", cell_text));
            }
            table_text.push_str(&format!("{}\n", row_text.trim_end()));
        }

        table_text
    }

    pub fn parse_blockquote(&self, node: &tl::Node) -> String {
        let quote = self.parse_inline_element(node, 0);
        format!("> {}\n", quote)
    }

    pub fn parse_pre(&self, node: &tl::Node) -> String {
        let content = self.parse_pre_inline_element(node, 0);
        format!("\n{}\n", content.trim_end())
    }

    pub fn parse_code(&self, node: &tl::Node) -> String {
        self.parse_inline_element(node, 0)
    }

    pub fn parse_emphasis(&self, node: &tl::Node) -> String {
        self.parse_inline_element(node, 0)
    }

    pub fn parse_strong(&self, node: &tl::Node) -> String {
        self.parse_inline_element(node, 0)
    }

    pub fn parse_link(&self, node: &tl::Node) -> String {
        self.parse_inline_element(node, 0)
    }

    pub fn parse_image(&self, node: &tl::Node) -> String {
        let tag_attributes = get_tag_attributes(node);
        if let Some(alt) = tag_attributes.get("alt") {
            alt.clone()
        } else {
            "".to_string()
        }
    }

    pub fn parse_strikethrough(&self, node: &tl::Node) -> String {
        self.parse_inline_element(node, 0)
    }

    pub fn parse_br(&self, _node: &tl::Node) -> String {
        "\n".to_string()
    }

    pub fn parse_hr(&self, _node: &tl::Node) -> String {
        "\n---\n".to_string()
    }

    pub fn parse_block_element(&self, node: &tl::Node, depth: usize) -> String {
        // stack overflow prevention
        if depth >= self.max_depth {
            return "".to_string();
        }

        // Initialize the block text strings
        let mut blocks: Vec<String> = Vec::new();

        // Iterate through children
        for child_handle in node.children().unwrap().top().iter() {
            if let Some(child) = child_handle.get(self.parser()) {
                let child_tag_name = get_tag_name(&child).to_ascii_lowercase();

                // Skip if the tag is in the exclude list
                if self.config.exclude_tags.contains(&child_tag_name) {
                    continue;
                }

                match child_tag_name.as_str() {
                    "body" | "main" | "section" | "article" | "aside" | "header" | "footer"
                    | "nav" => {
                        blocks.push(format!(
                            "\n{}\n",
                            self.parse_block_element(&child, depth + 1)
                        ));
                    }
                    "div" => {
                        blocks.push(format!("\n{}\n", self.parse_div_element(&child, depth + 1)));
                    }
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                        blocks.push(format!("\n{}\n", self.parse_heading(&child)));
                    }
                    "p" => {
                        blocks.push(format!("\n{}\n", self.parse_paragraph(&child)));
                    }
                    "ul" | "ol" | "dl" | "dt" | "dd" => {
                        blocks.push(self.parse_list(&child, 0) + "\n");
                    }
                    "table" => {
                        blocks.push(self.parse_table(&child) + "\n");
                    }
                    "blockquote" => {
                        blocks.push(self.parse_blockquote(&child) + "\n");
                    }
                    "pre" => {
                        blocks.push(self.parse_pre(&child) + "\n");
                    }
                    "code" => {
                        blocks.push(self.parse_code(&child));
                    }
                    "em" => {
                        blocks.push(self.parse_emphasis(&child));
                    }
                    "strong" => {
                        blocks.push(self.parse_strong(&child));
                    }
                    "center" => {
                        blocks.push(self.parse_strong(&child));
                    }
                    "a" => {
                        blocks.push(self.parse_link(&child));
                    }
                    "img" => {
                        blocks.push(self.parse_image(&child));
                    }
                    "s" | "strike" | "del" => {
                        blocks.push(self.parse_strikethrough(&child));
                    }
                    "br" => {
                        blocks.push(self.parse_br(&child));
                    }
                    "hr" => {
                        blocks.push(self.parse_hr(&child));
                    }
                    _ => {
                        if !child_tag_name.is_empty() {
                            // dbg!(format!("Unknown tag being treated as block: {}", child_tag_name));
                            blocks.push(self.parse_block_element(&child, depth + 1) + "\n");
                        }
                    }
                }
            }
        }

        // Remove empty blocks and then join them with double newline
        blocks
            .iter()
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn to_plain_text(&self) -> String {
        // Get the top element
        if let Some(top_element) = self.get_top_element() {
            // Get the block elements
            normalize_text(&self.parse_block_element(&top_element, 0).trim().to_string()) + "\n"
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path;

    fn get_sample_single_p() -> String {
        r#"<html><body><main><p>Hello</p></main></body></html>"#.to_string()
    }

    fn get_sample_formatted_p() -> String {
        r#"<html><body><main><p>Hello <strong>world</strong>.</p><p>This is a test.</p></main></body></html>"#.to_string()
    }

    fn get_sample_double_div() -> String {
        r#"<html><body><div><p>This is a test.</p></div><div><span><em>You</em> are not <a href="/">here</a> today.</span></div></body</html>"#.to_string()
    }

    // one with a basic bulleted list
    fn get_sample_simple_list() -> String {
        r#"<html><body><main><ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul></main></body></html>"#.to_string()
    }

    // nested list
    fn get_sample_nested_list() -> String {
        r#"<html><body><main><ul><li>Item 1</li><li><ul><li>Item 2</li><li>Item 3</li></ul></li><li>Item 4</li></ul></main></body></html>"#.to_string()
    }

    fn get_sample_simple_table() -> String {
        r#"<html><body><table>
            <tr><th>Header 1</th><th>Header 2</th></tr>
            <tr><td>Row 1, Cell 1</td><td>Row 1, Cell 2</td></tr>
            <tr><td>Row 2, Cell 1</td><td>Row 2, Cell 2</td></tr>
        </table></body></html>"#
            .to_string()
    }

    // test get_first_selector
    #[test]
    fn test_get_first_selector() {
        let sample = get_sample_single_p();
        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, true, true), &sample);

        let result = parser.get_first_selector("main");
        assert!(result.is_some());
        assert_eq!(get_tag_name(&result.unwrap()), "main");
    }

    // test markdown
    #[test]
    fn test_markdown_single_p() {
        let sample = get_sample_single_p();
        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, true, true), &sample);

        let result = parser.to_markdown();
        assert_eq!(result, "Hello\n");
    }

    // test markdown
    #[test]
    fn test_markdown_formatted_p() {
        let sample = get_sample_formatted_p();
        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, true, true), &sample);

        let result = parser.to_markdown();
        assert_eq!(result, "Hello **world**.\n\nThis is a test.\n");
    }

    // test markdown
    #[test]
    fn test_markdown_double_div() {
        let sample = get_sample_double_div();
        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, true, true), &sample);

        let result = parser.to_markdown();
        assert_eq!(
            result,
            "This is a test.\n\n*You* are not [here](/) today.\n"
        );
    }

    // test markdown
    #[test]
    fn test_markdown_simple_list() {
        let sample = get_sample_simple_list();
        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, true, true), &sample);

        let result = parser.to_markdown();
        assert_eq!(result, "- Item 1\n- Item 2\n- Item 3\n");
    }

    // test markdown
    #[test]
    fn test_markdown_nested_list() {
        let sample = get_sample_nested_list();
        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, true, true), &sample);

        let result = parser.to_markdown();
        assert_eq!(result, "- Item 1\n  - Item 2\n  - Item 3\n- Item 4\n");
    }

    #[test]
    fn test_markdown_list_newlines() {
        let sample = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Page Title</title>
    </head>
    <body>
        <h1>This is a Heading</h1>
        <p>This is a paragraph.</p>
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
        </ul>
    </body>
</html>
"#;

        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, true, true), &sample);
        let result = parser.to_markdown();

        // check that we have \n- Item 1\n-Item 2\n" in the result
        assert!(result.contains("\n- Item 1\n- Item 2\n"));
    }

    #[test]
    fn test_markdown_simple_table() {
        let sample = get_sample_simple_table();
        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, true, true), &sample);

        let result = parser.to_markdown();
        let expected = r#"
| Header 1      | Header 2      |
|---------------|---------------|
| Row 1, Cell 1 | Row 1, Cell 2 |
| Row 2, Cell 1 | Row 2, Cell 2 |
"#
        .trim_ascii_start()
        .to_string();
        assert_eq!(result, expected);
    }

    // test file1.html
    #[test]
    fn test_file1() {
        // load from CARGO_MANIFEST_DIR
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let file_path = path::Path::new(manifest_dir).join("resources/file1.html");
        let sample = fs::read_to_string(file_path).unwrap();

        // parse the file
        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, true, true), &sample);
        let result = dbg!(parser.to_markdown());

        assert!(result.contains("\n\n# Our blog\n\nWhat we're reading, thinking, and doing.\n\n[press release - ](/blog/tags/press release)"));
    }

    #[test]
    fn test_treasury_001() {
        // load from CARGO_MANIFEST_DIR
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let file_path = path::Path::new(manifest_dir).join("resources/treasury.html");
        let sample = fs::read_to_string(file_path).unwrap();

        // parse the file
        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, true, true), &sample);
        let result = dbg!(parser.to_markdown());

        // check that we don't have #block-content-homepage-hero in the output from <script> or <style>
        assert!(!result.contains("#block-content-homepage-hero"));
        assert!(result.contains("U.S. Department of the Treasury Announces Maine Will Join IRS Direct File for Filing Season 2025"));
    }

    #[test]
    fn test_dol_001() {
        // load from CARGO_MANIFEST_DIR
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let file_path = path::Path::new(manifest_dir).join("resources/entrepre.htm");
        let sample = fs::read_to_string(file_path).unwrap();

        // parse the file
        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, true, true), &sample);
        let result = dbg!(parser.to_markdown());

        // check for 'Microenterprise organizations include capital' in the markdown
        assert!(result.contains("Microenterprise organizations include capital"));
    }

    // test no link
    #[test]
    fn test_no_link() {
        let sample = get_sample_double_div();
        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, false, true), &sample);

        let result = parser.to_markdown();
        assert_eq!(result, "This is a test.\n\n*You* are not here today.\n");
    }

    // test text for dol 001
    #[test]
    fn test_text_dol_001() {
        // load from CARGO_MANIFEST_DIR
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let file_path = path::Path::new(manifest_dir).join("resources/entrepre.htm");
        let sample = fs::read_to_string(file_path).unwrap();

        // parse the file
        let parser = HtmlToPlainTextParser::new(ParserConfig::new(None, true, true), &sample);
        let result = dbg!(parser.to_plain_text());

        // check for 'Microenterprise organizations include capital' in the markdown
        assert!(result.contains("Microenterprise organizations include capital"));
    }

    #[test]
    fn test_reg_doc_001() {
        // load from CARGO_MANIFEST_DIR
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let file_path = path::Path::new(manifest_dir).join("resources/reg_doc_001.html");
        let sample = fs::read_to_string(file_path).unwrap();

        // parse the file
        let parser = HtmlToPlainTextParser::new(ParserConfig::new(None, true, true), &sample);
        let result = dbg!(parser.to_plain_text());

        // check for 'This section of the FEDERAL REGISTER' in the markdown
        assert!(result.contains(
            "[Federal Register: June 15, 2010 (Volume 75, Number 114)]\n[Proposed Rules]\n"
        ));
    }

    #[test]
    fn test_segfault_001_text() {
        // load from CARGO_MANIFEST_DIR
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let file_path = path::Path::new(manifest_dir).join("resources/segfault001.html");
        let sample = fs::read_to_string(file_path).unwrap();

        // parse the file
        let parser = HtmlToPlainTextParser::new(ParserConfig::new(None, true, true), &sample);
        let result = dbg!(parser.to_plain_text());

        assert!(result.contains("resulting in the acquisition of 99.99% of the capital"));
    }

    #[test]
    fn test_segfault_001_md() {
        // load from CARGO_MANIFEST_DIR
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let file_path = path::Path::new(manifest_dir).join("resources/segfault001.html");
        let sample = fs::read_to_string(file_path).unwrap();

        // parse the file
        let parser = HtmlToMarkdownParser::new(ParserConfig::new(None, true, true), &sample);
        let result = dbg!(parser.to_markdown());

        assert!(result.contains("resulting in the acquisition of 99.99% of the capital"));
    }
}
