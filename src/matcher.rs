use regex::Regex;
use crate::task::StringMatcher;

pub fn matches(value: &str, matcher: &StringMatcher) -> bool {
    let ignore_case = matcher.ignore_case.unwrap_or(false);
    let val = if ignore_case { value.to_lowercase() } else { value.to_string() };
    if let Some(exact) = &matcher.exact {
        return val == (if ignore_case { exact.to_lowercase() } else { exact.clone() });
    }
    if let Some(prefix) = &matcher.prefix {
        return val.starts_with(&if ignore_case { prefix.to_lowercase() } else { prefix.clone() });
    }
    if let Some(suffix) = &matcher.suffix {
        return val.ends_with(&if ignore_case { suffix.to_lowercase() } else { suffix.clone() });
    }
    if let Some(contains) = &matcher.contains {
        return val.contains(&if ignore_case { contains.to_lowercase() } else { contains.clone() });
    }
    if let Some(regex) = &matcher.safe_regex {
        let re = Regex::new(regex).unwrap();
        return re.is_match(&val);
    }
    false
} 