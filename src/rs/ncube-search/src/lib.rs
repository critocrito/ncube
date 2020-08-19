#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::clone::Clone;
use std::default::Default;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Limit {
    Tag(String),
    Source(String),
}

impl Limit {
    pub fn new_tag(term: &str) -> Self {
        Self::Tag(term.to_string())
    }

    pub fn new_source(term: &str) -> Self {
        Self::Source(term.to_string())
    }
}

impl std::string::ToString for Limit {
    fn to_string(&self) -> String {
        match self {
            Self::Tag(term) => format!("tag:{}", term),
            Self::Source(term) => format!("source:{}", term),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SearchQuery {
    pub terms: Option<String>,
    pub limits: Vec<Limit>,
}

impl SearchQuery {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn append_term(&mut self, term: &str) {
        match &self.terms {
            Some(terms) => {
                let mut new_terms = terms.clone();
                new_terms.push_str(" ");
                new_terms.push_str(term);
                self.terms.replace(new_terms.clone());
            }

            None => {
                self.terms = Some(term.to_string());
            }
        };
    }

    pub fn append_limit(&mut self, limit: Limit) {
        self.limits.push(limit);
    }
}

impl std::string::ToString for SearchQuery {
    fn to_string(&self) -> String {
        let mut terms: Vec<String> = vec![];

        if let Some(term) = &self.terms {
            terms.push(term.clone());
        }

        for limit in &self.limits {
            terms.push(limit.to_string());
        }

        terms.join(" ")
    }
}

#[derive(Parser)]
#[grammar = "search.pest"]
pub struct SearchParser;

pub fn parse_query(input: &str) -> SearchQuery {
    let query = SearchParser::parse(Rule::query, input).unwrap();

    let mut result = SearchQuery::new();

    for expr in query {
        match expr.as_rule() {
            Rule::limit => {
                let mut inner_rules = expr.into_inner();
                let selector = inner_rules.next().unwrap().as_rule();
                let term = inner_rules.next().unwrap().as_str();

                let limit = match selector {
                    Rule::tag => Limit::new_tag(term),
                    Rule::source => Limit::new_source(term),
                    _ => unreachable!(),
                };

                result.append_limit(limit);
            }

            Rule::term => {
                let term = expr.as_str();
                result.append_term(term);
            }

            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_single_term() {
        let input = "term";
        let query = parse_query(&input);
        assert_eq!(query.terms, Some(input.to_string()));
        assert!(query.limits.is_empty());
    }

    #[test]
    fn test_a_compund_terms() {
        let input = "term1 term2";
        let query = parse_query(&input);
        assert_eq!(query.terms, Some(input.to_string()));
        assert!(query.limits.is_empty());
    }

    #[test]
    fn test_a_quoted_term() {
        let input = r#""term1 term2""#;
        let query = parse_query(&input);
        assert_eq!(query.terms, Some(input.to_string()));
        assert!(query.limits.is_empty());
    }

    #[test]
    fn test_tag_selector() {
        let input = "tag:term";
        let query = parse_query(&input);
        assert_eq!(query.limits, vec![Limit::new_tag("term")]);
        assert!(query.terms.is_none());
    }

    #[test]
    fn test_tag_selector_with_compound_term() {
        let input = r#"tag:"term1 term2""#;
        let query = parse_query(&input);
        assert_eq!(query.limits, vec![Limit::new_tag(r#""term1 term2""#)]);
        assert!(query.terms.is_none());
    }

    #[test]
    fn test_source_selector() {
        let input = "source:term";
        let query = parse_query(&input);
        assert_eq!(query.limits, vec![Limit::new_source("term")]);
        assert!(query.terms.is_none());
    }

    #[test]
    fn test_advanced_query() {
        let input = "source:youtube_video term1 term2 tag:greg term3";
        let query = parse_query(&input);
        assert_eq!(query.terms, Some("term1 term2 term3".to_string()));
        assert_eq!(
            query.limits,
            vec![Limit::new_source("youtube_video"), Limit::new_tag("greg")]
        );
    }
}
