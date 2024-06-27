<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from 'svelte'
  import type { AppClient, Record, AgentPubKey } from '@holochain/client'
  import { clientContext } from '../../contexts'
  import type { Post } from './types'
  import '@material/mwc-button'
  import '@material/mwc-snackbar'
  import type { Snackbar } from '@material/mwc-snackbar'
  import '@material/mwc-textfield'
  import '@material/mwc-textarea'

  let client: AppClient = (getContext(clientContext) as any).getClient()

  const dispatch = createEventDispatcher()

  export let author!: AgentPubKey

  let name: string = ''
  let content: string = ''

  let errorSnackbar: Snackbar

  $: name, content, author
  $: isPostValid = true && name !== '' && content !== ''

  onMount(() => {
    if (author === undefined) {
      throw new Error(`The author input is required for the CreatePost element`)
    }
  })

  async function createPost() {
    const postEntry: Post = {
      name: name!,
      content: content!,
      author: author!,
    }

    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: 'blog',
        zome_name: 'blog',
        fn_name: 'create_post',
        payload: postEntry,
      })
      dispatch('post-created', { postHash: record.signed_action.hashed.hash })
    } catch (e) {
      errorSnackbar.labelText = `Error creating the post: ${e.data}`
      errorSnackbar.show()
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading> </mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Post</span>

  <div style="margin-bottom: 16px">
    <mwc-textfield
      outlined
      label="Name"
      value={name}
      on:input={(e) => {
        name = e.target.value
      }}
      required
    ></mwc-textfield>
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textarea
      outlined
      label="Content"
      value={content}
      on:input={(e) => {
        content = e.target.value
      }}
      required
    ></mwc-textarea>
  </div>

  <!-- svelte-ignore a11y-click-events-have-key-events -->

  <mwc-button
    raised
    label="Create Post"
    disabled={!isPostValid}
    on:click={() => createPost()}
  ></mwc-button>
</div>
