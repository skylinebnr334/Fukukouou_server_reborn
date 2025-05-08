use diesel::sql_types::Integer;

pub struct Round1DataColumn {
    pub id: Integer,
    pub team1 : Integer,
    pub team2 : Integer,
    pub  team3 : Integer,
    pub  team4 : Integer,
    pub  team5 : Integer,
    pub  team6 : Integer,
}
pub struct Round1IndexRound {
    pub id: Integer,
    pub current_stage : Integer,
}
pub struct Round1ScoreConfigDataColumn {
    pub id:Integer,
    pub correct:Integer,
    pub miss:Integer,
    pub ask_throw:Integer,
}