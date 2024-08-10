# Challenge 2 - Links and Collections

In this challenge we will learn how to manipulate links to achieve the following functionality in Holochain:

- [ ] 1. Retrieving entries within a collection
- [ ] 2. Retrieving the latest update of an entry
- [ ] 3. Deleting links
- [ ] 4. Retrieving entries that should correspond to another entry (ie: Post/Comments)

## Setup

1. Fork this repo and clone it onto your local machine
2. cd into `holochain-challenge-2` directory and run `nix develop`.
3. run `npm i` to install package dependencies
4. run `npm start` and open up the holochain playground

## Retrieving entries within a collection

In this project we are going to be developing a simple blogging app, where users can create blog posts, and comment on them.

Notice how each of the Agent windows are empty.

#### 1. Link to some pre-built UI components

Copy and paste the following code into `ui/src/App.svelte`, save the file, and tab over into an Agent window

```svelte
    <CreatePost author={client.myPubKey} />
    <AllPosts />
```

Read through the code in the components to get an idea of how they work.

In your Agent browsers you should see a form and an error message.


#### 2. Enter some text into each text field, and create a new post

If you look back over at the playground, similar to in the last challenge, this will have created a new **record** (action-entry pair) for the post.

However the creation of this post won't be reflected in the UI, and there will be an error instead.

Links will help us implement the retrieval of posts across agents to solve this issue.

#### 3. Create a link to the post on creation

Navigate to `dnas/blog/zomes/coordinator/blog/src/post.rs` and paste the following code inside the `create_post` function

```rust
    let path = Path::from("all_posts");
    create_link(path.path_entry_hash()?, post_hash.clone(), LinkTypes::AllPosts, ())?;
```

The addition of these lines of code will create a link from an arbitrary 'point' on the DHT to the post. This point is called a collection and will help us retrieve all of the posts in the app from a single location.

Restart the Holochain app, and open the playground

You should see a couple of things when you create a new post

- As usual, the action and entry will appear in the agent's source chain, but there will also be a new **createLink** action
- Inside the dht-entries panel you will also see a link has been created that points from the new anchor (labled as **Unknown**) to the create action.

#### 4. Get all posts

Navigate to `ui/src/blog/blog/AllPosts.svelte` and take a look at the `fetchPosts` function

You should notice a few things here.

- The client is making a zome call to `get_all_posts`
- The zome call returns an array of links which are then converted into action hashes
- At the bottom of the component, we can see that each action hash is passed into a new component `PostDetail` which renders that post's entry.

To get our posts rendering, we need to add the zome function do that.

Navigate to `dnas/blog/zomes/coordinator/blog/src/all_posts.rs` and paste following code

```rust
#[hdk_extern]
pub fn get_all_posts() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_posts");
    get_links(GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllPosts)?.build())
}
```

The addition of these lines of get all links from an arbitrary point on the DHT (A path called "all_posts" in this case). Notice how this matches up with `create_link` code in the `create_post` zome function.

Restart the Holochain app, and add a couple of new posts

You will see that each one shows up in the UI!

Note: if you see the following error on startup, just reload the browser with ctrl-r

```
Error fetching the posts: Error: Error invoking remote method 'sign-zome-call': TypeError: Cannot read properties of undefined (reading 'agentPubKey').
```


## Retrieving the latest update of an entry

#### 1. Update a post inside the UI and refresh the window

You can update a post by clicking on the pencil icon and changing the text.

You will notice that the updates are not reflected in the UI.

This is because the links from get_all_posts are the links from the initial create action, so they point to the initial entry, not the updated one.

#### 2. Create a link from the original post to the updated post.

Navigate to to `dnas/blog/zomes/coordinator/blog/src/post.rs` and find the update_post function. Then add the following code snippet

```rust
create_link(
    input.original_post_hash.clone(),
    updated_post_hash.clone(),
    LinkTypes::PostUpdates,
    ()
)?;
```

#### 3. Restart the app, create a post, and update it a couple of times

Can you see the CreateLink actions in the source chain after each update? If you look at the contents of the action the base_Address will be the same, pointing to the original post action.

The post details won't be visible in the UI, let's fix that now.

#### 4. Update the UI to get the most recent post

Navigate to `PostDetail.svelte`  and change the zome call inside `fetchPost` from `get_original_post` to `get_latest_post`

Navigate to `dnas/blog/zomes/coordinator/src/post.rs` and add the following zome function

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
Go through the function line by line and look up the docs for any functions that are new to you.

Remember we need to restart the Holochain app as we've edited zome code.

Now when you edit a post, the updated content displays correctly in the UI.

## Deleting links

#### 1. Try delete a post

It won't properly work. When we delete an entry, we also need to remove all the links that point to it's action.

Unlike regular entries, links can be permanently removed from the DHT.

#### 2. Delete the link to posts when deleting an entry

Inside `dnas/zomes/coordinator/blog/src/post.rs`, add the following code inside the `delete_post` function

```rust
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
```

#### 3. Restart the holochain app

Now when you delete posts, you won't see them in the UI.

## Retrieving entries that correspond to another entry (ie: Post/Comments)

#### 1. Start by uncommenting the comment-section code found near the bottom of the `PostDetail.svelte` component

#### 2. Create a comment.

Ignore the zome function not found error and try creating a comment.

Notice how a new record is added to the dht-entries panel and the agents source chain, but on the agents window, the comment doesn't show up.

To resolve this we need to do two things:

- When creating a comment we also need to create a link from its post to itself.
- We need to implement a zome function `get_comments_for_post`, which returns all the links pointing from the post action to each comment. This zome function is being called by `CommentsForPost.svelte`.

#### 3. Implement comment creation and retrevial

First have a look in the integrity code to get familiar with the entries and link types.

You can ignore most of the code in `dnas/blog/zomes/integrity/blog/src/lib.rs` but take a look at the EntryTypes and LinkTypes.

The structure of the EntryTypes is defined in `comment.rs` and `post.rs` in the same folder. You can ignore the validation code for now, we will cover that in a future challenge.

When creating a comment we want to create a link from the post to the comment.
Then we want to make make a zome function get all comments for a post.

Try do it without using the hints!

<details>
<summary>
Hint for creating the link
</summary>

Add this block of code to the create_comment function inside `dnas/blog/zomes/coordinator/blog/src/comment.rs`

```rust
    create_link(comment.post_hash.clone(), comment_hash.clone(), LinkTypes::PostToComments, ())?;
```

</details>

<details>
<summary>
Hint for retrieving the links
</summary>

Add this zome function to `dnas/blogs/zomes/coordinator/blog/src/comment.rs`

```rust
#[hdk_extern]
pub fn get_comments_for_post(post_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(post_hash, LinkTypes::PostToComments)?.build())
}

```

</details>

Don't forget to restart your app after changing zome code.

#### 4. Make comments updatable

Similarly to with updating posts, updating a comment will add a new action and its corresponding entry to the DHT. However once again, these changes will not be reflected in the agent's window UI.

We could resolve this issue the same way we did with the posts; by creating a link from the create action to each new update action, however, there is another solution that we can use instead.

Navigate to `CommentDetail.svelte` and modify the `fetchComment` function to call `get_latest_comment` instead of `get_original_comment`

Navigate to `comment.rs` and paste in the following zome function

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

The upside of using this method for update retrieval is that it requires less storage to be used on the DHT, as no extra links need to be created. However the recursive nature of this method can make it take longer to retrieve information if there is a long chain of updates spread around the DHT. 

Ultimately the choice of how to retrieve updated entries is up to you and this challenge shows you two strategies.

#### 5. Restart the app, create a comment, and update it.

#### 8. Make comments deletable

To delete a comment, paste the following zome function inside `dnas/blog/zomes/coordinator/blog/src/comment.rs`

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

How is this zome function different to the `delete_post`?

Well done! You made it to the end.
