use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetLeagueSeasonsRes {
    pub complete: bool,
    pub name: String,
    pub season: i32,
    pub round: i32,
}

#[derive(Debug, Deserialize)]
pub struct GetLeagueLeaderBoardRes {
    #[serde(rename = "Name")]
    pub name: String,
    pub player_id: String,
    #[serde(rename = "Wins")]
    pub wins: i32,
    #[serde(rename = "Losses")]
    pub losses: i32,
    #[serde(rename = "Rank_Stat_Conquest")]
    pub rank_conquest: String,
}

#[derive(Debug, Deserialize)]
pub struct GetMatchIdsByQueueRes {
    #[serde(rename = "Active_Flag")]
    pub is_active: String,
    #[serde(rename = "Match")]
    pub match_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetMatchDetailsRes {
    #[serde(rename = "Ban1")]
    pub ban1: String,
    #[serde(rename = "Ban2")]
    pub ban2: String,
    #[serde(rename = "Ban3")]
    pub ban3: String,
    #[serde(rename = "Ban4")]
    pub ban4: String,
    #[serde(rename = "Ban5")]
    pub ban5: String,
    #[serde(rename = "Ban6")]
    pub ban6: String,
    #[serde(rename = "Ban7")]
    pub ban7: String,
    #[serde(rename = "Ban8")]
    pub ban8: String,
    #[serde(rename = "Ban9")]
    pub ban9: String,
    #[serde(rename = "Ban10")]
    pub ban10: String,

    #[serde(rename = "Item_Active_1")]
    pub active1: String,
    #[serde(rename = "Item_Active_2")]
    pub active2: String,

    #[serde(rename = "Item_Purch_1")]
    pub item1: String,
    #[serde(rename = "Item_Purch_2")]
    pub item2: String,
    #[serde(rename = "Item_Purch_3")]
    pub item3: String,
    #[serde(rename = "Item_Purch_4")]
    pub item4: String,
    #[serde(rename = "Item_Purch_5")]
    pub item5: String,
    #[serde(rename = "Item_Purch_6")]
    pub item6: String,

    #[serde(rename = "Match")]
    pub match_id: u64,

    #[serde(rename = "playerId")]
    pub player_id: String,
    #[serde(rename = "playerName")]
    pub player_name: String,

    #[serde(rename = "GodId")]
    pub god_id: u64,

    #[serde(rename = "Reference_Name")]
    pub god: String,

    #[serde(rename = "Win_Status")]
    pub team: String,

}
