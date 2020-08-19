use ncube_search::{Limit, SearchQuery};
use rusqlite::ToSql;

mod account;
mod config;
mod investigation;
mod methodology;
mod process;
mod search;
mod segment;
mod source;
mod stat;
mod unit;
mod workspace;

pub use self::account::*;
pub use self::config::*;
pub use self::investigation::*;
pub use self::methodology::*;
pub use self::process::*;
pub use self::search::*;
pub use self::segment::*;
pub use self::source::*;
pub use self::stat::*;
pub use self::unit::*;
pub use self::workspace::*;

pub(crate) struct SearchQuerySqlite {
    query: SearchQuery,
}

impl SearchQuerySqlite {
    pub(crate) fn from(query: &SearchQuery) -> Self {
        Self {
            query: query.clone(),
        }
    }

    fn populate(&self, params: Vec<Box<dyn ToSql>>) -> (Vec<String>, Vec<Box<dyn ToSql>>) {
        let mut conditionals: Vec<String> = vec![];
        let mut params = params;
        let mut param_index = params.len() + 1;

        if let Some(terms) = &self.query.terms {
            conditionals.push(format!("unit_fts MATCH ?{}", param_index));
            params.push(Box::new(terms.clone()));
            param_index += 1;
        }

        for limit in &self.query.limits {
            match limit {
                Limit::Tag(term) => {
                    conditionals.push(format!("qt.label = ?{}", param_index));
                    params.push(Box::new(term.replacen("\"", "", 2).clone()));
                }
                Limit::Source(term) => {
                    conditionals.push(format!("u.source = ?{}", param_index));
                    params.push(Box::new(term.replacen("\"", "", 2).clone()));
                }
            };

            param_index += 1;
        }

        (conditionals, params)
    }

    pub(crate) fn to_sql(
        &self,
        tmpl: &str,
        params: Vec<Box<dyn ToSql>>,
    ) -> (String, Vec<Box<dyn ToSql>>) {
        let (conditionals, params) = self.populate(params);

        (tmpl.replacen("{}", &conditionals.join(" AND "), 2), params)
    }
}
