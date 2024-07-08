use hdk::prelude::*;
use blog_integrity::*;

// Paste your zome functions here
// ...

#[hdk_extern]
pub fn get_latest_post(original_post_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(original_post_hash.clone(), LinkTypes::PostUpdates)?.build()
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_post_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("No action hash associated with link".to_string())
                    )
                )?
        }
        None => original_post_hash.clone(),
    };
    get(latest_post_hash, GetOptions::default())
}

#[hdk_extern]
pub fn create_post(post: Post) -> ExternResult<Record> {
    let post_hash = create_entry(&EntryTypes::Post(post.clone()))?;
    let record = get(post_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("Could not find the newly created Post".to_string()))
    )?;

    // Paste your Link creation code here!
    // ...
    let path = Path::from("all_posts");
    create_link(path.path_entry_hash()?, post_hash.clone(), LinkTypes::AllPosts, ())?;

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

    // Paste your link creation code here
    // ...
    create_link(
        input.original_post_hash.clone(),
        updated_post_hash.clone(),
        LinkTypes::PostUpdates,
        ()
    )?;

    let record = get(updated_post_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("Could not find the newly updated Post".to_string()))
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_latest_post(original_post_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(original_post_hash.clone(), LinkTypes::PostUpdates)?.build()
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_post_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("No action hash associated with link".to_string())
                    )
                )?
        }
        None => original_post_hash.clone(),
    };
    get(latest_post_hash, GetOptions::default())
}

#[hdk_extern]
pub fn delete_post(original_post_hash: ActionHash) -> ExternResult<ActionHash> {
    // Add Link deletion code here
    // ...
    let path = Path::from("all_posts");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllPosts)?.build()
    )?;
    for link in links {
        if let Some(hash) = link.target.into_action_hash() {
            if hash == original_post_hash {
                delete_link(link.create_link_hash)?;
            }
        }
    }

    delete_entry(original_post_hash)
}
