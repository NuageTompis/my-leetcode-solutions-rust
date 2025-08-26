use std::collections::HashMap;

use crate::parse_api::{
    try_parse_class_problem_testcase, try_split_array, ClassMetaData, ParamJson, ReturnJson,
    ScalarType,
};

use super::create_test_module::{TEST_FUNCTION_CONTENT_PATTERN, TEST_FUNCTION_ID_PATTERN};

pub fn try_create_test_functions(
    mold: &str,
    example_testcases: &str,
    metadata: &ClassMetaData,
) -> Result<String, String> {
    // group example testcases
    let grouped_testcases = try_group_example_testcases(example_testcases)?;

    // add eact test function to the resulting String
    let mut functions_str = String::new();
    for (id, group) in grouped_testcases.iter().enumerate() {
        let test_function = try_create_test_function(mold, id + 1, group, metadata)?;
        functions_str.push_str(&test_function);
    }

    Ok(functions_str)
}

pub fn try_create_test_function(
    mold: &str,
    id: usize,
    testcase: &(String, String),
    metadata: &ClassMetaData,
) -> Result<String, String> {
    let res = mold.replace(TEST_FUNCTION_ID_PATTERN, &id.to_string());
    let mut content = String::new();

    let methods_name = &testcase.0;
    let methods_arguments = &testcase.1;
    let testcase = try_parse_class_problem_testcase(methods_name, methods_arguments)?;

    let methods_map: HashMap<String, (&Vec<ParamJson>, &Option<ReturnJson>)> = metadata
        .methods
        .iter()
        .map(|s| (s.name.to_owned(), (&s.params, &s._return)))
        .collect();

    // write class instantiation
    let constructor_args_testcase = try_split_array(&testcase.first().unwrap().1)?;
    for (param, value) in metadata
        .constructor
        .params
        .iter()
        .zip(constructor_args_testcase)
    {
        let variable_formatted = param._type.try_write_variable(&value)?;
        let line = format!(
            "        let {}: {} = {};\n",
            param.name, param._type, variable_formatted
        );
        content.push_str(&line);
    }
    let arguments = metadata
        .constructor
        .params
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
        .join(", ");
    let class_instantiation = format!(
        "        let my_class: {} = {}::new({});\n",
        metadata.class_name, metadata.class_name, arguments
    );
    content.push_str(&class_instantiation);

    // write each method call
    for (x, y) in &testcase[1..] {
        let method_call_block = try_write_method_call(x, y, &methods_map)?;
        content.push_str(&method_call_block);
    }

    Ok(res.replace(TEST_FUNCTION_CONTENT_PATTERN, &content))
}

fn try_write_method_call(
    method_name: &str,
    arguments: &str,
    methods_map: &HashMap<String, (&Vec<ParamJson>, &Option<ReturnJson>)>,
) -> Result<String, String> {
    let (params, ret) = methods_map
        .get(method_name)
        .ok_or("Example testcase called method undefined in problem metadata".to_owned())?;

    let arguments = try_split_array(arguments)?;
    if arguments.len() != params.len() {
        return Err("Methods and arguments amount mismatch".into());
    }

    let mut res = String::new();
    for (param, value) in params.iter().zip(arguments) {
        let variable_formatted = param._type.try_write_variable(&value)?;
        let line = format!(
            "        let {}: {} = {};\n",
            param.name, param._type, variable_formatted
        );
        res.push_str(&line);
    }

    let is_return_void = if let Some(return_type) = ret {
        return_type._type.scalar_type == ScalarType::Void
    } else {
        true
    };
    let arguments = params
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    let code_leading_function_call = if is_return_void { "" } else { "let res = " };
    res.push_str(&format!(
        "        {}my_class.{}({});\n",
        code_leading_function_call, method_name, arguments
    ));
    if !is_return_void {
        if let Some(return_type) = ret {
            let code_leading_expected_expression = if is_return_void {
                ""
            } else {
                &format!(": {}", return_type._type)
            };
            res.push_str(&format!(
                "        let expected{} = todo!(); // Fill in this value\n",
                code_leading_expected_expression
            ));
            let variable_to_assert = if is_return_void { "todo!()" } else { "res" };
            res.push_str(&format!(
                "        assert_eq!({}, expected);\n",
                variable_to_assert,
            ));
        }
    }

    Ok(res)
}

pub fn try_group_example_testcases(
    example_testcases: &str,
) -> Result<Vec<(String, String)>, String> {
    let lines: Vec<&str> = example_testcases.lines().collect();
    if lines.len() % 2 != 0 {
        return Err("Error grouping the example testcases into a list as we expected pairs of lists (function calls, function arguments)".into());
    }

    let mut res: Vec<(String, String)> = Vec::new();
    // Iterate over the lines in chunks of 2
    for chunk in lines.chunks(2) {
        let first = chunk[0].to_string();
        let second = chunk[1].to_string();
        res.push((first, second));
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::parse_api::{
        ConstructorJson, DataType, FunctionMetaData, ParamJson, ReturnJson, ScalarType,
    };

    use super::*;

    #[test]
    fn test_try_group_example_testcases_class_problem() {
        let example_testcases = r#"["FindElements","find","find"]
[[[-1,null,-1]],[1],[2]]
["FindElements","find","find","find"]
[[[-1,-1,-1,-1,-1]],[1],[3],[5]]"#;
        let res = try_group_example_testcases(example_testcases);
        let expected = vec![
            (
                r#"["FindElements","find","find"]"#.into(),
                r#"[[[-1,null,-1]],[1],[2]]"#.into(),
            ),
            (
                r#"["FindElements","find","find","find"]"#.into(),
                r#"[[[-1,-1,-1,-1,-1]],[1],[3],[5]]"#.into(),
            ),
        ];
        assert_eq!(res, Ok(expected));
        let example_testcases = "single_line";
        let res = try_group_example_testcases(example_testcases);
        assert!(res.is_err());
    }

    #[test]
    fn test_try_create_test_function() {
        let reduced_mold = "fn example_%EXAMPLE_ID%() {
%CONTENT%    }";
        let testcase: (String, String) = (
            r#"["LRUCache","put","put","get"]"#.into(),
            r#"[[2],[1,1],[2,2],[1]]"#.into(),
        );
        let metadata = ClassMetaData {
            class_name: "LRUCache".into(),
            constructor: ConstructorJson {
                params: vec![ParamJson::from_scalar("capacity", ScalarType::Integer)],
            },
            methods: vec![
                FunctionMetaData {
                    name: "get".into(),
                    params: vec![ParamJson::simple("key")],
                    _return: Some(ReturnJson {
                        _type: DataType::from(ScalarType::Integer),
                    }),
                },
                FunctionMetaData {
                    name: "put".into(),
                    params: vec![ParamJson::simple("key"), ParamJson::simple("value")],
                    _return: Some(ReturnJson {
                        _type: DataType::from(ScalarType::Void),
                    }),
                },
            ],
            _return: ReturnJson {
                _type: DataType::from(ScalarType::String),
            },
        };
        let res = try_create_test_function(reduced_mold, 1, &testcase, &metadata);
        let expected = r"fn example_1() {
        let capacity: i32 = 2;
        let my_class: LRUCache = LRUCache::new(capacity);
        let key: i32 = 1;
        let value: i32 = 1;
        my_class.put(key, value);
        let key: i32 = 2;
        let value: i32 = 2;
        my_class.put(key, value);
        let key: i32 = 1;
        let res = my_class.get(key);
        let expected: i32 = todo!(); // Fill in this value
        assert_eq!(res, expected);
    }"
        .into();
        assert_eq!(res, Ok(expected));
    }
}
