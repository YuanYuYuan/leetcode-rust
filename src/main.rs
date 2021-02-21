use anyhow::{Result, bail};
use serde_json::{from_str, from_value, Value, json};
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::io::{self, Write};

const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms";

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatWithStatus {
    stat: Stat,
    difficulty: Difficulty,
    paid_only: bool,
    is_favor: bool,
    frequency: u32,
    progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    question_id: u32,
    #[serde(rename = "question__article__slug")]
    question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    question_hide: bool,
    total_acs: u32,
    total_submitted: u32,
    frontend_question_id: u32,
    is_new_question: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Difficulty {
    level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    content: String,
    stats: String,
    #[serde(rename = "codeDefinition")]
    code_definition: String,
    #[serde(rename = "sampleTestCase")]
    sample_test_case: String,
    #[serde(rename = "metaData")]
    meta_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: serde_json::Value,
    query: String,
}

const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        stats
        codeDefinition
        sampleTestCase
        metaData
    }
}"#;

impl Query {
    fn question_query(title_slug: &str) -> Query {
        Query {
            operation_name: "questionData".to_string(),
            variables: json!({ "titleSlug": title_slug }),
            query: QUESTION_QUERY_STRING.to_string(),
        }
    }
}

#[derive(Debug)]
struct Problem {
    id: u32,
    title: String,
    title_slug: String,
    content: String,
    code: CodeDefinition,
}

impl Problem {
    fn save(&self) -> Result<()> {
        let name = format!("{:04}_{}", self.id, self.title_slug.replace("-", "_"));
        let prob_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("problems");
        let sample_dir = prob_dir.join("sample");
        let output_dir = prob_dir.join(&name);
        if output_dir.exists() {
            bail!("The probelm {} had already been generated.", &name)
        } else {
            std::fs::create_dir_all(&output_dir)?;
        }

        // Generate the Cargo.toml
        std::fs::write(
            &output_dir.join("Cargo.toml"),
            std::fs::read_to_string(sample_dir.join("Cargo.toml"))?
                .replace("sample", &self.title_slug),
        )?;

        // Generate the sample source code
        std::fs::create_dir_all(&output_dir.join("src"))?;
        std::fs::write(
            &output_dir.join("src").join("lib.rs"),
            std::fs::read_to_string(sample_dir.join("src/lib.rs"))?
                .replace("__PROBLEM_ID__", &format!("{}", self.id))
                .replace("__PROBLEM_TITLE__", &self.title)
                .replace("__PROBLEM_DEFAULT_CODE__", &self.code.default_code)
        )?;

        // Generate the problem description
        std::fs::write(
            &output_dir.join("README.md"),
            std::fs::read_to_string(sample_dir.join("README.md"))?
                .replace("__PROBLEM_ID__", &format!("{}", self.id))
                .replace("__PROBLEM_TITLE__", &self.title)
                .replace("__PROBLEM_STATEMENT__", &self.content)
                .replace(
                    "__DISCUSS_LINK__",
                    &format!("https://leetcode.com/problems/{}/discuss", &self.title_slug),
                )
                .replace(
                    "__PROBLEM_LINK__",
                    &format!("https://leetcode.com/problems/{}", &self.title_slug),
                )
                .replace("__PROBLEM_DEFAULT_CODE__", &self.code.default_code)
        )?;

        println!("Successfully generate the problem {:?}", &name);
        Ok(())
    }
}

async fn get_problem_by_id(id: u32) -> Result<Problem> {
    let problem_list: Value = from_str(&reqwest::get(PROBLEMS_URL).await?.text().await?)?;
    let problem_stats: Vec<StatWithStatus> = from_value(problem_list["stat_status_pairs"].to_owned())?;
    let stat = problem_stats
        .into_iter()
        .find(|x| x.stat.frontend_question_id == id && !x.paid_only)
        .expect("Invalid ID or paid only.")
        .stat;
    let title = stat.question_title.expect("Empty title.");
    let title_slug = stat.question_title_slug.expect("Empty title slug.");

    let response: Value = from_str(
        &reqwest::Client::new()
            .post("https://leetcode.com/graphql")
            .json(&Query::question_query(&title_slug))
            .send()
            .await?
            .text()
            .await?
    )?;
    let question: Question = from_value(response["data"]["question"].to_owned())?;

    let availabe_codes: Vec<CodeDefinition> = from_str(&question.code_definition)?;
    let code_definition = availabe_codes
        .into_iter()
        .find(|x| x.value == "rust")
        .expect("Rust version not support.");

    Ok(Problem {
        id,
        title,
        title_slug,
        code: code_definition,
        content: question.content,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut id = String::new();
    print!("Enter the problem ID to initialize: ");
    io::stdout().flush()?;
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line");
    let problem = get_problem_by_id(id.trim().parse::<u32>()?).await?;
    problem.save()?;
    Ok(())
}
