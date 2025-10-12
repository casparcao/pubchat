use crate::model::friend::Friend;
use crate::repository;
use crate::vo::friend::FriendResponse;
use anyhow::Result;
use crate::common::auth::User as AuthUser;

pub async fn add_friend(user: AuthUser, friend_id: i64) -> Result<i64> {
    // Check if friend relationship already exists
    if let Some(_) = repository::friend::select_friend_by_ids(user.id, friend_id).await? {
        // Relationship already exists, return success
        return Ok(0);
    }
    
    // Create new friend relationship
    let friend = Friend {
        id: snowflaker::next_id()? as i64,
        user_id: user.id,
        friend_id,
        status: 1, // Normal status
        ..Default::default()
    };
    
    let id = repository::friend::insert_friend(friend).await?;
    Ok(id)
}

pub async fn get_friend_list(user: AuthUser) -> Result<Vec<FriendResponse>> {
    let friends = repository::friend::select_friends_by_user_id(user.id).await?;
    
    let friend_responses: Vec<FriendResponse> = friends
        .into_iter()
        .map(|f| FriendResponse {
            id: f.id,
            name: f.name,
            avatar: f.avatar,
        })
        .collect();
        
    Ok(friend_responses)
}

pub async fn remove_friend(user: AuthUser, friend_id: i64) -> Result<bool> {
    let rows_affected = repository::friend::delete_friend(user.id, friend_id).await?;
    Ok(rows_affected > 0)
}