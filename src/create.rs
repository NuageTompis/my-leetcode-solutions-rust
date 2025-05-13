use colored::Colorize;

use crate::fetch::{ProblemContent, ProblemJSON};
use crate::parse_api::ProbMetaData;
use crate::read_write::LocalReadResult;
use crate::{fetch, read_write};

use crate::args::CreateCommand;

pub const UNEXPECTED_ERR_HEADER: &str = "Unexpected error while";
const ERR_HEADER: &str = "Error while";
const PREMIUM_COMMAND: &str = "`cargo conf premium (0 or 1)`";

const NO_PREMIUM_ERR: &str = "This problem seems to be premium-only but you are registered as a free-user. Please run `cargo conf premium 1` if you are premium.";

const TEST_COMPILER_CONFIGURATION_ATTRIBUTE: &str = "#[cfg(test)]\n";
const PATTERNS_TO_GIVE_TEST_ATTRIBUTE: [&str; 2] = ["impl ", "struct "];

pub async fn handle_create_command(create: CreateCommand) {
    println!("Checking if you're premium...");
    let premium_res = try_checking_if_user_is_premium();
    let premium = match premium_res {
        Ok(premium) => premium,
        Err(_) => {
            return;
        }
    };

    println!("Trying to find slug locally...");
    let slug_local_read_result = try_read_slug_locally(create.problem_id, premium);
    let slug_option = match slug_local_read_result {
        Ok(slug) => slug,
        Err(_) => {
            return;
        }
    };

    let slug = if let Some(slug) = slug_option {
        slug
    } else {
        println!("Couldn't find slug locally, trying to fetch it...");
        let slug_fetch_result = try_fetch_slug(create.problem_id, premium).await;
        match slug_fetch_result {
            Ok(slug) => slug,
            Err(_) => {
                return;
            }
        }
    };

    println!("Trying to fetch problem content...");
    let content_fetch_result = fetch::try_fetch_content(&slug).await;
    let problem_content = match content_fetch_result {
        Ok(val) => val,
        Err(e) => {
            e.log(create.problem_id);
            return;
        }
    };

    println!("Trying to fetch problem example testcases...");
    let example_testcases_result = fetch::try_fetch_example_testcases(&slug).await;
    let example_testcases = match example_testcases_result {
        Ok(ex_testcases) => ex_testcases,
        Err(_) => {
            return;
        }
    };
    // todo: add the test module and the test cases

    println!("Trying to create a solution file...");
    try_creating_solution_file(
        problem_content,
        &example_testcases,
        create.problem_id,
        &slug,
    );
}

/// Tries finding the slug of a problem using the local file `slugs_and_ids.txt`
///
/// It will return an error if we sould abort (eg the problem is premium-only but the user is not premium), or an Option<String> if we should continue
///
/// ### Arguments
///
/// * `problem_id` - The (front-end) id of the problem
/// * `premium` - The boolean describing if the user declared himself as premium or not
fn try_read_slug_locally(problem_id: u16, premium: bool) -> Result<Option<String>, ()> {
    let slug_read_res: LocalReadResult<(String, String)> =
        read_write::try_read_variable(&problem_id.to_string(), ',');
    let slug = match slug_read_res {
        LocalReadResult::Found((slug, prem)) => {
            // prem should be either 0 or 1
            let prem = match prem.parse::<u8>() {
                Ok(val) => match val {
                    0 => Some(false),
                    1 => Some(true),
                    _ => None,
                },
                Err(_) => None,
            };
            let prem = match prem {
                Some(prem) => prem,
                None => {
                    println!(
                        "{} parsing premium value locally for problem {}",
                        UNEXPECTED_ERR_HEADER.red().bold(),
                        problem_id
                    );
                    return Err(());
                }
            };
            if prem && !premium {
                println!("{}", NO_PREMIUM_ERR);
                return Err(());
            }
            Some(slug)
        }
        LocalReadResult::FileMissing => {
            println!("File `slugs_and_ids` missing, creating it...");
            None
        }
        LocalReadResult::LineMissing => None,
        LocalReadResult::LineCorrupted => {
            println!("We did find problem {} locally but the line seems to be corrupted. Starting api call...", problem_id);
            None
        }
        LocalReadResult::UnexpectedError => {
            println!(
                "{} reading slug locally.",
                UNEXPECTED_ERR_HEADER.red().bold()
            );
            None
        }
    };
    Ok(slug)
}

/// Starts by fetching all problems slugs from leetcode's api, then update `slugs_and_ids.txt`, then looks for the given problem's slug
///
/// ### Arguments
///
/// * `problem_id` - The (front-end) id of the problem
/// * `premium` - The boolean describing if the user declared himself as premium or not
async fn try_fetch_slug(problem_id: u16, premium: bool) -> Result<String, ()> {
    let all_result = fetch::try_fetch_problem_list().await;
    let res = handle_problems_fetch_and_find(all_result, problem_id);
    if let Some((slug, prem)) = res {
        if prem && !premium {
            println!("{}", NO_PREMIUM_ERR);
            Err(())
        } else {
            Ok(slug)
        }
    } else {
        println!(
            "We could not find problem {}, please double-check its number.",
            problem_id
        );
        Err(())
    }
}

/// Reads the `.env` file and searches if the user is premium
fn try_checking_if_user_is_premium() -> Result<bool, ()> {
    let premium_read_res: LocalReadResult<String> = read_write::try_read_variable("premium", '=');
    let premium = match premium_read_res {
        LocalReadResult::Found(value) => {
            let parse_res = value.parse::<u8>();
            match parse_res {
                Ok(val) => match val {
                    0 => false,
                    1 => true,
                    _ => {
                        println!(
                            "{} reading premium bool from .env, it should be 0 or 1",
                            UNEXPECTED_ERR_HEADER.red().bold()
                        );
                        return Err(());
                    }
                },
                Err(_) => {
                    println!(
                        "{} parsing premium bool from .env, it should be 0 or 1",
                        UNEXPECTED_ERR_HEADER.red().bold()
                    );
                    return Err(());
                }
            }
        }
        LocalReadResult::FileMissing => {
            println!(
                "There is no .env file, please use {} to create it",
                PREMIUM_COMMAND
            );
            false
        }
        LocalReadResult::LineMissing => {
            println!(
                "Please specify if you're a premium leetcode user by running {}",
                PREMIUM_COMMAND
            );
            false
        }
        LocalReadResult::LineCorrupted => {
            println!(
                "The .env line with the premium variable seems to be corrupted. Please run {}",
                PREMIUM_COMMAND
            );
            false
        }
        LocalReadResult::UnexpectedError => {
            println!(
                "{} trying to read premium variable in .env",
                UNEXPECTED_ERR_HEADER.red().bold()
            );
            false
        }
    };

    Ok(premium)
}

fn try_creating_solution_file(
    problem_content: ProblemContent,
    example_testcases: &str,
    problem_id: u16,
    slug: &str,
) {
    let content = apply_modifications_to_solution_file(
        problem_content.default_code,
        problem_content.metadata,
    );
    let filename = format!("s{}_{}", problem_id, slug).replace('-', "_");
    let file_path = format!("./src/solutions/{}.rs", filename);

    let write_file_res = read_write::try_write_solution_template(&file_path, &content); // we check that it doesn't already exist
    match write_file_res {
        Ok(_) => {
            println!(
                "{} created and wrote to {}",
                "Successfully".cyan().bold(),
                file_path
            );
            // declares the newly created module file so that it is included in unit tests
            let append_mod_res = read_write::try_append_solution_module(&filename);
            if let Err(e) = append_mod_res {
                println!(
                    "{} declaring module of solution file: {}",
                    UNEXPECTED_ERR_HEADER.red().bold(),
                    e
                );
            }
        }
        Err(e) => {
            println!("{} creating solution file: {}", ERR_HEADER, e);
        }
    }
}

fn apply_modifications_to_solution_file(content: String, metadata: ProbMetaData) -> String {
    let mut content = if let ProbMetaData::Function(_) = metadata {
        format!("struct Solution;\n\n{}", content)
    } else {
        content
    };

    for pattern in PATTERNS_TO_GIVE_TEST_ATTRIBUTE {
        let concat = format!("{}{}", TEST_COMPILER_CONFIGURATION_ATTRIBUTE, pattern);
        content = content.replace(pattern, &concat);
    }

    content
}

fn handle_problems_fetch_and_find(
    result: Result<Vec<ProblemJSON>, Box<dyn std::error::Error>>,
    problem_id: u16,
) -> Option<(String, bool)> {
    match result {
        Ok(problems) => {
            let mut problem: Option<(String, bool)> = None;
            let new_slugs_and_ids_content = problems
                .iter()
                .map(|prob| {
                    if prob.problem_stat.frontend_question_id == problem_id {
                        problem = Some((
                            prob.problem_stat.question_title_slug.clone(),
                            prob.paid_only,
                        ));
                    }
                    format!(
                        "{},{},{}",
                        prob.problem_stat.frontend_question_id,
                        prob.problem_stat.question_title_slug,
                        match prob.paid_only {
                            true => 1,
                            false => 0,
                        }
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");
            let res = read_write::try_write_slugs_and_ids(new_slugs_and_ids_content);
            if let Err(e) = res {
                println! {"{} writing all problems to slugs_and_ids: {}", UNEXPECTED_ERR_HEADER.red().bold(), e};
            }
            problem
        }
        Err(e) => {
            println!(
                "{} fecthing all problems: {}",
                UNEXPECTED_ERR_HEADER.red().bold(),
                e
            );
            None
        }
    }
}
