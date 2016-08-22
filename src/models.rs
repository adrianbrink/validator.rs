// use super::schema::*;
//
// #[derive(Debug, Deserialize, Qeuryable, PartialEq, Serialize)]
// pub struct Summoner {
//     pub id: i32,
//     pub name: String,
//     pub profileIconId: i32,
//     pub summonerLevel: i32,
//     pub revisionDate: i32,
// }
//
// #[insertable_into(summoners)]
// pub struct NewSummoner<'a> {
//     pub id: i32,
//     pub name: &'a str,
//     pub profileIconId: i32,
//     pub summonerLevel: i32,
//     pub revisionDate: i32,
// }
