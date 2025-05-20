use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Deserialize, Serialize, Clone)]
#[diesel(table_name = round1_questions)]
pub struct Round1QuestionDataColumn{
    pub stageno:i32,
    pub question:String,
    pub answer:String,
    pub comment:String,
}