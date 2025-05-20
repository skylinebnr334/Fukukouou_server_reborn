use crate::schema::round1_info;
use crate::schema::round1_tokutendt;
use crate::schema::round1_data;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(ToSchema,Serialize,Deserialize)]
pub struct ErrorMsgStruct{
    pub error_shortmsg:String,
    pub error_msg:String,
}
#[derive(Queryable, Insertable, Deserialize, Serialize, Clone, Copy,ToSchema)]
#[diesel(table_name = round1_data)]
pub struct Round1DataColumn {
    pub id: i32,
    pub team1 : i32,
    pub team2 : i32,
    pub  team3 : i32,
    pub  team4 : i32,
    pub  team5 : i32,
    pub  team6 : i32,
}
#[derive(Queryable, Deserialize, Serialize,ToSchema)]

pub struct Round1DataReturnStruct{
    pub result_data:Vec<Round1DataColumn>,
}
#[derive(Queryable, Deserialize, Serialize,ToSchema)]

pub struct Round1DataReturnStruct_KOBETSU{
    pub result_data:Round1DataColumn,
}

#[derive(Queryable, Deserialize, Serialize)]

pub struct Round1ScoreSettingReturnStruct{
    pub result_data:Vec<Round1ScoreConfigDataColumn>,
}


#[derive(Queryable, Deserialize, Serialize)]

pub struct SuccessReturnJson{
    pub status:String
}

#[derive(Queryable, Insertable, Deserialize, Serialize, Clone, Copy)]
#[diesel(table_name = round1_info)]
pub struct Round1IndexRound {
    pub id: i32,
    pub current_stage : i32,
}

#[derive(Queryable, Insertable, Deserialize, Serialize, Clone, Copy)]
#[diesel(table_name = round1_tokutendt)]
pub struct Round1ScoreConfigDataColumn {
    pub id:i32,
    pub correct:i32,
    pub miss:i32,
    pub ask_throw:i32,
}

#[derive(Queryable, Deserialize, Serialize, Clone, Copy)]
pub struct Round1NextRoundDT{
    pub current_stage:i32
}

#[derive(Deserialize, IntoParams)]
pub struct TID{
    pub(crate) id:i32
}
impl TID{
    pub fn id(&self) -> i32{
        self.id
    }
}