use crate::schema::round2_data;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Queryable, Insertable, Deserialize, Serialize, Clone,ToSchema)]
#[diesel(table_name = round2_data)]
pub struct Round2DataColumn{
    pub team_id:i32,
    pub current_phase:i32,
    pub latest_down_num:i32,
    pub miss_timing:i32,
}