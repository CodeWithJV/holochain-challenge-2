<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Post } from "./types";

const dispatch = createEventDispatcher();
let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let name: string = "";
let content: string = "";

export let author!: AgentPubKey;

$: name, content, author;
$: isPostValid = true && name !== "" && content !== "";

onMount(async () => {
  if (author === undefined) {
    throw new Error(`The author input is required for the CreatePost element`);
  }
  client = await appClientContext.getClient();
});

async function createPost() {
  const postEntry: Post = {
    name: name!,
    content: content!,
    author: author!,
  };

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: "blog",
      zome_name: "blog",
      fn_name: "create_post",
      payload: postEntry,
    });
    dispatch("post-created", { postHash: record.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<div>
  <h3>Create Post</h3>

  <div>
    <label for="Name">Name</label>
    <textarea name="Name" bind:value={name} required />
  </div>
  <div>
    <label for="Content">Content</label>
    <textarea name="Content" bind:value={content} required />
  </div>

  <button disabled={!isPostValid} on:click={() => createPost()}>
    Create Post
  </button>
</div>
