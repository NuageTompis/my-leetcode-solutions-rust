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
    class_name: String,
    constructor: ConstructorJson,
    methods: Vec<FunctionMetaData>,
    #[serde(rename = "return")]
    _return: ReturnJson,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ConstructorJson {
    params: Vec<ParamJson>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FunctionMetaData {
    name: String,
    params: Vec<ParamJson>,
    #[serde(rename = "return")]
    _return: Option<ReturnJson>, // was set as an Option to handle problem 470
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ParamJson {
    name: String,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "deserialize_data_type")]
    _type: DataType,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ReturnJson {
    #[serde(rename = "type")]
    #[serde(deserialize_with = "deserialize_data_type")]
    _type: DataType,
}

#[derive(Debug, PartialEq, Serialize)]
struct DataType {
    scalar_type: ScalarType,
    vec_depth: u8,
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
            ScalarType::ListNode => write!(f,"Option<Box<ListNode>>"),
            ScalarType::Double => write!(f, "f64"),
            ScalarType::TreeNode => write!(f, "Option<Rc<RefCell<TreeNode>>>"),
            ScalarType::Void => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
enum ScalarType {
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
}

fn deserialize_data_type<'de, D>(deserializer: D) -> Result<DataType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    DataType::from_str(&s).map_err(de::Error::custom)
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
            name: "spiralOrder".into(),
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
                _type: DataType {
                    scalar_type: ScalarType::Boolean,
                    vec_depth: 0,
                },
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
        let expected = DataType {
            scalar_type: ScalarType::String,
            vec_depth: 0,
        };
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
        let string_type = DataType {
            scalar_type: ScalarType::String,
            vec_depth: 0,
        };
        let res = format!("{}", string_type);
        assert_eq!(res, String::from("String"));

        let char_type = DataType {
            scalar_type: ScalarType::Character,
            vec_depth: 2,
        };
        let res = format!("{}", char_type);
        assert_eq!(res, String::from("Vec<Vec<char>>"));
    }
}
