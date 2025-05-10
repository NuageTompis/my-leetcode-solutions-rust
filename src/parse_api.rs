use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ProbMetaData {
    Class(ClassMetaData),
    Function(FunctionMetaData),
    Unknown(String),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ClassMetaData {
    #[serde(rename = "classname")]
    class_name: String,
    constructor: ConstructorJson,
    methods: Vec<FunctionMetaData>,
    #[serde(rename = "return")]
    _return: ReturnJson,
    systemdesign: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ConstructorJson {
    params: Vec<ParamJson>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct FunctionMetaData {
    name: String,
    params: Vec<ParamJson>,
    #[serde(rename = "return")]
    _return: ReturnJson,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ParamJson {
    name: String,
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ReturnJson {
    #[serde(rename = "type")]
    _type: String,
}

pub fn parse_meta_data(meta_data: &str) -> Result<ProbMetaData, serde_json::Error> {
    let raw: serde_json::Value = serde_json::from_str(meta_data)?;
    if raw.get("name").is_some() {
        let res: FunctionMetaData = serde_json::from_str(meta_data)?;
        Ok(ProbMetaData::Function(res))
    } else if raw.get("classname").is_some() {
        let res: ClassMetaData = serde_json::from_str(meta_data)?;
        Ok(ProbMetaData::Class(res))
    } else {
        Ok(ProbMetaData::Unknown(meta_data.into()))
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
enum MyEnum {
    #[serde(rename = "dog")]
    Dog,
    #[serde(rename = "cat")]
    Cat,
    #[serde(rename = "elf")]
    Elf,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
enum ReturnJsonEnum {
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "string")]
    String,
    // list qqch
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Message {
    Text {
        content: String,
    },
    Image {
        url: String,
        width: u32,
        height: u32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_data_function() {
        let meta_data = r#"
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
        let res = parse_meta_data(meta_data).unwrap();
        let expected = ProbMetaData::Function(FunctionMetaData {
            name: "spiralOrder".into(),
            params: vec![ParamJson {
                name: "matrix".into(),
                _type: "integer[][]".into(),
            }],
            _return: ReturnJson {
                _type: "list<integer>".into(),
            },
        });
        assert_eq!(res, expected);
    }

    #[test]
    fn test_meta_data_class() {
        let meta_data = r#"
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
        let res = parse_meta_data(meta_data).unwrap();
        let expected = ProbMetaData::Class(ClassMetaData {
            class_name: "RandomizedSet".into(),
            constructor: ConstructorJson { params: Vec::new() },
            methods: Vec::new(),
            _return: ReturnJson {
                _type: "boolean".into(),
            },
            systemdesign: Some(true),
        });
        assert_eq!(res, expected);
    }

    #[test]
    fn test_meta_data_unknown() {
        let meta_data = r#"
        {
        "someting": "that",
        "we": {
        "do_not": "know"
        }
        }
        "#;
        let res = parse_meta_data(meta_data).unwrap();
        let expected = ProbMetaData::Unknown(meta_data.into());
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_enum() {
        let dog = r#" {"type": "dog"} "#;
        let dog_res = serde_json::from_str::<MyEnum>(dog);
        assert_eq!(dog_res.unwrap(), MyEnum::Dog);
        let nothing = r#" {"type": "god"} "#;
        let nothing_res = serde_json::from_str::<MyEnum>(nothing);
        assert!(nothing_res.is_err());
        let cat = r#" {"type": "cat"} "#;
        let cat_res = serde_json::from_str::<MyEnum>(cat);
        assert_eq!(cat_res.unwrap(), MyEnum::Cat);
        let nothing_bis = r#" {"type": "catdog"} "#;
        let nothing_bis_res = serde_json::from_str::<MyEnum>(nothing_bis);
        assert!(nothing_bis_res.is_err());
    }

    #[test]
    fn test_one() {
        let ex_1 = r#"{ "type": "Text", "content": "Hello, world!" }"#;
        let ex_2 = r#"{ "type": "Image", "url": "http://example.com/cat.jpg", "width": 800, "height": 600 }"#;
        let msg: Message = serde_json::from_str(ex_1).unwrap();
        let msg: Message = serde_json::from_str(ex_2).unwrap();
    }
}
