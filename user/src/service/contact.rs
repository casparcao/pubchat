use crate::model::contact::Contact;
use crate::repository;
use crate::vo::contact::ContactResponse;
use anyhow::Result;
use crate::common::auth::User as AuthUser;

pub async fn add(user: AuthUser, there: i64) -> Result<i64> {
    // Check if friend relationship already exists
    if let Some(_) = repository::contact::select_by_ids(user.id, there).await? {
        // Relationship already exists, return success
        return Ok(0);
    }
    
    // Create new friend relationship
    let contact = Contact {
        id: snowflaker::next_id()? as i64,
        here: user.id,
        there,
        status: 1, // Normal status
        ..Default::default()
    };
    
    let id = repository::contact::insert(contact).await?;
    Ok(id)
}

pub async fn get_list(user: AuthUser) -> Result<Vec<ContactResponse>> {
    let friends = repository::contact::select_by_here(user.id).await?;
    let friend_responses: Vec<ContactResponse> = friends
        .into_iter()
        .map(|f| ContactResponse {
            id: f.id,
            name: f.name,
            avatar: f.avatar,
        })
        .collect();
    Ok(friend_responses)
}

pub async fn remove(user: AuthUser, there: i64) -> Result<bool> {
    let rows_affected = repository::contact::delete(user.id, there).await?;
    Ok(rows_affected > 0)
}