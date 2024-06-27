# Challenge 2 - Links and Collections

In the last challenged we learned that update/delete actions do not modify the original entry, but instead create a new one and mark the old one as redundant.

In Holochain, it is common to retrieve entries using links instead of directly from an Action Hash. This method also helps to resolve the issue of old outdated entries were being easily accessible.

In this challenge we will learn how to manipulate links to achieve the following functionality in Holochain:

- [ ] 1. Retrieving entries within a collection
- [ ] 2. Retrieving the latest update of an entry
- [ ] 3. Deleting links
- [ ] 4. Retrieving entries that should correspond to another entry (ie: Post/Comments)

## Setup

#### 1. Fork this repo and clone it onto your local machine

#### 2. cd into `c-2` directory and run `nix develop`.

#### 3. run `npm i` to install package dependencies

#### 4. run `npm start` and open up the holochain playground

## Retrieving entries within a collection

In this project we are going to be developing a simple blogging app, where users can create blog posts, and comment on them.

Notice how each of the Agent windows are empty.

#### 1. Add two svelte components `CreatePost` and `AllPosts` to `ui/src/App.svelte`, save the file, and tab over into an Agent window

#### 2. Enter some text into each text field, and create a new post

If you look back over at the playground, similar to in the last challenge, this will have created a new **record** (action-entry pair) for the post.

However the creation of this post won't be reflected in the UI, and there will be an error instead.

Links will help us implement the retrieval of posts across agents to solve this issue.

#### 3. Navigate to `dnas/blog/zomes/coordinator/blog/src/post.rs` and paste the following code inside the `create_post` function

```rust
    let path = Path::from("all_posts");
    create_link(path.path_entry_hash()?, post_hash.clone(), LinkTypes::AllPosts, ())?;
```

The addition of these lines of code will create a link from an arbitrary 'point' on the DHT to the post. This point is called a collection and will help us retrieve all of the posts in the app from a single location.

#### 4. Restart the Holochain app, and open the playground

You should see a couple of things when you create a new post

- As usual, the action and entry will appear in the agents source chain, but there will also be a new **createLink** action
- Inside the dht-entries panel you will also see a link has been created that that points from the new collection (labled as **Unknown**). This link points to the Create Action, however as of Mid June 2024 this link points to another 'Unknown' point (This is a bug an will be fixed in a later version of the playground).

#### 5. Navigate to `ui/src/blog/blog/AllPosts.svelte` and take a look at the `fetchPosts` function

You should notice a few things here.

- The client is making a zome call to `get_all_posts`
- The zome call returns an array of links which are then converted into action hashes
- At the bottom of the component, we can see that each action hash is passed into a new component `PostDetail` which renders that post's entry.

To get our posts rendering, we need to add the zome function do that.

#### 6. Navigate to `dnas/blog/zomes/coordinator/blog/src/all_posts.rs` and paste following code

```rust
#[hdk_extern]
pub fn get_all_posts() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_posts");
    get_links(GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllPosts)?.build())
}

```

#### 7. Restart the Holochain app, and add a couple of new posts

You will see that each one shows up in the UI!

## Retrieving the latest update of an entry

#### 1. Update a post inside the UI

You will notice that the updates are not reflected in the UI.

This makes sense, as our links coming from the collection point towards the create action, and not the update action.

#### 2. navigate to to `dnas/blog/zomes/coordinator/blog/src/post.rs`

Find the update_post function and create a link of type `LinkTypes::PostUpdates`. It should point from the original post hash to the updated post hash.

#### 3. Navigate to `PostDetail.svelte`

Notice how the component takes in the post's hash as a prop, and makes a call to the zome function `get_original_post` inside the of `fetchPost` function.

#### 4. Replace the name of the zome function to `get_latest_post`

#### 5. Navigate to `post.rs`

Notice how the original `get_original_post` zome function takes in the action hash, and returns the entry corresponding to it.

Instead of returning this entry, we want to loop over each link, pointing from this action hash, find the record whos entry's timestamp is the most recent, and return it's action.

#### 6. Paste the following code

```rust
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
```

#### 7. Restart the holochain app

Now when you edit a post, the updated content displays correctly in the UI.

## Deleting links

#### 1. Try delete a post

It won't properly work. When we delete an entry, we also need to remove all the links that point to it's action.

Unlike regular entries, links can be permanently removed from the DHT.

#### 2. Inside the `delete_post` function of `post.rs` add code to delete the link

The code should be simialr to the following structure:

```rust
// Retrieve all the links of type `LinkTypes::AllPosts`

// Iterate over the links and convert them into action hashes using link.target.into_action_hash()

// If the action hash == to original_post_hash, call delete_link(link.create_link_hash)?;
```

#### 3. Restart the holochain app

Now when you delete posts, you won't be able to access them anymore

## Retrieving entries that correspond to another entry (ie: Post/Comments)

#### 1. Start by uncommenting the `CreateComment` and `CommentsForPost` components implemented near the bottom of `PostDetail.svelte`

#### 2. Create a comment.

Notice how a new record is added to the dht-entries panel and the agents source chain, but on the agents window, the comment doesn't show up.

To resolve this we need to do two things:

- When creating a comment we also need to create a link from its post to it.
- We need to implement a zome function `get_comments_for_post`, which returns all the links pointing from the post action to each comment. This zome function is being called by `CommentsForPost.svelte`.

#### 3. Inside the `create_comment function` of `comment.rs` add a line of code to create a link of `LinkTypes::PostToComments` from the post hash to the comment hash.

#### 4. Inside `comment.rs` add a zome function called `get_comments_for_post` it should return `ExternResult<Vec<Link>>`

#### . Restart the app, create a comment and update it

Similarly to with updating posts, updating a comment will add a new action to the agents source chain. However, these changes will not be reflected in the agent's window UI.

We could resolve this issue the same way we did with the posts; by creating a link from the create action to each new update action, however, there is another solution that we can use instead.

#### 5. Navigate to `CommentDetail.svelte` and modify the `fetchComment` function to call `get_latest_comment` instead of `get_original_comment`

#### 6. Navigate to `comment.rs` and paste in the following zome function

```rust
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
```

Its important to understand that each entry and action sits inside of a record, which also hold meta information about about the pair, as well as any deletes and updates associated with it.

When we call `get_latest_comment` from the client, it retrieves the record details for this action, grabs most recent update action, and recursively calls the function again with this retrieved action's hash as the parameter. It keeps doing this until it can't find a new update (meaning that the current update is the latest), and then returns the record for this update.

The upside of using this method for update retrieval is that it requires less storage to be used on the DHT, as no extra links need to be created. However the recursive nature of this method can make it take longer to retrieve information if there is a long chain of updates spread around the DHT. It is for this reason, that it is recommended to use links for retrieval.

#### 7. Restart the app, create a comment, and update it.

#### 8. To delete a comment, paste the following zome function inside `comment.rs`

```rust
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

```

This code block works more or less in the same way as `delete_post`, there's just a couple extra steps to get the links from the action hash of the post.

Well done! You made it to the end.
