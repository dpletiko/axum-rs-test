use crate::{db, error_handler::AppError};
use crate::schema::widgets;
use chrono::DateTime;
use chrono::offset::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = widgets)]
pub struct CreateWidget {
    pub name: String,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = widgets)]
pub struct Widget {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Widget {
    pub fn all() -> Result<Vec<Self>, anyhow::Error> {
        let mut conn = db::connection()?;
        let widgets = widgets::table.load::<Widget>(&mut conn)?;
        Ok(widgets)
    }

    pub fn find(id: i32) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let widget = widgets::table.filter(widgets::id.eq(id)).first(&mut conn)?;
        Ok(widget)
    }

    pub fn create(widget: CreateWidget) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let widget = CreateWidget::from(widget);
        let widget = diesel::insert_into(widgets::table)
            .values(widget)
            .get_result(&mut conn)?;
        Ok(widget)
    }

    pub fn update(id: i32, widget: CreateWidget) -> Result<Self, anyhow::Error> {
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

impl CreateWidget {
    fn from(widget: CreateWidget) -> CreateWidget {
        CreateWidget {
            name: widget.name,
            // created_at: widget.created_at,
            // updated_at: widget.updated_at
        }
    }
}
