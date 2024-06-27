use hdk::prelude::*;
use blog_integrity::*;

#[hdk_extern]
pub fn create_comment(comment: Comment) -> ExternResult<Record> {
    let comment_hash = create_entry(&EntryTypes::Comment(comment.clone()))?;
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
