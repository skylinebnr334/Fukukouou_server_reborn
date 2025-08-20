use crate::schema::round1_questions;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Insertable, Deserialize, Serialize, Clone,ToSchema)]
#[diesel(table_name = round1_questions)]
pub struct Round1QuestionDataColumn{
    pub stageno:i32,
    pub question:String,
    pub answer:String,
    pub comment:String,
}