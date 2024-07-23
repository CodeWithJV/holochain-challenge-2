<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from 'svelte'
  import type {
    AppClient,
    Record,
    EntryHash,
    AgentPubKey,
    ActionHash,
    DnaHash,
  } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { Comment } from './types'
  import '@material/mwc-button'
  import '@material/mwc-snackbar'
  import type { Snackbar } from '@material/mwc-snackbar'

  import '@material/mwc-textarea'
  let client: AppClient = (getContext(clientContext) as any).getClient()

  const dispatch = createEventDispatcher()

  export let postHash!: ActionHash

  export let author!: AgentPubKey

  let content: string = ''

  let errorSnackbar: Snackbar

  $: content, postHash, author
  $: isCommentValid = true && content !== ''

  onMount(() => {
    if (postHash === undefined) {
      throw new Error(
        `The postHash input is required for the CreateComment element`
      )
    }
    if (author === undefined) {
      throw new Error(
        `The author input is required for the CreateComment element`
      )
    }
  })

  async function createComment() {
    const commentEntry: Comment = {
      content: content!,
      post_hash: postHash!,
      author: author!,
    }

    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: 'blog',
        zome_name: 'blog',
        fn_name: 'create_comment',
        payload: commentEntry,
      })
      dispatch('comment-created', {
        commentHash: record.signed_action.hashed.hash,
      })
    } catch (e) {
      errorSnackbar.labelText = `Error creating the comment: ${e}`
      errorSnackbar.show()
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>
<div
  style="display: flex; flex-direction: column; gap: 16px; margin-bottom: 16px;"
>
  <span style="font-size: 18px"><strong>Create Comment</strong></span>

  <mwc-textarea
    outlined
    label="Content"
    value={content}
    on:input={(e) => {
      content = e.target.value
    }}
    required
  ></mwc-textarea>

  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div style="display: flex; flex: row; gap: 8px;">
    <mwc-button
      style="flex: 1;"
      outlined
      label="Cancel"
      on:click={() => {
        dispatch('canceled')
      }}
    ></mwc-button>
    <mwc-button
      raised
      style="flex: 1;"
      label="Create Comment"
      disabled={!isCommentValid}
      on:click={() => createComment()}
    ></mwc-button>
  </div>
</div>
