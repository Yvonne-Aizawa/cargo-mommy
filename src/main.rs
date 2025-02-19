#![allow(clippy::let_and_return)]

use std::process::Output;

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use serde::Deserialize;

const RESPONSES: &str = include_str!("../responses.json");
const AFFECTIONATE_TERM_PLACEHOLDER: &str = "AFFECTIONATE_TERM";
const MOMMYS_PRONOUN_PLACEHOLDER: &str = "MOMMYS_PRONOUN";
const MOMMYS_ROLE_PLACEHOLDER: &str = "MOMMYS_ROLE";

const AFFECTIONATE_TERMS_ENV_VAR: &str = "CARGO_MOMMYS_LITTLE";
const MOMMYS_PRONOUNS_ENV_VAR: &str = "CARGO_MOMMYS_PRONOUNS";
const MOMMYS_ROLES_ENV_VAR: &str = "CARGO_MOMMYS_ROLES";

const AFFECTIONATE_TERMS_DEFAULT: &str = "girl";
const MOMMYS_PRONOUNS_DEFAULT: &str = "her";
const MOMMYS_ROLES_DEFAULT: &str = "mommy";

#[derive(Deserialize)]
struct Responses {
    positive: Vec<String>,
    negative: Vec<String>,
    natural: Vec<String>,
}

enum ResponseType {
    Positive,
    Negative,
    Natural,
}

fn main() {
    // Ideally mommy would use ExitCode but that's pretty new and mommy wants
    // to support more little ones~
    let code = real_main().unwrap_or_else(|e| {
        eprintln!("Error: {:?}", e);
        -1
    });
    std::process::exit(code)
}

fn real_main() -> Result<i32, Box<dyn std::error::Error>> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let mut arg_iter = std::env::args();
    let _cargo = arg_iter.next();
    let _mommy = arg_iter.next();

    let mut cmd = std::process::Command::new(cargo);
    cmd.args(arg_iter);
    let output = cmd.output()?;
    eprintln!("\x1b[1m");
    if output.status.success() {
        if parse_cargo_output(&output) {
            eprintln!("{}", select_response(ResponseType::Natural))
        } else {
            eprintln!("{}", select_response(ResponseType::Positive))
        }
    } else {
        eprintln!("{}", select_response(ResponseType::Negative));
    }
    eprintln!("\x1b[0m");
    Ok(output.status.code().unwrap_or(-1))
}

fn select_response(response_type: ResponseType) -> String {
    let mut rng = StdRng::from_entropy();

    // Get mommy's options~
    let affectionate_terms = parse_options(AFFECTIONATE_TERMS_ENV_VAR, AFFECTIONATE_TERMS_DEFAULT);
    let mommys_pronouns = parse_options(MOMMYS_PRONOUNS_ENV_VAR, MOMMYS_PRONOUNS_DEFAULT);
    let mommys_roles = parse_options(MOMMYS_ROLES_ENV_VAR, MOMMYS_ROLES_DEFAULT);

    // Choose what mommy will say~
    let responses: Responses = serde_json::from_str(RESPONSES).expect("RESPONSES to be valid JSON");

    let response = match response_type {
        ResponseType::Positive => &responses.positive,
        ResponseType::Negative => &responses.negative,
        ResponseType::Natural => &responses.natural,
    }
    .choose(&mut rng)
    .expect("non-zero amount of responses");

    // Apply options to the message template~
    let response = apply_template(
        response,
        AFFECTIONATE_TERM_PLACEHOLDER,
        &affectionate_terms,
        &mut rng,
    );
    let response = apply_template(
        &response,
        MOMMYS_PRONOUN_PLACEHOLDER,
        &mommys_pronouns,
        &mut rng,
    );
    let response = apply_template(&response, MOMMYS_ROLE_PLACEHOLDER, &mommys_roles, &mut rng);

    // Done~!
    response
}
fn parse_cargo_output(output: &Output) -> bool {
    let string = String::from_utf8(output.stderr.clone()).expect("failed to convert stderr");

    string.contains("generated") && string.contains("warning")
}
fn parse_options(env_var: &str, default: &str) -> Vec<String> {
    std::env::var(env_var)
        .unwrap_or_else(|_| default.to_owned())
        .split('/')
        .map(|s| s.to_owned())
        .collect()
}

fn apply_template(input: &str, template_key: &str, options: &[String], rng: &mut StdRng) -> String {
    let mut last_position = 0;
    let mut output = String::new();
    for (index, matched) in input.match_indices(template_key) {
        output.push_str(&input[last_position..index]);
        output.push_str(options.choose(rng).unwrap());
        last_position = index + matched.len();
    }
    output.push_str(&input[last_position..]);
    output
}
#[cfg(test)]
#[test]
fn test() {
    // Uncomment if you want a failing test
    // panic!("oops!!");
}
