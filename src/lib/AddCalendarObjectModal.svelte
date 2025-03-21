<script lang="ts">
  import type { ModalProps } from "$lib/Modals.svelte";

  interface Props extends ModalProps {}

  let { close }: Props = $props();

  let dialog: HTMLDialogElement | null = null;

  function onclose() {
    console.log(dialog);
    close();
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
