<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

    
interface IbanResult { 
  iban: string;
  isValidCountry: boolean;
}

  let ibanInput = "";
  let ibanResult: IbanResult;

const greet = async () => {
  // invoke greet function on backend side (src-tauri/src/main.rs)
  ibanResult = await invoke("validate_iban", { iban: ibanInput });
}

// Whenever iban variable is reassigned, greet function is called
$: ibanInput, ibanInput && greet();

</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a iban..." bind:value={ibanInput} />
  </div>
  <div>
    {#if ibanResult}
      <p>IBAN: {ibanResult.iban}</p>
      <p>Is Valid Country: {ibanResult.isValidCountry}</p>
    {/if}

  </div>
</div>