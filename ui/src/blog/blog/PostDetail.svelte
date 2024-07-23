<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte'
  import '@material/mwc-circular-progress'
  import { decode } from '@msgpack/msgpack'
  import type {
    Record,
    ActionHash,
    AppClient,
    EntryHash,
    AgentPubKey,
    DnaHash,
  } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { Post } from './types'
  import '@material/mwc-circular-progress'
  import type { Snackbar } from '@material/mwc-snackbar'
  import '@material/mwc-snackbar'
  import '@material/mwc-icon-button'
  import EditPost from './EditPost.svelte'
  import CreateComment from './CreateComment.svelte'
  import CommentsForPost from './CommentsForPost.svelte'

  const dispatch = createEventDispatcher()

  export let postHash: ActionHash

  let client: AppClient = (getContext(clientContext) as any).getClient()

  let loading: boolean
  let creatingComment: boolean
  let error: any = undefined

  let record: Record | undefined
  let post: Post | undefined

  let editing = false

  let errorSnackbar: Snackbar

  $: editing, error, loading, record, post, creatingComment

  onMount(async () => {
    if (postHash === undefined) {
      throw new Error(
        `The postHash input is required for the PostDetail element`
      )
    }
    await fetchPost()
  })

  async function fetchPost() {
    loading = true

    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'blog',
        zome_name: 'blog',
        fn_name: 'get_latest_post',
        payload: postHash,
      })
      if (record) {
        post = decode((record.entry as any).Present.entry) as Post
      }
    } catch (e) {
      error = e
    }

    loading = false
  }

  async function deletePost() {
    try {
      await client.callZome({
        cap_secret: null,
        role_name: 'blog',
        zome_name: 'blog',
        fn_name: 'delete_post',
        payload: postHash,
      })
      dispatch('post-deleted', { postHash: postHash })
    } catch (e: any) {
      errorSnackbar.labelText = `Error deleting the post: ${e}`
      errorSnackbar.show()
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>

{#if loading}
  <mwc-circular-progress indeterminate></mwc-circular-progress>
{:else if error}
  <span>Error fetching the post: {error}</span>
{:else if editing}
  <EditPost
    originalPostHash={postHash}
    currentRecord={record}
    on:post-updated={async () => {
      editing = false
      await fetchPost()
    }}
    on:edit-canceled={() => {
      editing = false
    }}
  ></EditPost>
{:else}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div style="display: flex; flex-direction: column; width: 100%;">
    <div style="display: flex; flex-direction: row; width: 100%;">
      <div
        style="display: flex; flex-direction: row; flex: 1; align-items: center;"
      >
        <span style="margin-right: 4px"><strong>Name:</strong></span>
        <span style="white-space: pre-line">{post?.name}</span>
      </div>
      <mwc-icon-button
        style="margin-left: 8px"
        icon="edit"
        on:click={() => {
          editing = true
        }}
      ></mwc-icon-button>
      <mwc-icon-button
        style="margin-left: 8px"
        icon="delete"
        on:click={() => deletePost()}
      ></mwc-icon-button>
    </div>

    <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      <span style="white-space: pre-line; text-align: start;"
        >{post?.content}</span
      >
    </div>

    <!-- Uncomment this section -->

    {#if creatingComment}
      <CreateComment
        on:canceled={() => {
          creatingComment = false
        }}
        on:comment-created={() => {
          creatingComment = false
        }}
        {postHash}
        author={client.myPubKey}
      />
    {:else}
      <CommentsForPost {postHash} />
      <mwc-button
        dense
        outlined
        label="Create Comment"
        on:click={() => {
          creatingComment = true
        }}
      ></mwc-button>
    {/if}
  </div>
  <hr style="width: 70%;" />
{/if}
