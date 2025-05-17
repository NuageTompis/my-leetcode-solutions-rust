use crate::{
    parse_api::{try_group_example_testcases, FunctionMetaData, ParamJson, ProbMetaData},
    read_write,
};

const TEST_FUNCTION_ID_PATTERN: &str = "%EXAMPLE_ID%";
const TEST_FUNCTION_CONTENT_PATTERN: &str = "%CONTENT%";
const TEST_MODULE_FUNCTIONS_PATTERN: &str = "%TEST_FUNCTIONS%";

pub fn try_create_test_module(
    example_testcases: &str,
    metadata: &ProbMetaData,
) -> Result<String, ()> {
    let metadata = match metadata {
        ProbMetaData::Class(_) => todo!("Test module generation for class problems is not yet implemented !"),
        ProbMetaData::Function(function_meta_data) => function_meta_data,
    };

    // read the mold for the test functions
    let function_mold_res = read_write::try_read_test_function_mold();
    let function_mold = match function_mold_res {
        Ok(mold) => mold,
        Err(e) => {
            println!("{e}");
            return Err(());
        }
    };

    // read the mold for the test module
    let module_mold_res = read_write::try_read_test_module_mold();
    let module_mold = match module_mold_res {
        Ok(mold) => mold,
        Err(e) => {
            println!("{e}");
            return Err(());
        }
    };

    // group example testcases
    let grouped_testcases_res =
        try_group_example_testcases(example_testcases, metadata.params.len());
    let grouped_testcases = match grouped_testcases_res {
        Ok(groups) => groups,
        Err(e) => {
            println!("{e}");
            return Err(());
        }
    };

    // add the test functions to the resulting String
    let mut functions_str = String::new();
    for (id, group) in grouped_testcases.iter().enumerate() {
        let test_function_creation_res =
            create_test_function(&function_mold, id + 1, &metadata.params, group, metadata);
        let test_function = match test_function_creation_res {
            Ok(fun) => fun,
            Err(e) => {
                println!("{e}");
                return Err(());
            }
        };
        functions_str.push_str(&test_function);
    }

    Ok(module_mold.replace(TEST_MODULE_FUNCTIONS_PATTERN, &functions_str))
}

fn create_test_function(
    mold: &str,
    id: usize,
    params: &[ParamJson],
    testcase: &[String],
    metadata: &FunctionMetaData,
) -> Result<String, String> {
    let res = mold.replace(TEST_FUNCTION_ID_PATTERN, &id.to_string());
    let mut content = String::new();

    for (param, value) in params.iter().zip(testcase.iter()) {
        let variable_formatted = param._type.try_write_variable(value)?;
        let line = format!(
            "        let {}: {} = {};\n",
            param.name, param._type, variable_formatted
        );
        content.push_str(&line);
    }

    let arguments = params
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
        .join(", ");
    content.push_str(&format!(
        "        let res = Solution::{}({});\n",
        metadata.name, arguments
    ));
    if let Some(return_type) = &metadata._return {
        content.push_str(&format!(
            "        let expected: {} = todo!(); // Fill in this value\n",
            return_type._type
        ));
        content.push_str("        assert_eq!(res, expected);\n");
    }

    Ok(res.replace(TEST_FUNCTION_CONTENT_PATTERN, &content))
}

#[cfg(test)]
mod tests {
    use crate::parse_api::{DataType, FunctionMetaData, ReturnJson, ScalarType};

    use super::*;

    #[test]
    fn test_create_test_module() {
        let example_testcases = r"[4,5,6,7,0,1,2]
0";
        let params: Vec<ParamJson> = vec![
            ParamJson {
                name: "nums".into(),
                _type: DataType {
                    scalar_type: ScalarType::Integer,
                    vec_depth: 1,
                },
            },
            ParamJson {
                name: "target".into(),
                _type: DataType {
                    scalar_type: ScalarType::Integer,
                    vec_depth: 0,
                },
            },
        ];
        let metadata = ProbMetaData::Function(FunctionMetaData {
            name: "search".into(),
            params: params.clone(),
            _return: Some(ReturnJson {
                _type: DataType {
                    scalar_type: ScalarType::Integer,
                    vec_depth: 0,
                },
            }),
        });
        let res = try_create_test_module(example_testcases, &metadata);
        let expected = r"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![4,5,6,7,0,1,2];
        let target: i32 = 0;
        let res = Solution::search(nums, target);
        let expected: i32 = todo!(); // Fill in this value
        assert_eq!(res, expected);
    }
}"
        .into();
        assert_eq!(res, Ok(expected));
    }
}
