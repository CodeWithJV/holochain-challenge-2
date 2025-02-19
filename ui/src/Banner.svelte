<script lang="ts">
  import logo from './assets/holochainLogo.svg'

  export let challengeNumber: number = 0
  export let challengeName: string = ''
</script>

<svelte:head>
  <link
    href="https://fonts.googleapis.com/css?family=Gelasio"
    rel="stylesheet"
  />
</svelte:head>

<div
  style="display: flex; flex-direction: column; height: 100vh; width: 100vw; padding: 16px; box-sizing: border-box;"
>
  <header style="height: 48px; display: flex; justify-content: space-between;">
    <h1 style="font: 1.4em lato; margin-top: 5px; font-weight: 300;">
      Challenge {challengeNumber}
      <span style="font-weight: 600;">{challengeName}</span>
    </h1>
    <img src={logo} alt="logo" style="height: 100%;" />
  </header>
  <main style="height: 100%; width: 100%; text-align: center;">
    <slot />
  </main>
</div>
