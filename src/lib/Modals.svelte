<script module lang="ts">
  import type { Component as Comp } from "svelte";

  interface ModalCtx {
    Component: Comp;
    args: Object;
  }

  let stack = $state<ModalCtx[]>([]);

  export function push(Component: Comp, args: Object) {
    stack.push({ Component, args });
  }

  export function pop() {
    if (stack.length == 0) throw new Error("Modal stack is empty");
    stack.pop();
  }
</script>

{#if stack.length != 0}
  <div class="modal-container">
    {#each stack as { Component, args }}
      <Component {...args}></Component>
    {/each}
  </div>
{/if}

<style>
  .modal-container {
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;

    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 99;

    background-color: rgba(0, 0, 0, 0.2);
  }

  :global(dialog) {
    border-radius: 8px;
    border: none;
    box-shadow: 0 0 3px 1px rgba(0, 0, 0, 0.5);
  }
</style>
