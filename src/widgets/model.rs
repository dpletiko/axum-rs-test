use crate::{db, error_handler::AppError};
use crate::schema::widgets;
use chrono::DateTime;
use chrono::offset::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = widgets)]
pub struct Widget {
    pub name: String,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = widgets)]
pub struct Widgets {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Widgets {
    pub fn all() -> Result<Vec<Self>, anyhow::Error> {
        let mut conn = db::connection()?;
        let widgets = widgets::table.load::<Widgets>(&mut conn)?;
        Ok(widgets)
    }

    pub fn find(id: i32) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let widget = widgets::table.filter(widgets::id.eq(id)).first(&mut conn)?;
        Ok(widget)
    }

    pub fn create(widget: Widget) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let widget = Widget::from(widget);
        let widget = diesel::insert_into(widgets::table)
            .values(widget)
            .get_result(&mut conn)?;
        Ok(widget)
    }

    pub fn update(id: i32, widget: Widget) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let widget = diesel::update(widgets::table)
            .filter(widgets::id.eq(id))
            .set(widget)
            .get_result(&mut conn)?;
        Ok(widget)
    }

    pub fn delete(id: i32) -> Result<usize, anyhow::Error> {
        let mut conn = db::connection()?;
        let res = diesel::delete(widgets::table.filter(widgets::id.eq(id))).execute(&mut conn)?;
        Ok(res)
    }
}

impl Widget {
    fn from(widget: Widget) -> Widget {
        Widget {
            name: widget.name,
            // created_at: widget.created_at,
            // updated_at: widget.updated_at
        }
    }
}
