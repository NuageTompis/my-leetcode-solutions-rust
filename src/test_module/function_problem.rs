use crate::parse_api::{FunctionMetaData, ScalarType};

use super::create_test_module::{TEST_FUNCTION_CONTENT_PATTERN, TEST_FUNCTION_ID_PATTERN};

pub fn try_create_test_functions(
    mold: &str,
    example_testcases: &str,
    metadata: &FunctionMetaData,
) -> Result<String, String> {
    // group example testcases
    let grouped_testcases = try_group_example_testcases(example_testcases, metadata.params.len())?;

    // add each test function to the resulting String
    let mut functions_str = String::new();
    for (id, group) in grouped_testcases.iter().enumerate() {
        let test_function = try_create_test_function(mold, id + 1, group, metadata)?;
        functions_str.push_str(&test_function);
    }

    Ok(functions_str)
}

/// ## Description
///
/// Given the mold for a test function, its testcase, and the metadata for the problem, writes the rust code for the testcase in a String
///
/// ## Example
///
/// Returns a `String` mostly like this one
///
/// ```
/// fn example_1() {
///     let num: i32 = 38;
///     let res = Solution::add_digits(num);
///     let expected: i32 = todo!(); // Fill in this value
///     assert_eq!(res, expected);
/// }
/// ```
fn try_create_test_function(
    mold: &str,
    id: usize,
    testcase: &[String],
    metadata: &FunctionMetaData,
) -> Result<String, String> {
    let res = mold.replace(TEST_FUNCTION_ID_PATTERN, &id.to_string());
    let mut content = String::new();

    for (param, value) in metadata.params.iter().zip(testcase.iter()) {
        let variable_formatted = param._type.try_write_variable(value)?;
        let line = format!(
            "        let {}: {} = {};\n",
            param.name, param._type, variable_formatted
        );
        content.push_str(&line);
    }

    let arguments = metadata
        .params
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    let is_return_void = if let Some(return_type) = &metadata._return {
        return_type._type.scalar_type == ScalarType::Void
    } else {
        false
    };
    let code_leading_function_call = if is_return_void { "" } else { "let res = " };
    content.push_str(&format!(
        "        {}Solution::{}({});\n",
        code_leading_function_call, metadata.name, arguments
    ));

    if let Some(return_type) = &metadata._return {
        let code_leading_expected_expression = if is_return_void {
            ""
        } else {
            &format!(": {}", return_type._type)
        };
        content.push_str(&format!(
            "        let expected{} = todo!(); // Fill in this value\n",
            code_leading_expected_expression
        ));
        let variable_to_assert = if is_return_void { "todo!()" } else { "res" };
        content.push_str(&format!(
            "        assert_eq!({}, expected);\n",
            variable_to_assert,
        ));
    }

    Ok(res.replace(TEST_FUNCTION_CONTENT_PATTERN, &content))
}

/// Tries grouping the example testcases fetched from leetcode into a nice list
///
/// ### Arguments
///
/// * `example_testcases` - The example testcases as string separated by `\n`
/// * `params_amt` - The amount of parameters the function expects
///
/// ### Example
///
/// ```
/// let example_testcases = "[4,5]\n0\n[6]\n8";
/// let res = try_group_example_testcases(example_testcases, 2);
/// let expected = vec![vec!["[4,5]".into(),"0".into()],vec!["[6]".into(),"8".into()]];
/// assert_eq!(res, Ok(expected));
/// ```
fn try_group_example_testcases(
    example_testcases: &str,
    params_amt: usize,
) -> Result<Vec<Vec<String>>, String> {
    let lines: Vec<&str> = example_testcases.lines().collect();
    let example_elements_amt = lines.len();
    if example_elements_amt % params_amt != 0 {
        return Err("Error grouping the example testcases into a list of groups of the same size as the parameters amount".into());
    }

    let mut res: Vec<Vec<String>> = Vec::new();
    let groups_amt = example_elements_amt / params_amt;
    for group in 0..groups_amt {
        let ndx = group * params_amt;
        let slice = &lines[ndx..ndx + params_amt];
        res.push(slice.iter().map(|el| el.to_string()).collect());
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::parse_api::{DataType, ParamJson, ReturnJson};

    use super::*;

    #[test]
    fn test_group_example_testcases() {
        let example_testcases = r"[4,5,6,7,0,1,2]
0
[4,5,6,7,0,1,2]
3
[1]
0";
        let res = try_group_example_testcases(example_testcases, 2);
        let mut expected = Vec::new();
        expected.push(vec!["[4,5,6,7,0,1,2]".into(), "0".into()]);
        expected.push(vec!["[4,5,6,7,0,1,2]".into(), "3".into()]);
        expected.push(vec!["[1]".into(), "0".into()]);
        assert_eq!(res, Ok(expected));

        let example_testcases = r"single line";
        let res = try_group_example_testcases(example_testcases, 2);
        assert!(res.is_err());
    }

    #[test]
    fn test_try_create_test_function() {
        let reduced_mold = "fn example_%EXAMPLE_ID%() {
%CONTENT%    }";
        let testcase: &[String] = &["38".into()];
        let metadata = FunctionMetaData {
            name: "add_digits".into(),
            params: vec![ParamJson::simple("num")],
            _return: Some(ReturnJson {
                _type: DataType::from(ScalarType::Integer),
            }),
        };
        let res = try_create_test_function(reduced_mold, 1, testcase, &metadata);
        let expected = r"fn example_1() {
        let num: i32 = 38;
        let res = Solution::add_digits(num);
        let expected: i32 = todo!(); // Fill in this value
        assert_eq!(res, expected);
    }"
        .into();
        assert_eq!(res, Ok(expected));
    }
}
