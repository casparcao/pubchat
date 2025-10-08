use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FoodResponse{
    pub id: i64,
    pub name: String,
    pub pick_count: i32,
    pub tags: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceRequest{
    pub tag: Option<i64>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DecideRequest{
    pub fid: i64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChoiceResponse{
    pub id: i64,
    pub name: String,
    pub tags: Vec<TagResponse>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagResponse{
    pub id: i64,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FoodRequest{
   pub name: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FoodTagRequest{
    pub fid: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChosenResponse{
    pub id: i64,
    pub name: String,
    pub time: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChosenRequest{
    pub name: Option<String>
}