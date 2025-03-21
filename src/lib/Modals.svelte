<script module lang="ts">
  import type { Component } from "svelte";
  import { fade } from "svelte/transition";

  export interface ModalProps {
    close: () => void;
  }

  type ModalComponent = Component<ModalProps>;

  interface StackEntry {
    Modal: ModalComponent;
    args: Object;
    id: number;
    close: () => void;
  }

  const modalStack = $state<StackEntry[]>([]);
  let modalNextId = 0;

  function close(id: number) {
    const idx = modalStack.findIndex((v) => v.id === id);
    if (idx === -1) {
      console.warn("Tried to close a modal that doesn't exist");
      return;
    }
    modalStack.splice(idx, 1);
  }

  export function open(Modal: ModalComponent, args: Object) {
    const modalId = ++modalNextId;
    const innerClose = () => {
      close(modalId);
    };

    modalStack.push({ Modal, args, id: modalId, close: innerClose });
  }
</script>

<script lang="ts">
  function portal(node: HTMLElement) {
    document.body.appendChild(node);
    return {
      destroy() {
        node.remove();
      },
    };
  }
</script>

{#if modalStack.length > 0}
  <div use:portal class="modals-container">
    {#each modalStack as { Modal, args, close }}
      <div
        class="backdrop"
        transition:fade|global={{ duration: 200 }}
        role="presentation"
        onclick={() => close()}
      ></div>
      <Modal {...args} {close} />
    {/each}
  </div>
{/if}

<style>
  .modals-container {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;

    display: flex;
    align-items: center;
    justify-content: center;

    z-index: 1;
  }
  .backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;

    background-color: rgba(0, 0, 0, 0.2);
  }
</style>
