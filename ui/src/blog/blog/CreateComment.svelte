<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Comment } from "./types";

const dispatch = createEventDispatcher();
let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let content: string = "";

export let postHash!: ActionHash;
export let author!: AgentPubKey;

$: content, postHash, author;
$: isCommentValid = true && content !== "";

onMount(async () => {
  if (postHash === undefined) {
    throw new Error(`The postHash input is required for the CreateComment element`);
  }
  if (author === undefined) {
    throw new Error(`The author input is required for the CreateComment element`);
  }
  client = await appClientContext.getClient();
});

async function createComment() {
  const commentEntry: Comment = {
    content: content!,
    post_hash: postHash!,
    author: author!,
  };

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: "blog",
      zome_name: "blog",
      fn_name: "create_comment",
      payload: commentEntry,
    });
    dispatch("comment-created", { commentHash: record.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<div>
  <h3>Create Comment</h3>

  <div>
    <label for="Content">Content</label>
    <input name="Content" bind:value={content} required />
  </div>

  <button disabled={!isCommentValid} on:click={() => createComment()}>
    Create Comment
  </button>
</div>
