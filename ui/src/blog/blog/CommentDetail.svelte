<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import EditComment from "./EditComment.svelte";
import type { Comment } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

let loading: boolean = false;
let editing = false;
let error: HolochainError | undefined;
let record: Record | undefined;
let comment: Comment | undefined;

export let commentHash: ActionHash;

$: editing, error, loading, record, comment;

onMount(async () => {
  if (commentHash === undefined) {
    throw new Error(`The commentHash input is required for the CommentDetail element`);
  }
  client = await appClientContext.getClient();
  await fetchComment();
});

async function fetchComment() {
  loading = true;
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: "blog",
      zome_name: "blog",
      fn_name: "get_latest_comment",
      payload: commentHash,
    });
    if (record) {
      comment = decode((record.entry as any).Present.entry) as Comment;
    }
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
}

async function deleteComment() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: "blog",
      zome_name: "blog",
      fn_name: "delete_comment",
      payload: commentHash,
    });
    dispatch("comment-deleted", { commentHash: commentHash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">Error fetching the comment: {error.message}</div>
{:else if editing}
  <EditComment
    currentRecord={record}
    on:comment-updated={async () => {
      editing = false;
      await fetchComment();
    }}
    on:edit-canceled={() => {
      editing = false;
    }}
  />
{:else}
  <div class="comment">
    <div class="comment-content">{comment?.content}</div>

    <div class="comment-actions">
      <button
        class="edit-button"
        on:click={() => {
          editing = true;
        }}
      >Edit</button>
      <button class="delete-button" on:click={() => deleteComment()}>Delete</button>
    </div>
  </div>
{/if}
