use super::schema::{ContestId, problem::Problem};
use super::prelude::*;
pub(crate) struct Contest {
    pub title: String,
    pub id: ContestId,
}

#[juniper::object(Context = Context)]
impl Contest {
    /// E.g. "Berlandian Olympiad in Informatics. Finals. Day 3."
    fn title(&self) -> &str {
        &self.title
    }

    /// Configured by human, something readable like 'olymp-2019', or 'test-contest'
    fn id(&self) -> &str {
        &self.id
    }

    fn problems(&self, ctx: &Context) -> Vec<Problem> {
        let contest_cfg = ctx.cfg.contests.get(0).unwrap();
        contest_cfg
            .problems
            .iter()
            .map(|p| Problem {
                title: p.title.clone(),
                id: p.code.clone(),
            })
            .collect()
    }
}
