<script lang="ts">
  import IbanValidator from './lib/IbanValidator.svelte'
</script>

<main class="container">
    <IbanValidator />
</main>

<style>
  .container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}
</style>