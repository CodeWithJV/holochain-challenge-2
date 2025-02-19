<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Comment } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

export let currentRecord!: Record;

let currentComment: Comment = decode((currentRecord.entry as any).Present.entry) as Comment;
let content: string | undefined = currentComment.content;

$: content;
$: isCommentValid = true && content !== "";

onMount(async () => {
  if (!currentRecord) {
    throw new Error(`The currentRecord input is required for the EditComment element`);
  }
  client = await appClientContext.getClient();
});

async function updateComment() {
  const comment: Comment = {
    content: content!,
    post_hash: currentComment.post_hash,
    author: currentComment.author,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: "blog",
      zome_name: "blog",
      fn_name: "update_comment",
      payload: {
        previous_comment_hash: currentRecord.signed_action.hashed.hash,
        updated_comment: comment,
      },
    });

    dispatch("comment-updated", { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<section>
  <div>
    <label for="Content">Content</label>
    <input name="Content" bind:value={content} required />
  </div>

  <div>
    <button on:click={() => dispatch("edit-canceled")}>Cancel</button>
    <button disabled={!isCommentValid} on:click={() => updateComment()}>
      Edit Comment
    </button>
  </div>
</section>
