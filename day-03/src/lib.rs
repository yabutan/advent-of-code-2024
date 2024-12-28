use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Mul(i32, i32),
    Do,
    DoNot,
}

impl Token {
    pub fn parse_all(input: &str) -> anyhow::Result<Vec<Self>> {
        let (_, res) = parse_all_tokens(input)
            .map_err(|e| anyhow::anyhow!("Failed to parse input: {:?}", e))?;
        Ok(res)
    }
}

fn parse_all_tokens(mut input: &str) -> IResult<&str, Vec<Token>> {
    let mut list = Vec::new();
    while !input.is_empty() {
        input = match parse_token(input) {
            Ok((i, token)) => {
                list.push(token);
                i
            }
            Err(_) => &input[1..],
        }
    }

    Ok((input, list))
}

fn parse_mul(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = tag("mul(")(input)?;

    let (input, (a, b)) = separated_pair(
        map_res(digit1, str::parse),
        tag(","),
        map_res(digit1, str::parse),
    )(input)?;

    let (input, _) = tag(")")(input)?;

    Ok((input, (a, b)))
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((
        map(parse_mul, |(a, b)| Token::Mul(a, b)),
        map(tag("do()"), |_| Token::Do),
        map(tag("don't()"), |_| Token::DoNot),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mul() {
        let input = "mul(1,2)";
        let (i, list) = parse_mul(input).unwrap();
        assert_eq!(i, "");
        assert_eq!(list, (1, 2));
    }

    #[test]
    fn test_parse_all_tokens() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let (_, res) = parse_all_tokens(input).unwrap();
        assert_eq!(
            res,
            vec![
                Token::Mul(2, 4),
                Token::DoNot,
                Token::Mul(5, 5),
                Token::Mul(11, 8),
                Token::Do,
                Token::Mul(8, 5),
            ]
        );
    }
}
