use hdk::prelude::*;
use blog_integrity::*;

// Add your zome functions here
// ...

#[hdk_extern]
pub fn create_post(post: Post) -> ExternResult<Record> {
    let post_hash = create_entry(&EntryTypes::Post(post.clone()))?;
    let record = get(post_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("Could not find the newly created Post".to_string()))
    )?;

    // Add your Link creation code here!
    // ...

    Ok(record)
}

#[hdk_extern]
pub fn get_original_post(original_post_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_post_hash, GetOptions::default())? else {
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
pub struct UpdatePostInput {
    pub original_post_hash: ActionHash,
    pub previous_post_hash: ActionHash,
    pub updated_post: Post,
}

#[hdk_extern]
pub fn update_post(input: UpdatePostInput) -> ExternResult<Record> {
    let updated_post_hash = update_entry(input.previous_post_hash.clone(), &input.updated_post)?;

    // Add your link creation code here
    // ...

    let record = get(updated_post_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("Could not find the newly updated Post".to_string()))
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_post(original_post_hash: ActionHash) -> ExternResult<ActionHash> {
    // Add your Link deletion code here
    // ...

    delete_entry(original_post_hash)
}
