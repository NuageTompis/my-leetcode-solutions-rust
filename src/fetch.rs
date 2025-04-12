use crate::args::{FetchCommand, FetchSubcommand};
use serde::{Deserialize, Serialize};

pub const PROBLEM_LIST_URL: &str = "https://leetcode.com/api/problems/algorithms/";

// todo: find a way to refactor this
#[derive(Debug, Deserialize, Serialize)]
struct Json1 {
    data: Json2,
}

#[derive(Debug, Deserialize, Serialize)]
struct Json2 {
    question: Json3,
}
#[derive(Debug, Deserialize, Serialize)]
struct Json3 {
    #[serde(rename = "codeDefinition")]
    code_definition: String,
    #[serde(rename = "metaData")]
    meta_data: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct Json4 {
    #[serde(rename = "defaultCode")]
    default_code: String,
    value: String,
}
//
#[derive(Debug, Serialize, Deserialize)]
struct Variables {
    #[serde(rename = "titleSlug")]
    title_slug: String,
}
const QUERY: &str = "query questionData($titleSlug: String!) { question(titleSlug: $titleSlug) { codeDefinition metaData }}";
const GRAPHQL_URL: &str = "https://leetcode.com/graphql";
impl QueryBody {
    fn new(title_slug: &str) -> Self {
        QueryBody {
            operation_name: "questionData".into(),
            variables: {
                Variables {
                    title_slug: title_slug.into(),
                }
            },
            query: QUERY.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryBody {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: Variables,
    query: String,
}

pub fn handle_fetch_command(fetch: FetchCommand) {
    match fetch.command {
        FetchSubcommand::Slugs => {
            println!("This command is not implemented yet");
        }
        FetchSubcommand::Unimplemented => {
            println!("This command is not implemented yet");
        }
    }
}

pub async fn try_fetch_problem_list() -> Result<Vec<ProblemJSON>, Box<dyn std::error::Error>> {
    let result: ProblemListJSON = reqwest::Client::new()
        .get(PROBLEM_LIST_URL)
        .send()
        .await?
        .json()
        .await?;

    Ok(result.problems)
}

#[derive(Debug)]
pub enum FetchContentErr {
    NotFound,
    ReqwestErr(reqwest::Error),
    ParseError(serde_json::Error),
}

// For unit tests
impl PartialEq for FetchContentErr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FetchContentErr::NotFound, FetchContentErr::NotFound) => true,
            (FetchContentErr::ReqwestErr(_), FetchContentErr::ReqwestErr(_)) => true,
            (FetchContentErr::ParseError(_), FetchContentErr::ParseError(_)) => true,
            _ => false,
        }
    }
}

pub async fn try_fetch_content(slug: &str) -> Result<String, FetchContentErr> {
    let body = QueryBody::new(slug);
    let response: Json1 = reqwest::Client::new()
        .post(GRAPHQL_URL)
        .json(&body)
        .send()
        .await
        .map_err(|e| FetchContentErr::ReqwestErr(e))?
        .json()
        .await
        .map_err(|e| FetchContentErr::ReqwestErr(e))?;

    let languages = response.data.question.code_definition;
    println!("meta data: {}", response.data.question.meta_data);
    let parsed: Vec<Json4> =
        serde_json::from_str(&languages).map_err(|e| FetchContentErr::ParseError(e))?;

    for lang in &parsed {
        if lang.value == "rust" {
            return Ok(lang.default_code.clone());
        }
    }

    Err(FetchContentErr::NotFound)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProblemListJSON {
    #[serde(rename = "stat_status_pairs")]
    pub problems: Vec<ProblemJSON>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProblemJSON {
    #[serde(rename = "stat")]
    pub problem_stat: ProblemStatJSON,
    pub paid_only: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProblemStatJSON {
    pub frontend_question_id: u16,
    #[serde(rename = "question__title_slug")]
    pub question_title_slug: String,
}

#[cfg(test)]
mod tests_fetch {
    use super::*;

    #[tokio::test]
    async fn rust_unavailable() {
        let problem_slug = "find-a-corresponding-node-of-a-binary-tree-in-a-clone-of-that-tree"; // problem_id = 1379
        let content = try_fetch_content(problem_slug).await;
        assert_eq!(content, Err(FetchContentErr::NotFound));
    }
}
