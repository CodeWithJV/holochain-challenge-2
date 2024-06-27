use hdk::prelude::*;
use blog_integrity::*;

#[hdk_extern]
pub fn create_comment(comment: Comment) -> ExternResult<Record> {
    let comment_hash = create_entry(&EntryTypes::Comment(comment.clone()))?;
    create_link(comment.post_hash.clone(), comment_hash.clone(), LinkTypes::PostToComments, ())?;
    let record = get(comment_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("Could not find the newly created Comment".to_string()))
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_original_comment(original_comment_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_comment_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => {
            Err(wasm_error!(WasmErrorInner::Guest("Malformed get details response".to_string())))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCommentInput {
    pub previous_comment_hash: ActionHash,
    pub updated_comment: Comment,
}

#[hdk_extern]
pub fn update_comment(input: UpdateCommentInput) -> ExternResult<Record> {
    let updated_comment_hash = update_entry(input.previous_comment_hash, &input.updated_comment)?;
    let record = get(updated_comment_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("Could not find the newly updated Comment".to_string()))
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_latest_comment(original_comment_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_comment_hash, GetOptions::default())? else {
        return Ok(None);
    };
    let record_details = (match details {
        Details::Entry(_) => { Err(wasm_error!(WasmErrorInner::Guest("Malformed details".into()))) }
        Details::Record(record_details) => Ok(record_details),
    })?;
    match record_details.updates.last() {
        Some(update) => get_latest_comment(update.action_address().clone()),
        None => Ok(Some(record_details.record)),
    }
}

#[hdk_extern]
pub fn delete_comment(original_comment_hash: ActionHash) -> ExternResult<ActionHash> {
    let details = get_details(original_comment_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("Comment not found".to_string()))
    )?;
    let record = (match details {
        Details::Record(details) => Ok(details.record),
        _ => {
            Err(wasm_error!(WasmErrorInner::Guest("Malformed get details response".to_string())))
        }
    })?;
    let entry = record
        .entry()
        .as_option()
        .ok_or(wasm_error!(WasmErrorInner::Guest("Comment record has no entry".to_string())))?;
    let comment = <Comment>::try_from(entry)?;
    let links = get_links(
        GetLinksInputBuilder::try_new(comment.post_hash.clone(), LinkTypes::PostToComments)?.build()
    )?;
    for link in links {
        if let Some(action_hash) = link.target.into_action_hash() {
            if action_hash == original_comment_hash {
                delete_link(link.create_link_hash)?;
            }
        }
    }
    delete_entry(original_comment_hash)
}

#[hdk_extern]
pub fn get_comments_for_post(post_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(post_hash, LinkTypes::PostToComments)?.build())
}
