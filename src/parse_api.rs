use std::fmt::{self};
use std::str::FromStr;

use regex::Regex;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ProbMetaData {
    Class(ClassMetaData),
    Function(FunctionMetaData),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ClassMetaData {
    #[serde(rename = "classname")]
    pub class_name: String,
    pub constructor: ConstructorJson,
    pub methods: Vec<FunctionMetaData>,
    #[serde(rename = "return")]
    pub _return: ReturnJson,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ConstructorJson {
    pub params: Vec<ParamJson>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FunctionMetaData {
    #[serde(deserialize_with = "deserialize_pascal_to_snake_case")]
    pub name: String,
    pub params: Vec<ParamJson>,
    #[serde(rename = "return")]
    pub _return: Option<ReturnJson>, // was set as an Option to handle problem 470
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ParamJson {
    pub name: String,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "deserialize_data_type")]
    pub _type: DataType,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ReturnJson {
    #[serde(rename = "type")]
    #[serde(deserialize_with = "deserialize_data_type")]
    pub _type: DataType,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DataType {
    pub scalar_type: ScalarType,
    pub vec_depth: u8,
}

impl HasSpecialDataType for ParamJson {
    fn has_tree_node(&self) -> bool {
        self._type.scalar_type == ScalarType::TreeNode
    }

    fn has_list_node(&self) -> bool {
        self._type.scalar_type == ScalarType::ListNode
    }
}

impl HasSpecialDataType for ProbMetaData {
    fn has_tree_node(&self) -> bool {
        match self {
            ProbMetaData::Class(metadata) => metadata
                .constructor
                .params
                .iter()
                .any(|param| param.has_tree_node()),
            ProbMetaData::Function(metadata) => {
                metadata.params.iter().any(|param| param.has_tree_node())
            }
        }
    }

    fn has_list_node(&self) -> bool {
        match self {
            ProbMetaData::Class(metadata) => metadata
                .constructor
                .params
                .iter()
                .any(|param| param.has_list_node()),

            ProbMetaData::Function(metadata) => {
                metadata.params.iter().any(|param| param.has_list_node())
            }
        }
    }
}

pub trait HasSpecialDataType {
    fn has_tree_node(&self) -> bool;
    fn has_list_node(&self) -> bool;
}

impl ParamJson {
    #[cfg(test)]
    pub fn simple(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            _type: DataType::from(ScalarType::Integer),
        }
    }

    #[cfg(test)]
    pub fn from_scalar(name: &str, scalar_type: ScalarType) -> Self {
        Self {
            name: name.to_owned(),
            _type: DataType::from(scalar_type),
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = self.scalar_type.to_string();
        for _ in 0..self.vec_depth {
            res = format!("Vec<{}>", res);
        }
        write!(f, "{}", res)
    }
}

impl fmt::Display for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScalarType::Integer => write!(f, "i32"),
            ScalarType::Character => write!(f, "char"),
            ScalarType::Boolean => write!(f, "bool"),
            ScalarType::Long => write!(f, "i64"),
            ScalarType::String => write!(f, "String"),
            ScalarType::ListNode => write!(f, "Option<Box<ListNode>>"),
            ScalarType::Double => write!(f, "f64"),
            ScalarType::TreeNode => write!(f, "Option<Rc<RefCell<TreeNode>>>"),
            ScalarType::Void => write!(f, "()"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum ScalarType {
    Integer,
    Character,
    Boolean,
    Long,
    String,
    ListNode,
    Double,
    TreeNode,
    Void,
}

impl FromStr for DataType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DataType::recur(s, 0)
    }
}

impl DataType {
    fn recur(outer: &str, depth: u8) -> Result<Self, String> {
        // first we check if the string has the 'list' modifier
        let list_re = Regex::new("^list<(.+)>$").unwrap();
        let mut results = vec![];
        for (_, [inner]) in list_re.captures_iter(outer).map(|c| c.extract()) {
            results.push(inner);
        }
        // if not we check if it has the array modifier
        if results.is_empty() {
            let array_re = Regex::new(r"^(.+)\[\]$").unwrap();
            for (_, [inner]) in array_re.captures_iter(outer).map(|c| c.extract()) {
                results.push(inner);
            }
        }
        match results.len() {
            // no modifier
            0 => {
                let scalar_res = outer.parse::<ScalarType>();
                match scalar_res {
                    Ok(scalar) => Ok(DataType {
                        scalar_type: scalar,
                        vec_depth: depth,
                    }),
                    Err(e) => Err(e),
                }
            }
            // one modifier
            1 => Self::recur(results[0], depth + 1),
            // multiple modifiers ?
            _ => Err(format!(
                "Error parsing the string {} as a data type, found multiple modifier patterns",
                outer
            )),
        }
    }

    /// Tries to parse a function argument (param) into the corresponding rust code
    ///
    /// ### Example
    ///
    /// ```
    /// let data_type = DataType {
    ///     scalar_type: ScalarType::Character,
    ///     vec_depth: 2,
    /// };
    /// let res = data_type.try_write_variable(r#"[["5","3"],["6","."]]"#);
    /// let expected = "vec![vec!['5','3'],vec!['6','.']]".into();
    /// assert_eq!(res, Ok(expected));
    /// ```
    pub fn try_write_variable(&self, value: &str) -> Result<String, String> {
        try_write_variable_recur(self.scalar_type, value, self.vec_depth)
    }

    #[cfg(test)]
    pub fn from(scalar_type: ScalarType) -> Self {
        Self {
            scalar_type,
            vec_depth: 0,
        }
    }
}

fn try_write_variable_recur(
    scalar_type: ScalarType,
    from: &str,
    depth: u8,
) -> Result<String, String> {
    if depth == 0 {
        return scalar_type.format(from);
    }

    let mut result = String::new();
    let mut chars = from.chars();

    if chars.next() == Some('[') {
        result.push_str("vec![");
    } else {
        panic!("Expecting a leading '['");
    }

    let mut buffer = String::new();
    let mut level = 0;

    for c in chars {
        match c {
            '[' => {
                buffer.push(c);
                level += 1;
            }
            ']' => {
                if level > 0 {
                    buffer.push(c);
                    level -= 1;
                } else {
                    if !buffer.is_empty() {
                        // format the last element of the array (needed since there is no ending comma)
                        let element = try_write_variable_recur(scalar_type, &buffer, depth - 1);
                        result.push_str(&element?);
                        buffer.clear();
                    }
                    result.push(']');
                    break;
                }
            }
            ',' if level == 0 => {
                let element = try_write_variable_recur(scalar_type, &buffer, depth - 1);
                result.push_str(&element?);
                result.push(',');
                buffer.clear();
            }
            _ => {
                buffer.push(c);
            }
        }
    }

    Ok(result)
}

fn deserialize_data_type<'de, D>(deserializer: D) -> Result<DataType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    DataType::from_str(&s).map_err(de::Error::custom)
}

fn deserialize_pascal_to_snake_case<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.snake_case())
}

impl FromStr for ScalarType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "integer" => Ok(Self::Integer),
            "character" => Ok(Self::Character),
            "boolean" => Ok(Self::Boolean),
            "long" => Ok(Self::Long),
            "string" => Ok(Self::String),
            "ListNode" => Ok(Self::ListNode),
            "double" => Ok(Self::Double),
            "TreeNode" => Ok(Self::TreeNode),
            "void" => Ok(Self::Void),
            "String" => Ok(Self::String), // handles typo in problem 933
            _ => Err(format!("{} is not a known scalar type", s)),
        }
    }
}

impl ScalarType {
    fn format(&self, value: &str) -> Result<String, String> {
        let formatted: String = match self {
            ScalarType::Integer => value.into(),
            ScalarType::Character => {
                let c = value.chars().nth(1).ok_or(format!(
                    r#"Erorr formatting the char {}, expecting someting like "c""#,
                    value
                ))?;
                format!("'{}'", c)
            }
            ScalarType::Boolean => value.into(),
            ScalarType::Long => value.into(),
            ScalarType::String => format!("{}.into()", value),
            ScalarType::ListNode => format!("linked!{}", value),
            ScalarType::Double => value.into(),
            ScalarType::TreeNode => format!("tree!{}", value),
            ScalarType::Void => todo!(),
        };
        Ok(formatted)
    }
}

pub trait SnakeCase {
    fn snake_case(&self) -> String;
}

impl SnakeCase for str {
    fn snake_case(&self) -> String {
        let mut res = String::new();
        let mut prev_is_cap = false;
        for c in self.chars() {
            match c {
                'A'..='Z' => {
                    if !prev_is_cap {
                        res.push('_');
                    }
                    res.push_str(&c.to_lowercase().to_string());
                    prev_is_cap = true;
                }
                'a'..='z' => {
                    res.push(c);
                    prev_is_cap = false;
                }
                _ => panic!(
                    "Incorrect character '{}' found, expecting a pascal case string",
                    c
                ),
            }
        }
        res
    }
}

/// ## Description
///
/// Takes an array of values, usually arrays, described in a string, and returns a `Vec` containing all outer strings
///
/// ## Example
/// ```
/// let array = "[[1,2],[3]]";
/// let res = try_split_array(array);
/// let expected = vec!["[1,2]".into(), "[3]".into()];
/// assert_eq!(res, Ok(expected));
/// ```
pub fn try_split_array(array: &str) -> Result<Vec<String>, String> {
    let mut chars = array.chars();
    let bracket = chars.next().ok_or("Empty".to_owned())?;
    if bracket != '[' {
        return Err("No leading bracket".into());
    }
    let mut bracket_depth = 0;
    let mut res: Vec<String> = Vec::new();
    let mut curr = String::new();
    for c in chars {
        match c {
            ',' if bracket_depth == 0 => {
                res.push(curr);
                curr = String::new();
            }
            '[' => {
                bracket_depth += 1;
                curr.push(c);
            }
            ']' => {
                if bracket_depth == 0 {
                    res.push(curr);
                    curr = String::new();
                }
                bracket_depth -= 1;
                curr.push(c);
            }
            _ => {
                curr.push(c);
            }
        }
    }
    Ok(res)
}

/// ## Description
///
/// Given an array of methods as string, and an array of corresponding arguments as string, parses the input into pairs of method name with their arguments
///
/// Example
///
/// ```
/// let methods_name = r#"["Class","put"]"#;
/// let methods_arguments = "[[2],[1,1]]";
/// let res = try_parse_class_problem_testcase(methods_name, methods_arguments);
/// let expected = vec![
///     ("Class", "[2]"),
///     ("put", "[1,1]"),
/// ];
/// assert_eq!(res, Ok(expected));
/// ```
pub fn try_parse_class_problem_testcase(
    methods_name: &str,
    methods_arguments: &str,
) -> Result<Vec<(String, String)>, String> {
    let names: Vec<String> = try_split_array(methods_name)?
        .iter()
        .map(|name| {
            if name.len() < 2 {
                Err("No quotes".into())
            } else {
                Ok(name[1..name.len() - 1].to_owned())
            }
        })
        .collect::<Result<_, String>>()?;
    let args: Vec<String> = try_split_array(methods_arguments)?;

    if names.len() != args.len() {
        Err("Methods and arguments amount mismatch".into())
    } else {
        Ok(names
            .iter()
            .zip(args)
            .map(|(a, b)| (a.to_string(), b))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_function() {
        let metadata = r#"
        {
        "name": "spiralOrder",
        "params": [
            {
            "name": "matrix",
            "type": "integer[][]"
            }
        ],
        "return": {
            "type": "list<integer>",
            "size": "size_1 * col_size_1",
            "dealloc": true
        }
        }
        "#;
        let res = serde_json::from_str::<ProbMetaData>(metadata);
        let expected = ProbMetaData::Function(FunctionMetaData {
            name: "spiral_order".into(),
            params: vec![ParamJson {
                name: "matrix".into(),
                _type: DataType {
                    scalar_type: ScalarType::Integer,
                    vec_depth: 2,
                },
            }],
            _return: Some(ReturnJson {
                _type: DataType {
                    scalar_type: ScalarType::Integer,
                    vec_depth: 1,
                },
            }),
        });
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn test_metadata_class() {
        let metadata = r#"
        {
        "classname": "RandomizedSet",
        "constructor": {
        "params": []
        },
        "methods": [
        ],
        "return": {
        "type": "boolean"
        },
        "systemdesign": true
        }
        "#;
        let res = serde_json::from_str::<ProbMetaData>(metadata);
        let expected = ProbMetaData::Class(ClassMetaData {
            class_name: "RandomizedSet".into(),
            constructor: ConstructorJson { params: Vec::new() },
            methods: Vec::new(),
            _return: ReturnJson {
                _type: DataType::from(ScalarType::Boolean),
            },
        });
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn test_meta_data_unknown() {
        let metadata = r#"
        {
        "someting": "that",
        "we": {
        "do_not": "know"
        }
        }
        "#;
        let res = serde_json::from_str::<ProbMetaData>(metadata);
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_scalar_type() {
        let res = "boolean".parse::<ScalarType>();
        assert_eq!(res, Ok(ScalarType::Boolean));
        let res = "ListNode".parse::<ScalarType>();
        assert_eq!(res, Ok(ScalarType::ListNode));
        let res = "foo".parse::<ScalarType>();
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_data_type() {
        let res = "string".parse::<DataType>();
        let expected = DataType::from(ScalarType::String);
        assert_eq!(res, Ok(expected));
        let res = "list<list<long>>".parse::<DataType>();
        let expected = DataType {
            scalar_type: ScalarType::Long,
            vec_depth: 2,
        };
        assert_eq!(res, Ok(expected));
        let res = "character[]".parse::<DataType>();
        let expected = DataType {
            scalar_type: ScalarType::Character,
            vec_depth: 1,
        };
        assert_eq!(res, Ok(expected));
        let res = "list<foo[]".parse::<DataType>();
        assert!(res.is_err());
    }

    #[test]
    fn test_display_data_type() {
        let string_type = DataType::from(ScalarType::String);
        let res = format!("{}", string_type);
        assert_eq!(res, String::from("String"));

        let char_type = DataType {
            scalar_type: ScalarType::Character,
            vec_depth: 2,
        };
        let res = format!("{}", char_type);
        assert_eq!(res, String::from("Vec<Vec<char>>"));
    }

    #[test]
    fn test_try_write_variable() {
        let data_type = DataType {
            scalar_type: ScalarType::Integer,
            vec_depth: 1,
        };
        let res = data_type.try_write_variable("[1,2,3]");
        let expected = "vec![1,2,3]".into();
        assert_eq!(res, Ok(expected));

        let data_type = DataType {
            scalar_type: ScalarType::String,
            vec_depth: 1,
        };
        let res = data_type.try_write_variable(r#"["hello"]"#);
        let expected = r#"vec!["hello".into()]"#.into();
        assert_eq!(res, Ok(expected));

        let data_type = DataType {
            scalar_type: ScalarType::Character,
            vec_depth: 2,
        };
        let res = data_type.try_write_variable(r#"[["5","3"],["6","."]]"#);
        let expected = "vec![vec!['5','3'],vec!['6','.']]".into();
        assert_eq!(res, Ok(expected));
    }

    #[test]
    fn test_try_parse_class_problem_testcase() {
        let methods_name = r#"["LRUCache","put","put","get"]"#;
        let methods_arguments = "[[2],[1,1],[2,2],[1]]";
        let res = try_parse_class_problem_testcase(methods_name, methods_arguments);
        let expected = vec![
            ("LRUCache".into(), "[2]".into()),
            ("put".into(), "[1,1]".into()),
            ("put".into(), "[2,2]".into()),
            ("get".into(), "[1]".into()),
        ];
        assert_eq!(res, Ok(expected));
    }

    #[test]
    fn test_try_split_array() {
        let array = "[[1,2],[3]]";
        let res = try_split_array(array);
        let expected = vec!["[1,2]".into(), "[3]".into()];
        assert_eq!(res, Ok(expected));
    }

    #[test]
    fn test_snake_case() {
        let s = "twoSum";
        let res = s.snake_case();
        let expected = String::from("two_sum");
        assert_eq!(res, expected);
        let s = "lengthOfLIS"; // #1
        let res = s.snake_case();
        let expected = String::from("length_of_lis");
        assert_eq!(res, expected);
    }
}
