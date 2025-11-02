use anyhow::{Ok, Result};
use crate::{model::food::{Food, Tag}, vo::food::ChoiceResponse};
use core::{auth::User, response::ApiErr};
use core::request::Page;
use crate::model::food::{UserFoodChoice, UserFoodChoiceDetail};
use crate::repository::food;
use crate::vo::food::{ChosenRequest, ChosenResponse, DecideRequest, FoodRequest, FoodResponse, FoodTagRequest, TagResponse};

pub async fn choice(tag: Option<i64>) -> Result<ChoiceResponse> {
    //如果指定了标签，则该标签下的所有美食中随机选择一个。
    //否则从所有美食下随机选择一个
    let total_count : i32;
    if tag.is_none(){
        //未指定标签
        total_count = food::count().await?;
    }else{
        total_count = food::count_by_tag(tag.unwrap()).await?;
    }
    if total_count < 1 {
        return Err(ApiErr::Error("哎呀...我的饭呢?".to_string()).into());
    }
    let offset = 0;
    let choice: Option<Food>;
    if tag.is_none(){
        //从所有美食里面挑选
        choice = food::select_one(offset).await?;
    }else{
        choice = food::select_one_by_tag(tag.unwrap(), offset).await?;
    }
    if choice.is_none(){
        return Err(ApiErr::Bad(400, "哎呀...要不这顿省省？".to_string()).into());
    }
    let choice = choice.unwrap();
    let tags: Vec<Tag> = food::select_tags(choice.id).await?;
    let tags: Vec<TagResponse> = tags.iter().map(|t| {
        TagResponse{id: t.id, name: t.name.clone()}
    }).collect();

    Ok(ChoiceResponse { tags, id:choice.id, name: choice.name })
}

pub async fn populars() -> Result<Vec<FoodResponse>>{
    let foods = food::select_populars().await?;
    Ok(convert(foods))
}

fn convert(foods: Vec<Food>) -> Vec<FoodResponse> {
    foods.iter()
        .map(|f| FoodResponse { id: f.id, name: f.name.clone(), pick_count: f.pick_count, tags: [].to_vec() })
        .collect()
}

pub(crate) async fn list(page: Page, param: FoodRequest) -> Result<(Vec<FoodResponse>, i64)>{
    let result: (Vec<Food>, i64) = food::select_page(page, param).await?;
    Ok((convert(result.0), result.1))
}

pub(crate) async fn tags(p0: FoodTagRequest) -> Result<Vec<TagResponse>>{
    let result: Vec<Tag> = food::select_tags(p0.fid).await?;
    Ok(result.iter().map(|t| TagResponse{id: t.id, name: t.name.clone()}).collect())
}

pub(crate) async fn decide(claims: &User, body: DecideRequest) -> Result<()>{
    let id = snowflaker::next_id().unwrap();
    //保存一条 user_food_choice记录。
    //增加food.pick_count数量
    let choice = UserFoodChoice{
        id: id as i64,
        uid: claims.id,
        fid: body.fid
    };
    food::insert_choice(choice).await?;
    food::incr_pick_count(body.fid).await?;
    Ok(())
}

pub(crate) async fn chosen(uid: i64, page: Page, param: ChosenRequest) -> Result<(Vec<ChosenResponse>,i64)>{
    let result: (Vec<UserFoodChoiceDetail>, i64) = food::select_chosen(uid, page, param).await?;
    Ok((convert2(result.0), result.1))
}

fn convert2(origin: Vec<UserFoodChoiceDetail>) -> Vec<ChosenResponse> {
    origin.iter()
        .map(|f| ChosenResponse {
            id: f.id, name: f.name.clone(), time: f.time.format("%Y-%m-%d %H:%M:%S").to_string()
        })
        .collect()
}
