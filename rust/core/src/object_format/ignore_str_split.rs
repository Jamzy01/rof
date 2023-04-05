#[derive(Clone)]
pub enum SplitIgnoreRuleType {
    PAIR(char), /* Won't split when within a pair of this character, eg single or double quotation marks */
    NEST(
        char, /* Start new nest, eg opening curly bracker */
        char, /* End nest, eg closing curly bracket */
    ), /* Won't split when within a nest of characters, difference from pair is that it supports multiple nests, and will only allowing splitting when all of those nests have been closed */
}

#[derive(Clone)]
pub struct SplitIgnoreRule {
    split_ignore_rule_type: SplitIgnoreRuleType,
    within_pair: bool,
    nest_index: usize,
    encapsulates_raw_text: bool,
}

impl SplitIgnoreRule {
    pub fn new(split_ignore_rule_type: SplitIgnoreRuleType) -> Self {
        Self {
            split_ignore_rule_type,
            within_pair: false,
            nest_index: 0,
            encapsulates_raw_text: false,
        }
    }

    // Defines whether other split ignore rules should be ignored and whether escape characters can be used to ignore all rule checks, and all split checks

    pub fn set_ecapsulates_raw_text(mut self, encapsulates_raw_text: bool) -> Self {
        self.encapsulates_raw_text = encapsulates_raw_text;

        self
    }

    pub fn read_char(&mut self, character: char) {
        match self.split_ignore_rule_type {
            SplitIgnoreRuleType::PAIR(pair_character) => {
                if character == pair_character {
                    self.within_pair = !self.within_pair;
                }
            }
            SplitIgnoreRuleType::NEST(nest_start, nest_end) => {
                if character == nest_start {
                    self.nest_index += 1;
                }

                if character == nest_end {
                    if self.nest_index > 0 {
                        self.nest_index -= 1;
                    }
                }
            }
        }
    }

    pub fn should_ignore(&self) -> bool {
        self.within_pair || self.nest_index > 0
    }
}

// Used for splitting struct properties at semicolons, but not splitting at the semicolons when within a pair of single or double quotes, escape sequences are also checked so that single and double quotes can be written in strings without confusing the compiler

fn ignoring_compliant_split_str_max_splits(
    input_str: &str,
    split_character: char,
    retain_backslashes: bool,
    ignore_rules: Vec<SplitIgnoreRule>,
    max_splits: Option<usize>,
) -> Vec<String> {
    let mut str_fragments: Vec<String> = Vec::new();

    let mut built_str_fragment: String = String::new();

    let mut ignore_rules = ignore_rules.clone();

    let mut char_iter = input_str.chars();

    let mut splits: usize = 0;

    loop {
        match char_iter.next() {
            Some(str_char) => {
                ignore_rules
                    .iter_mut()
                    .for_each(|ignore_rule| ignore_rule.read_char(str_char));

                if str_char == '\\' {
                    match char_iter.next() {
                        Some(escape_char) => {
                            if retain_backslashes {
                                built_str_fragment.push('\\');
                            }

                            built_str_fragment.push(escape_char);
                        }
                        None => (),
                    }
                } else if str_char == split_character
                    && ignore_rules
                        .iter()
                        .all(|ignore_rule| !ignore_rule.should_ignore())
                    && (max_splits.is_none() || splits < max_splits.unwrap())
                {
                    str_fragments.push(built_str_fragment.clone());

                    splits += 1;

                    built_str_fragment.clear();
                } else {
                    built_str_fragment.push(str_char);
                }
            }
            None => break,
        }
    }

    if !built_str_fragment.is_empty() {
        str_fragments.push(built_str_fragment);
    }

    str_fragments
}

pub fn ignoring_compliant_split_str<'a>(
    input_str: &str,
    split_character: char,
    retain_backslashes: bool,
    ignore_rules: Vec<SplitIgnoreRule>,
) -> Vec<String> {
    ignoring_compliant_split_str_max_splits(
        input_str,
        split_character,
        retain_backslashes,
        ignore_rules,
        None,
    )
}

pub fn ignoring_compliant_split_once(
    input_str: &str,
    split_character: char,
    retain_backslashes: bool,
    ignore_rules: Vec<SplitIgnoreRule>,
) -> Option<(String, String)> {
    let split_data = ignoring_compliant_split_str_max_splits(
        input_str,
        split_character,
        retain_backslashes,
        ignore_rules,
        Some(1),
    );

    match split_data.len() {
        2 => Some((
            split_data.get(0).unwrap().clone(),
            split_data.get(1).unwrap().clone(),
        )),
        _ => None,
    }
}
