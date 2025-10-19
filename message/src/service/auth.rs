use crate::{common::enums::Gender, model::user::User, repository::user, vo::auth::RegisterRequest};
use snowflaker;
use anyhow::Result;
use crate::common::response::ApiErr;

pub async fn login(username: &str, password: &str) -> Result<i64> {
    let existed = user::select_user_by_name(username).await?;
    let exists = existed.is_some();
    if exists {
        let user = existed.unwrap();
        if user.password == password {
            Ok(user.id)
        } else {
            println!("密码错误");
            Err(ApiErr::Bad(400, "用户名不存在或者密码错误").into())
        }
    }else{
        println!("用户名不存在");
        Err(ApiErr::Bad(400, "用户名不存在或者密码错误").into())
    }
}


pub async fn wx_signup(open_id: String, union_id: String) -> Result<i64> {
    let id = snowflaker::next_id().unwrap();
    let user = User{id: id as i64, name: "微信用户".to_string(),
        open_id: Some(open_id), union_id: Some(union_id),
        ..Default::default()
    };
    user::insert_user(user).await
}

pub async fn signup(request: RegisterRequest) -> Result<i64> {
    let id = snowflaker::next_id().unwrap(); 
    let user: User = User{id: id as i64,
        name: request.username,
        password: request.password,
        gender: Gender::from(request.gender), 
        age: request.age,
        ..Default::default()
    };
    user::insert_user(user).await
}