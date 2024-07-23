<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from 'svelte'
  import type {
    AppClient,
    Record,
    EntryHash,
    AgentPubKey,
    DnaHash,
    ActionHash,
  } from '@holochain/client'
  import { decode } from '@msgpack/msgpack'
  import { clientContext } from '../../contexts'
  import type { Post } from './types'
  import '@material/mwc-button'
  import '@material/mwc-snackbar'
  import type { Snackbar } from '@material/mwc-snackbar'
  import '@material/mwc-textarea'
  import '@material/mwc-textfield'

  let client: AppClient = (getContext(clientContext) as any).getClient()

  const dispatch = createEventDispatcher()

  export let originalPostHash!: ActionHash

  export let currentRecord!: Record
  let currentPost: Post = decode(
    (currentRecord.entry as any).Present.entry
  ) as Post

  let name: string | undefined = currentPost.name
  let content: string | undefined = currentPost.content

  let errorSnackbar: Snackbar

  $: name, content
  $: isPostValid = true && name !== '' && content !== ''

  onMount(() => {
    if (currentRecord === undefined) {
      throw new Error(
        `The currentRecord input is required for the EditPost element`
      )
    }
    if (originalPostHash === undefined) {
      throw new Error(
        `The originalPostHash input is required for the EditPost element`
      )
    }
  })

  async function updatePost() {
    const post: Post = {
      name: name!,
      content: content!,
      author: currentPost.author,
    }

    try {
      const updateRecord: Record = await client.callZome({
        cap_secret: null,
        role_name: 'blog',
        zome_name: 'blog',
        fn_name: 'update_post',
        payload: {
          original_post_hash: originalPostHash,
          previous_post_hash: currentRecord.signed_action.hashed.hash,
          updated_post: post,
        },
      })

      dispatch('post-updated', {
        actionHash: updateRecord.signed_action.hashed.hash,
      })
    } catch (e) {
      errorSnackbar.labelText = `Error updating the post: ${e}`
      errorSnackbar.show()
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>
<div
  style="display: flex; flex-direction: column; gap: 16px; box-sizing: border-box; margin: 16px 0;"
>
  <span style="font-size: 18px; "><strong>Edit Post</strong></span>

  <mwc-textfield
    outlined
    label="Name"
    value={name}
    on:input={(e) => {
      name = e.target.value
    }}
    required
  ></mwc-textfield>

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

  <div style="display: flex; flex-direction: row">
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch('edit-canceled')}
      style="flex: 1; margin-right: 16px"
    ></mwc-button>
    <mwc-button
      raised
      label="Save"
      disabled={!isPostValid}
      on:click={() => updatePost()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
