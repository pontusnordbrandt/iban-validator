<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {
    CheckCircleOutlined,
    CloseCircleFilled
  } from 'svelte-ant-design-icons';
    
  interface IbanResult { 
    iban: string;
    isValidCountry: boolean;
    isCorrectLength: boolean;
    isDivisibleBy97: boolean;
  }

  let ibanInput = "DE89370400440532013000,DE89370400440532013000";
  let ibanResult: IbanResult[] = []

  async function validateIban() {
    ibanResult = await invoke("validate_iban", { iban_numbers: ibanInput.split(',').length === 1 ? [ibanInput] : ibanInput.split(',') });
  } 


  $: ibanInput, ibanInput && validateIban();
</script>

<main>
  <input id="greet-input" placeholder="Enter one iban, or multiple separated by comma..." bind:value={ibanInput} />
  {#each ibanResult as { iban, isValidCountry, isCorrectLength, isDivisibleBy97 }}
  <table class="table">
    <thead>
      <tr>
        <th class="title">{iban}</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <p>Is Country Code Valid?</p>
        {#if isValidCountry}
          <CheckCircleOutlined color="#22c55e"/>
        {:else}
          <CloseCircleFilled color="#ef4444"/>
        {/if}
      </tr>
      <tr>
        <p>Is Correct Length?</p>
        {#if isCorrectLength}
          <CheckCircleOutlined color="#22c55e"/>
        {:else}
          <CloseCircleFilled color="#ef4444"/>
        {/if}
      </tr>
      <tr>
        <p>Divisible by 97?</p>
        {#if isDivisibleBy97}
          <CheckCircleOutlined color="#22c55e"/>
        {:else}
          <CloseCircleFilled color="#ef4444"/>
        {/if}
      </tr>

    </tbody>
  </table>
  {/each}
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .table {
    padding: 0.6em 1.2em;
    width: 80%;
    background-color: #ffffff;
    color: #0f0f0f98;
    margin-top: 10px;
    border: 1px 91919198 solid;
    border-radius: 8px;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }


  .title {
      color: black;
    }

  tr {
    line-height: 3px;
    padding: 0px;
    margin: 0px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
</style>