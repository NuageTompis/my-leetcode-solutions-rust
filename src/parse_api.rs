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
}
