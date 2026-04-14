use crate::model::base::{CommonIden, DbBmc, TimestampIden};
use modql::field::{SeaField, SeaFields};
use sea_query::IntoIden;
use time::OffsetDateTime;

fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn prep_fields_for_create<MC>(fields: &mut SeaFields, user_id: i64)
where
    MC: DbBmc,
{
    if MC::has_owner_id() {
        fields.push(SeaField::new(CommonIden::OwnerId.into_iden(), user_id));
    }
    if MC::has_timestamps() {
        add_timestamps_for_create(fields, user_id);
    }
}

pub fn prep_fields_for_update<MC>(fields: &mut SeaFields, user_id: i64)
where
    MC: DbBmc,
{
    if MC::has_timestamps() {
        add_timestamps_for_update(fields, user_id);
    }
}

fn add_timestamps_for_create(fields: &mut SeaFields, user_id: i64) {
    let now = now_utc();
    fields.push(SeaField::new(TimestampIden::Cid, user_id));
    fields.push(SeaField::new(TimestampIden::Ctime, now));

    fields.push(SeaField::new(TimestampIden::Mid, user_id));
    fields.push(SeaField::new(TimestampIden::Mtime, now));
}

fn add_timestamps_for_update(fields: &mut SeaFields, user_id: i64) {
    let now = now_utc();
    fields.push(SeaField::new(TimestampIden::Mid, user_id));
    fields.push(SeaField::new(TimestampIden::Mtime, now));
}
