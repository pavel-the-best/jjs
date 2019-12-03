use super::{super::prelude::*,  ProblemId};

#[derive(GraphQLObject)]
pub(crate) struct Problem {
    /// Problem title as contestants see, e.g. "Find max flow".
    pub title: String,
    /// Problem external id (aka problem code) as contestants see. This is usually one letter or
    /// something similar, e.g. 'A' or '3F'.
    pub id: ProblemId,
}

impl<'a> From<&'a cfg::Problem> for Problem {
    fn from(p: &'a cfg::Problem) -> Self {
        Self {
            title: p.title.clone(),
            id: p.code.clone(),
        }
    }
}