use diesel::sql_types::Integer;

pub struct Round1_Data_Column{
    pub id: Integer,
    pub team1 : Integer,
    pub team2 : Integer,
    pub  team3 : Integer,
    pub  team4 : Integer,
    pub  team5 : Integer,
    pub  team6 : Integer,
}
