use colored::Colorize;

use crate::fetch::ProblemJSON;
use crate::{fetch, read_write};

use crate::args::CreateCommand;

const UNEXPECTED_ERR_HEADER: &str = "Unexpected error while";
const ERR_HEADER: &str = "Error while";
const PREMIUM_COMMAND: &str = "`cargo conf premium (0 or 1)`";

const NO_PREMIUM_ERR: &str = "This problem seems to be premium-only but you are registered as a free-user. Please run `cargo conf premium 1` if you are premium.";

const TEST_COMPILER_CONFIGURATION_ATTRIBUTE: &str = "#[cfg(test)]\n";
const PATTERNS_TO_GIVE_TEST_ATTRIBUTE: [&str; 2] = ["impl ", "struct "];

fn try_checking_if_user_is_premium() -> Result<bool, ()> {
    println!("checking if you're premium...");
    let premium_read_res: read_write::LRR<String> = read_write::try_read_variable("premium", '=');
    let premium = match premium_read_res {
        read_write::LRR::Found(value) => {
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
        read_write::LRR::FileMissing => {
            println!(
                "There is no .env file, please use {} to create it",
                PREMIUM_COMMAND
            );
            false
        }
        read_write::LRR::LineMissing => {
            println!(
                "Please specify if you're a premium leetcode user by running {}",
                PREMIUM_COMMAND
            );
            false
        }
        read_write::LRR::LineCorrupted => {
            println!(
                "The .env line with the premium variable seems to be corrupted. Please run {}",
                PREMIUM_COMMAND
            );
            false
        }
        read_write::LRR::UnexpectedError => {
            println!(
                "{} trying to read premium variable in .env",
                UNEXPECTED_ERR_HEADER.red().bold()
            );
            false
        }
    };

    Ok(premium)
}

pub async fn handle_create_command(create: CreateCommand) {
    let premium_res = try_checking_if_user_is_premium();
    let premium = match premium_res {
        Ok(premium) => premium,
        Err(_) => {
            return;
        }
    };

    let slug_read_res: read_write::LRR<(String, String)> =
        read_write::try_read_variable(&create.problem_id.to_string(), ',');
    let slug = match slug_read_res {
        read_write::LRR::Found((slug, prem)) => {
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
                        create.problem_id
                    );
                    return;
                }
            };
            if prem && !premium {
                println!("{}", NO_PREMIUM_ERR);
                return;
            }
            Some(slug)
        }
        read_write::LRR::FileMissing => {
            println!("File `slugs_and_ids` missing, creating it...");
            None
        }
        read_write::LRR::LineMissing => None,
        read_write::LRR::LineCorrupted => {
            println!("We did find problem {} locally but the line seems to be corrupted. Starting api call...", create.problem_id);
            None
        }
        read_write::LRR::UnexpectedError => {
            println!(
                "{} reading slug locally.",
                UNEXPECTED_ERR_HEADER.red().bold()
            );
            None
        }
    };

    println!(
        "Creating a solution file for problem {}...",
        create.problem_id
    );

    let slug = if let Some(slug) = slug {
        slug
    } else {
        let all_result = fetch::try_fetch_problem_list().await;
        let res = handle_problems_fetch_and_find(all_result, create.problem_id);
        if let Some((slug, prem)) = res {
            if prem && !premium {
                println!("{}", NO_PREMIUM_ERR);
                return;
            }
            slug
        } else {
            println!(
                "We could not find problem {}, please double-check its number.",
                create.problem_id
            );
            return;
        }
    };

    let content = fetch::try_fetch_content(&slug).await;

    let content = match content {
        Ok(val) => val,
        Err(e) => {
            match e {
                fetch::FetchContentErr::NotFound => {
                    println!(
                        "Problem {} doesn't seem to be available in Rust (T_T)",
                        create.problem_id
                    );
                }
                fetch::FetchContentErr::ReqwestErr(error) => {
                    println!(
                        "{} fetching content for problem {}: {}",
                        UNEXPECTED_ERR_HEADER.red().bold(),
                        create.problem_id,
                        error
                    );
                }
                fetch::FetchContentErr::ParseError(error) => {
                    println!(
                        "{} fetching content for problem {}: {}",
                        UNEXPECTED_ERR_HEADER.red().bold(),
                        create.problem_id,
                        error
                    );
                }
            }
            return;
        }
    };

    let content = apply_modifications_to_solution_file(content, true);
    let filename = format!("s{}_{}", create.problem_id, slug).replace('-', "_");
    let file_path = format!("./src/solutions/{}.rs", filename);

    let res = read_write::try_write_solution_template(&file_path, &content); // we check that it doesn't already exist

    match res {
        Ok(_) => {
            println!(
                "{} created and wrote to {}",
                "Successfully".cyan().bold(),
                file_path
            );
            // declares the newly created module file so that it is included in unit tests
            let res = read_write::try_append_solution_module(&filename);
            if let Err(e) = res {
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

fn apply_modifications_to_solution_file(
    content: String,
    should_add_solution_struct: bool,
) -> String {
    let mut content = if should_add_solution_struct {
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
