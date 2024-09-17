use crate::{
    CallParenType, CollapseSimpleStatement, Config, IndentType, LineEndings, QuoteStyle,
    SortRequiresConfig,
};
use ec4rs::{
    properties_of,
    property::{EndOfLine, IndentSize, IndentStyle, MaxLineLen, TabWidth, UnknownValueError},
    rawvalue::RawValue,
    Error, Properties, PropertyKey, PropertyValue,
};
use std::path::Path;

// Extracted from ec4rs::property
macro_rules! property_choice {
    ($prop_id:ident, $name:literal; $(($variant:ident, $string:literal)),+) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        #[repr(u8)]
        pub enum $prop_id {$($variant),+}

        impl PropertyValue for $prop_id {
            const MAYBE_UNSET: bool = false;
            type Err = UnknownValueError;
            fn parse(raw: &RawValue) -> Result<Self, Self::Err> {
                match raw.into_str().to_lowercase().as_str() {
                    $($string => Ok($prop_id::$variant),)+
                    _ => Err(UnknownValueError)
                }
            }
        }

        impl From<$prop_id> for RawValue {
            fn from(val: $prop_id) -> RawValue {
                match val {
                    $($prop_id::$variant => RawValue::from($string)),*
                }
            }
        }

        impl PropertyKey for $prop_id {
            fn key() -> &'static str {$name}
        }

        impl std::fmt::Display for $prop_id {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $($prop_id::$variant => $string),*
                })
            }
        }
    }
}

property_choice! {
    QuoteTypeChoice, "quote_type";
    (Double, "double"),
    (Single, "single"),
    (Auto, "auto")
}

property_choice! {
    CallParenthesesChoice, "call_parentheses";
    (Always, "always"),
    (NoSingleString, "nosinglestring"),
    (NoSingleTable, "nosingletable"),
    (None, "none")
}

property_choice! {
    CollapseSimpleStatementChoice, "collapse_simple_statement";
    (Never, "never"),
    (FunctionOnly, "functiononly"),
    (ConditionalOnly, "conditionalonly"),
    (Always, "always")
}

property_choice! {
    SortRequiresChoice, "sort_requires";
    (True, "true"),
    (False, "false")
}

// Override StyLua config with EditorConfig properties
fn load(mut config: Config, properties: &Properties) -> Config {
    if let Ok(end_of_line) = properties.get::<EndOfLine>() {
        match end_of_line {
            EndOfLine::Cr | EndOfLine::Lf => config.line_endings = LineEndings::Unix,
            EndOfLine::CrLf => config.line_endings = LineEndings::Windows,
        }
    }
    if let Ok(indent_size) = properties.get::<IndentSize>() {
        match indent_size {
            IndentSize::Value(indent_width) => config.indent_width = indent_width,
            IndentSize::UseTabWidth => {
                if let Ok(TabWidth::Value(indent_width)) = properties.get::<TabWidth>() {
                    config.indent_width = indent_width
                }
            }
        }
    }
    if let Ok(indent_style) = properties.get::<IndentStyle>() {
        match indent_style {
            IndentStyle::Tabs => config.indent_type = IndentType::Tabs,
            IndentStyle::Spaces => config.indent_type = IndentType::Spaces,
        }
    }
    if let Ok(max_line_length) = properties.get::<MaxLineLen>() {
        match max_line_length {
            MaxLineLen::Value(column_width) => config.column_width = column_width,
            MaxLineLen::Off => config.column_width = usize::MAX,
        }
    }
    if let Ok(quote_type) = properties.get::<QuoteTypeChoice>() {
        match quote_type {
            QuoteTypeChoice::Double => config.quote_style = QuoteStyle::AutoPreferDouble,
            QuoteTypeChoice::Single => config.quote_style = QuoteStyle::AutoPreferSingle,
            QuoteTypeChoice::Auto => (),
        }
    }
    if let Ok(call_parentheses) = properties.get::<CallParenthesesChoice>() {
        match call_parentheses {
            CallParenthesesChoice::Always => config.call_parentheses = CallParenType::Always,
            CallParenthesesChoice::NoSingleString => {
                config.call_parentheses = CallParenType::NoSingleString
            }
            CallParenthesesChoice::NoSingleTable => {
                config.call_parentheses = CallParenType::NoSingleTable
            }
            CallParenthesesChoice::None => config.call_parentheses = CallParenType::None,
        }
    }
    if let Ok(collapse_simple_statement) = properties.get::<CollapseSimpleStatementChoice>() {
        match collapse_simple_statement {
            CollapseSimpleStatementChoice::Never => {
                config.collapse_simple_statement = CollapseSimpleStatement::Never
            }
            CollapseSimpleStatementChoice::FunctionOnly => {
                config.collapse_simple_statement = CollapseSimpleStatement::FunctionOnly
            }
            CollapseSimpleStatementChoice::ConditionalOnly => {
                config.collapse_simple_statement = CollapseSimpleStatement::ConditionalOnly
            }
            CollapseSimpleStatementChoice::Always => {
                config.collapse_simple_statement = CollapseSimpleStatement::Always
            }
        }
    }
    if let Ok(sort_requires) = properties.get::<SortRequiresChoice>() {
        match sort_requires {
            SortRequiresChoice::True => config.sort_requires = SortRequiresConfig { enabled: true },
            SortRequiresChoice::False => {
                config.sort_requires = SortRequiresConfig { enabled: false }
            }
        }
    }

    config
}

// Read the EditorConfig files that would apply to a file at the given path
pub fn parse(config: Config, path: &Path) -> Result<Config, Error> {
    let properties = properties_of(path)?;

    if properties.iter().count() == 0 {
        return Ok(config);
    }

    log::debug!("editorconfig: found properties for {}", path.display());
    let new_config = load(config, &properties);

    Ok(new_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<&Properties> for Config {
        fn from(properties: &Properties) -> Self {
            load(Config::default(), properties)
        }
    }

    #[test]
    fn test_end_of_line_cr() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("end_of_line", "CR");
        let config = Config::from(&properties);
        assert_eq!(config.line_endings, LineEndings::Unix);
    }

    #[test]
    fn test_end_of_line_lf() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("end_of_line", "lf");
        let config = Config::from(&properties);
        assert_eq!(config.line_endings, LineEndings::Unix);
    }

    #[test]
    fn test_end_of_line_crlf() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("end_of_line", "CrLf");
        let config = Config::from(&properties);
        assert_eq!(config.line_endings, LineEndings::Windows);
    }

    #[test]
    fn test_indent_size() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("indent_size", "2");
        let config = Config::from(&properties);
        assert_eq!(config.indent_width, 2);
    }

    #[test]
    fn test_indent_size_use_tab_width() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("tab_width", "8");
        properties.insert_raw_for_key("indent_size", "tab");
        let config = Config::from(&properties);
        assert_eq!(config.indent_width, 8);
    }

    #[test]
    fn test_indent_style_space() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("indent_style", "space");
        let config = Config::from(&properties);
        assert_eq!(config.indent_type, IndentType::Spaces);
    }

    #[test]
    fn test_indent_style_tab() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("indent_style", "Tab");
        let config = Config::from(&properties);
        assert_eq!(config.indent_type, IndentType::Tabs);
    }

    #[test]
    fn test_max_line_length() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("max_line_length", "80");
        let config = Config::from(&properties);
        assert_eq!(config.column_width, 80);
    }

    #[test]
    fn test_max_line_length_off() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("max_line_length", "off");
        let config = Config::from(&properties);
        assert_eq!(config.column_width, usize::MAX);
    }

    #[test]
    fn test_quote_type_double() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("quote_type", "double");
        let config = Config::from(&properties);
        assert_eq!(config.quote_style, QuoteStyle::AutoPreferDouble);
    }

    #[test]
    fn test_quote_type_single() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("quote_type", "Single");
        let config = Config::from(&properties);
        assert_eq!(config.quote_style, QuoteStyle::AutoPreferSingle);
    }

    #[test]
    fn test_quote_type_auto() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("quote_type", "auto");
        let config = Config::from(&properties);
        assert_eq!(config.quote_style, QuoteStyle::AutoPreferDouble);
    }

    #[test]
    fn test_call_parentheses_always() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("call_parentheses", "always");
        let config = Config::from(&properties);
        assert_eq!(config.call_parentheses, CallParenType::Always);
    }

    #[test]
    fn test_call_parentheses_no_single_string() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("call_parentheses", "NoSingleString");
        let config = Config::from(&properties);
        assert_eq!(config.call_parentheses, CallParenType::NoSingleString);
    }

    #[test]
    fn test_call_parentheses_no_single_table() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("call_parentheses", "NoSingleTable");
        let config = Config::from(&properties);
        assert_eq!(config.call_parentheses, CallParenType::NoSingleTable);
    }

    #[test]
    fn test_call_parentheses_none() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("call_parentheses", "None");
        let config = Config::from(&properties);
        assert_eq!(config.call_parentheses, CallParenType::None);
    }

    #[test]
    fn test_collapse_simple_statement_never() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("collapse_simple_statement", "Never");
        let config = Config::from(&properties);
        assert_eq!(
            config.collapse_simple_statement,
            CollapseSimpleStatement::Never
        );
    }

    #[test]
    fn test_collapse_simple_statement_function_only() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("collapse_simple_statement", "FunctionOnly");
        let config = Config::from(&properties);
        assert_eq!(
            config.collapse_simple_statement,
            CollapseSimpleStatement::FunctionOnly
        );
    }

    #[test]
    fn test_collapse_simple_statement_conditional_only() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("collapse_simple_statement", "ConditionalOnly");
        let config = Config::from(&properties);
        assert_eq!(
            config.collapse_simple_statement,
            CollapseSimpleStatement::ConditionalOnly
        );
    }

    #[test]
    fn test_collapse_simple_statement_always() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("collapse_simple_statement", "always");
        let config = Config::from(&properties);
        assert_eq!(
            config.collapse_simple_statement,
            CollapseSimpleStatement::Always
        );
    }

    #[test]
    fn test_sort_requires_enabled() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("sort_requires", "true");
        let config = Config::from(&properties);
        assert!(config.sort_requires.enabled);
    }

    #[test]
    fn test_sort_requires_disabled() {
        let mut properties = Properties::new();
        properties.insert_raw_for_key("sort_requires", "false");
        let config = Config::from(&properties);
        assert!(!config.sort_requires.enabled);
    }

    #[test]
    fn test_invalid_properties() {
        let mut properties = Properties::new();
        let default_config = Config::new();
        let invalid_value = " ";
        for key in [
            "end_of_line",
            "indent_size",
            "indent_style",
            "quote_style",
            "call_parentheses",
            "collapse_simple_statement",
            "sort_requires",
        ] {
            properties.insert_raw_for_key(key, invalid_value);
        }
        let config = Config::from(&properties);
        assert_eq!(config.line_endings, default_config.line_endings);
        assert_eq!(config.indent_width, default_config.indent_width);
        assert_eq!(config.indent_type, default_config.indent_type);
        assert_eq!(config.column_width, default_config.column_width);
        assert_eq!(config.quote_style, default_config.quote_style);
        assert_eq!(config.call_parentheses, default_config.call_parentheses);
        assert_eq!(
            config.collapse_simple_statement,
            default_config.collapse_simple_statement
        );
        assert_eq!(
            config.sort_requires.enabled,
            default_config.sort_requires.enabled
        );
    }
}
