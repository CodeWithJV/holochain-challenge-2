use blog_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn create_comment(comment: Comment) -> ExternResult<Record> {
    let comment_hash = create_entry(&EntryTypes::Comment(comment.clone()))?;
    //create link from post to comment here
    create_link(
        comment.post_hash.clone(),
        comment_hash.clone(),
        LinkTypes::PostToComments,
        (),
    )?;
    let record = get(comment_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created Comment".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn get_original_comment(original_comment_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_comment_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed get details response".to_string()
        ))),
    }
}

//add get_latest_comment here
#[hdk_extern]
pub fn get_latest_comment(original_comment_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_comment_hash, GetOptions::default())? else {
        return Ok(None);
    };
    let record_details = match details {
        Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
        Details::Record(record_details) => Ok(record_details),
    }?;
    match record_details.updates.last() {
        Some(update) => get_latest_comment(update.action_address().clone()),
        None => Ok(Some(record_details.record)),
    }
}

// delete_comment here
#[hdk_extern]
pub fn delete_comment(original_comment_hash: ActionHash) -> ExternResult<ActionHash> {
    let details = get_details(original_comment_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest("Comment not found".to_string())),
    )?;
    let record = match details {
        Details::Record(details) => Ok(details.record),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed get details response".to_string()
        ))),
    }?;
    let entry = record
        .entry()
        .as_option()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Comment record has no entry".to_string()
        )))?;
    let comment = <Comment>::try_from(entry)?;
    let links = get_links(
        GetLinksInputBuilder::try_new(comment.post_hash.clone(), LinkTypes::PostToComments)?
            .build(),
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

// add get_comments_for_post here
#[hdk_extern]
pub fn get_comments_for_post(post_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(post_hash, LinkTypes::PostToComments)?.build())
}


#[hdk_extern]
pub fn get_all_revisions_for_comment(
    original_comment_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let Some(Details::Record(details)) = get_details(original_comment_hash, GetOptions::default())?
    else {
        return Ok(vec![]);
    };
    let mut records = vec![details.record];
    for update in details.updates {
        let mut update_records = get_all_revisions_for_comment(update.action_address().clone())?;
        records.append(&mut update_records);
    }
    Ok(records)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCommentInput {
    pub previous_comment_hash: ActionHash,
    pub updated_comment: Comment,
}

#[hdk_extern]
pub fn update_comment(input: UpdateCommentInput) -> ExternResult<Record> {
    let updated_comment_hash = update_entry(input.previous_comment_hash, &input.updated_comment)?;
    let record = get(updated_comment_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly updated Comment".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn get_all_deletes_for_comment(
    original_comment_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_comment_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
        Details::Record(record_details) => Ok(Some(record_details.deletes)),
    }
}

#[hdk_extern]
pub fn get_oldest_delete_for_comment(
    original_comment_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_comment(original_comment_hash)? else {
        return Ok(None);
    };
    deletes.sort_by(|delete_a, delete_b| {
        delete_a
            .action()
            .timestamp()
            .cmp(&delete_b.action().timestamp())
    });
    Ok(deletes.first().cloned())
}


#[hdk_extern]
pub fn get_deleted_comments_for_post(
    post_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        post_hash,
        LinkTypes::PostToComments,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_comments_for_author(author: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(author, LinkTypes::AuthorToComments)?.build())
}

#[hdk_extern]
pub fn get_deleted_comments_for_author(
    author: AgentPubKey,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        author,
        LinkTypes::AuthorToComments,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}
