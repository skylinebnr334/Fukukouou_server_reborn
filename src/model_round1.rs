use crate::schema::round1_data;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "round1_data"]
pub struct Round1DataColumn {
    pub id: i32,
    pub team1 : i32,
    pub team2 : i32,
    pub  team3 : i32,
    pub  team4 : i32,
    pub  team5 : i32,
    pub  team6 : i32,
}
#[derive(Queryable, Deserialize, Serialize)]

pub struct Round1DataReturnStruct{
    pub result_data:Vec<Round1DataColumn>,
}
pub struct Round1IndexRound {
    pub id: i32,
    pub current_stage : i32,
}
pub struct Round1ScoreConfigDataColumn {
    pub id:i32,
    pub correct:i32,
    pub miss:i32,
    pub ask_throw:i32,
}