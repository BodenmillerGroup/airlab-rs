use crate::web::Result;
use lib_core::ctx::Ctx;
use lib_core::model::member::{Member, MemberBmc, MemberFilter};
use lib_core::model::ModelManager;
use serde_json::json;

pub async fn get_member_id(
    ctx: &Ctx,
    mm: &ModelManager,
    group_id: i32,
    user_id: i32,
) -> Result<i32> {
    let filters: Vec<MemberFilter> = serde_json::from_value(json!([
        {
            "group_id": {"$eq":group_id},
            "user_id": {"$eq":user_id},
        }
    ]))?;
    let members: Vec<Member> = MemberBmc::list(ctx, mm, Some(filters), None).await?;
    let created_by = members.first().map_or(0, |o| o.id);
    Ok(created_by)
}
