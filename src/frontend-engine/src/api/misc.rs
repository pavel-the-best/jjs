use super::{contest, prelude::*};

pub(super) fn toolchains_list(ctx: &Context) -> ApiResult<Vec<schema::Toolchain>> {
    let res = ctx
        .cfg
        .toolchains
        .iter()
        .map(|tc| schema::Toolchain {
            name: tc.title.clone(),
            id: tc.name.clone(),
        })
        .collect();
    Ok(res)
}

pub(super) fn get_contests(ctx: &Context) -> ApiResult<Vec<contest::Contest>> {
    let contest_cfg = &ctx.cfg.contests[0];
    Ok(vec![super::contest::Contest {
        title: contest_cfg.title.clone(),
        id: "TODO".to_string(),
    }])
}
