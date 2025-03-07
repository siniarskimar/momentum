<script lang="ts">
  import { onMount } from "svelte";
  import Modals, { pop as modalPop } from "./Modals.svelte";

  let dialog: HTMLDialogElement | null = null;

  function onclose() {
    console.log(dialog);
    modalPop();
  }
</script>

<dialog open bind:this={dialog} {onclose}>
  <h1>Hello</h1>
  <button
    onclick={() => {
      dialog?.close();
    }}
  >
    Close
  </button>
</dialog>

<style>
  dialog {
    display: flex;
    flex-direction: column;
  }
</style>
