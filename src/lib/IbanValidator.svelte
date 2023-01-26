<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
    import { onMount } from "svelte";
  import {
    CheckCircleOutlined,
    CloseCircleFilled,
    FileAddOutlined
  } from 'svelte-ant-design-icons';
    
  interface IbanResult { 
    iban: string;
    isAlphanumeric: boolean;
    isValidCountry: boolean;
    isCorrectLength: boolean;
    isDivisibleBy97: boolean;
  }

  let ibanInput = "";
  let ibanResult: IbanResult[] = []
  let files: FileList;
  let errorMsg = "";
  
  let inputFieldRef: any;

  onMount(() => {
    inputFieldRef.focus()
  })

  async function readTextFile(e) {
    const [file] = files;
    if (!file) return; 
    errorMsg = "";
    if (file.type !== 'text/plain') {
      errorMsg = `${file.type} is not accepted as a file type.`;
      return;
    }
    validateIban(await file.text());
  }


  async function validateIban(ibanStr: string) {
    console.log('validateIBan')
    // Don't include numbers that have a length of less than 10, as it's probably a typo;
    const ibanNumbers = ibanStr
      .split(',')
      .filter(n => n.length > 10)
      .map(n => n.replace(/\s/g, ""));
    if (ibanNumbers.length) {
      ibanResult = await invoke("validate_iban", { ibanNumbers });
    } else {
      ibanResult = []
    }
  } 


  $: ibanInput, validateIban(ibanInput);
</script>

<main>
  <div class="sheet">
  <input bind:this={inputFieldRef} class="text-input" id="greet-input" placeholder="Enter one iban, or multiple separated by comma..." bind:value={ibanInput} />
  <p>Or Select a .txt file with comma separated values...</p>
  {#if errorMsg}
    <p class="error-text">{errorMsg}</p>
  {/if}
  
  
  <input
    class="center"
    accept="text/txt"
    bind:files
    on:change={readTextFile}
    name="iban-file"
    type="file"
  />
</div>
  {#each ibanResult as { iban, isAlphanumeric, isValidCountry, isCorrectLength, isDivisibleBy97 }}
  <table class="table">
    <thead>
      <tr>
        <th class="title">{iban}</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <p>Is IBAN alphanumeric?</p>
        {#if isAlphanumeric}
          <CheckCircleOutlined color="#22c55e"/>
        {:else}
          <CloseCircleFilled color="#ef4444"/>
        {/if}
      </tr>
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

  .center {
    margin: auto;
    display: flex;
    justify-content: center;
  }

  .title {
    color: black;
  }

  .error-text {
    font-size: small;
    color: red;
  }

  tr {
    line-height: 3px;
    padding: 0px;
    margin: 0px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .sheet {
    border-radius: 8px;
    border: 1px solid transparent;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    outline: none;
    text-align:center;
    width: 80%;
    padding: 10px 5px;
  }

  .text-input {
    border-radius: 8px;
    border: 1px solid transparent;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    outline: none;
    text-align:center;
    width: 80%;
    padding: 10px 5px;
  }
</style>