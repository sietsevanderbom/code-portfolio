use nom::{
    Parser,
    IResult,
    branch::alt,
    character::complete::{
        alphanumeric1,
        char,
        multispace0,
        multispace1,
        space0,
        digit1,
        line_ending,
    }};
use nom::combinator::eof;
use nom_supreme::{
    parser_ext::ParserExt,
    error::ErrorTree,
    final_parser::final_parser,
    multi::collect_separated_terminated,
    tag::complete::tag,
    parse_from_str,
};
use crate::task::Task;


fn parse_name(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    alphanumeric1
        .terminated(char('('))
        .context("task-name")
        .parse(input)
}

fn parse_duration(input: &str) -> IResult<&str, usize, ErrorTree<&str>> {
    parse_from_str(digit1)
        .terminated(char(')'))
        .context("duration")
        .parse(input)
}

fn parse_eof(input: &str) -> IResult<&str, Vec<String>, ErrorTree<&str>> {
    let (input, _) = multispace0
        .terminated(eof)
        .context("end-of-file")
        .parse(input)?;

    Ok((input, vec![]))
}

fn parse_check_no_dependencies(input: &str) -> IResult<&str, Vec<String>, ErrorTree<&str>> {
    let (input, _) = space0
        .precedes(line_ending)
        .terminated(alphanumeric1)
        .complete()
        .peek()
        .terminated(space0)
        .terminated(line_ending)
        .context("check-no-dependencies")
        .parse(input)?;

    Ok((input, vec![]))
}

fn parse_dependencies(input: &str) -> IResult<&str, Vec<String>, ErrorTree<&str>> {
    let (input, _) = tag("after")
        .delimited_by(multispace1)
        .complete()
        .cut()
        .context("dependencies")
        .parse(input)?;

    parse_dependencies_array(input)
}

fn parse_dependencies_array(input: &str) -> IResult<&str, Vec<String>, ErrorTree<&str>> {
    collect_separated_terminated(
        alphanumeric1.parse_from_str(),
        char(',').delimited_by(multispace0),
        char(']').preceded_by(space0),
    )
        .preceded_by(char('[').terminated(space0).complete())
        .complete()
        .cut()
        .context("dependencies-array")
        .parse(input)
}

fn parse_optional_dependencies(input: &str) -> IResult<&str, Vec<String>, ErrorTree<&str>> {
    alt((
        parse_eof,
        parse_check_no_dependencies,
        parse_dependencies,
    ))
        .context("dependencies-test")
        .parse(input)
}

fn parse_unscheduled_task(input: &str) -> IResult<&str, Task, ErrorTree<&str>> {
    let (input, name) = parse_name(input)?;
    let (input, duration) = parse_duration(input)?;
    let (input, dependencies) = parse_optional_dependencies(input)?;

    let task = Task::new(name, duration, dependencies);

    Ok((input, task))
}

pub fn parse_job(input: &str) -> Result<Vec<Task>, ErrorTree<nom_supreme::final_parser::Location>> {
    final_parser(
        collect_separated_terminated(
            parse_unscheduled_task,
            multispace0,
            multispace0.all_consuming(),
        )
            .context("parse_job"),
    )(input)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_name() {
        assert_eq!(parse_name(r#"B(1) "#).unwrap(), ("1) ", "B"));
    }

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration(r#"1) after"#).unwrap(), (" after", 1));
    }

    #[test]
    fn test_parse_check_no_dependencies() {
        assert_eq!(parse_check_no_dependencies(r#"
C(1)"#).unwrap(), ("C(1)", vec![]));
        assert!(parse_check_no_dependencies(r#" after [D]"#).is_err());
    }

    #[test]
    fn test_parse_dependencies_and_parse_dependencies_array() {
        assert_eq!(parse_dependencies(r#" after [D
        ,
        E  ,   F   ] "#).unwrap(), (" ", vec!["D".to_string(), "E".to_string(), "F".to_string()]));
        assert!(parse_dependencies(r#" after [D
        ,
        E     F   ] "#).is_err());
        assert!(parse_dependencies(r#" after [D
        ,
        E   ,  F    "#).is_err());
    }

    #[test]
    fn test_parse_job_with_one_task() {
        assert_eq!(parse_job(r#"A(1)
"#).unwrap(), vec![Task::new("A", 1, vec![])]);
    }

    #[test]
    fn test_parse_job_with_two_tasks_without_dependencies() {
        assert_eq!(parse_job(r#"A(1)
B(1)
"#).unwrap(), vec![Task::new("A", 1, vec![]),
                   Task::new("B", 1, vec![])]);
    }

    #[test]
    fn test_parse_job() {
        assert_eq!(parse_job(r#"A(1)
B(1) after [A]
C(1)
  after [A]
D(1) after [B]
F(1) after
  [B,
   C]
G(1) after [C]
H(1) after [D, F]
I(1) after
  [F, G]"#).unwrap(), vec![Task::new("A", 1, vec![]),
                           Task::new("B", 1, vec!["A".to_string()]),
                           Task::new("C", 1, vec!["A".to_string()]),
                           Task::new("D", 1, vec!["B".to_string()]),
                           Task::new("F", 1, vec!["B".to_string(), "C".to_string()]),
                           Task::new("G", 1, vec!["C".to_string()]),
                           Task::new("H", 1, vec!["D".to_string(), "F".to_string()]),
                           Task::new("I", 1, vec!["F".to_string(), "G".to_string()])]);
    }

    #[test]
    fn test_parse_job_failure() {
        const MISSING_TASK_NAME: &str = r#"(1)"#;
        const MISSING_DURATION_OPENING_BRACKET: &str = r#"A1)"#;
        const MISSING_DURATION_CLOSING_BRACKET: &str = r#"A(1"#;
        const WRONG_SPELLING_AFTER_TAG: &str = r#"B(1) afer [A]"#;
        const MISSING_OPENING_SQUARE_BRACKET: &str = r#"C(1) after A, B]"#;
        const MISSING_CLOSING_SQUARE_BRACKET: &str = r#"C(1) after [A, B"#;
        const MISSING_COMMA_IN_DEPENDENCY_ARRAY: &str = r#"C(1) after [A B]"#;
        const MISSING_TASK_IN_DEPENDENCY_ARRAY: &str = r#"D(1) after [A,, C]"#;
        const MISSING_CLOSING_SQUARE_BRACKET_WITH_FREE_LINING_1: &str = r#"A(1)
B(1) after [A]
C(1)
  after [A]
D(1) after [B]
F(1) after
  [B,
   C
G(1) after [C]
H(1) after [D, F]
I(1) after
  [F, G]"#;

        const MISSING_CLOSING_SQUARE_BRACKET_WITH_FREE_LINING_2: &str = r#"A(1)
B(1) after [A]
C(1)
  after [A]
D(1) after [B]
F(1) after
  [B,
   C]
G(1) after [C]
H(1) after [D, F
I(1) after
  [F, G]"#;
        const MISSING_COMMA_WITH_FREE_LINING: &str = r#"A(1)
B(1) after [A]
C(1)
  after [A]
D(1) after [B]
F(1) after
  [B,
   C]
G(1) after [C]
H(1) after [D, F]
I(1) after
  [F G]"#;
        const WRONG_SPELLING_AFTER_WITH_FREE_LINING: &str = r#"A(1)
B(1) after [A]
C(1)
  after [A]
D(1) after [B]
F(1) after
  [B,
   C]
G(1) after [C]
H(1) after [D, F]
I(1) aftr
  [F G]"#;

        let jobs_with_errors = [
            MISSING_TASK_NAME,
            MISSING_DURATION_OPENING_BRACKET,
            MISSING_DURATION_CLOSING_BRACKET,
            WRONG_SPELLING_AFTER_TAG,
            MISSING_OPENING_SQUARE_BRACKET,
            MISSING_CLOSING_SQUARE_BRACKET,
            MISSING_COMMA_IN_DEPENDENCY_ARRAY,
            MISSING_TASK_IN_DEPENDENCY_ARRAY,
            MISSING_CLOSING_SQUARE_BRACKET_WITH_FREE_LINING_1,
            MISSING_CLOSING_SQUARE_BRACKET_WITH_FREE_LINING_2,
            MISSING_COMMA_WITH_FREE_LINING,
            WRONG_SPELLING_AFTER_WITH_FREE_LINING
        ];

        for job in jobs_with_errors {
            assert!(parse_job(job).is_err());
        }
    }
}