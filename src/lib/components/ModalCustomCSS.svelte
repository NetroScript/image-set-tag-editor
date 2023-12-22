<script lang="ts">
	import type { SvelteComponent } from 'svelte';

	// Stores
	import { getModalStore } from '@skeletonlabs/skeleton';
	import { custom_css_classes } from '$lib/stores';

	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;

	const modalStore = getModalStore();

	// We've created a custom submit function to pass the response and close the modal.
	function onFormSubmit(): void {
		if ($modalStore[0].response) $modalStore[0].response($custom_css_classes);
		modalStore.close();
	}
</script>

<!-- @component This example creates a simple form modal. -->

{#if $modalStore[0]}
	<div class="card p-4 container shadow-xl space-y-4">
		<header class="text-2xl font-bold">Configure custom CSS classes.</header>
		<!-- Enable for debugging: -->
		<form class="modal-form p-2 space-y-4 rounded-container-token">
			<textarea
				class="textarea"
				rows="16"
				placeholder="Enter your custom css"
				bind:value={$custom_css_classes}
				on:keydown={(event) => {
					event.stopPropagation();
				}}
			/>
		</form>
		<!-- prettier-ignore -->
		<footer class="modal-footer {parent.regionFooter}">
        <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{parent.buttonTextCancel}</button>
        <button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Confirm</button>
    </footer>
	</div>
{/if}
