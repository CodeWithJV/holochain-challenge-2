<script lang="ts">
  import Banner from './Banner.svelte'
  import { onMount, setContext } from 'svelte'
  import type { ActionHash, AppClient } from '@holochain/client'
  import { AppWebsocket } from '@holochain/client'
  import '@material/mwc-circular-progress'

  import { clientContext } from './contexts'
  import CreatePost from './blog/blog/CreatePost.svelte'
  import AllPosts from './blog/blog/AllPosts.svelte'

  let client: AppClient | undefined

  let loading = true

  onMount(async () => {
    // We pass an unused string as the url because it will dynamically be replaced in launcher environments
    client = await AppWebsocket.connect()

    loading = false
  })

  setContext(clientContext, {
    getClient: () => client,
  })
</script>

<Banner challengeNumber={2} challengeName="Links & Collections">
  <main>
    {#if loading}
      <div
        style="display: flex; flex: 1; align-items: center; justify-content: center"
      >
        <mwc-circular-progress indeterminate />
      </div>
    {:else}
      <div id="content" style="display: flex; flex-direction: column; flex: 1;">
        <!-- Add your CreatePost and AllPosts components here -->
      </div>
    {/if}
  </main>
</Banner>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
