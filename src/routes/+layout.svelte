<script lang="ts">
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';

	import { Modal, Toast, getModalStore, getToastStore } from '@skeletonlabs/skeleton';
	import type { ModalComponent, ToastSettings, ToastStore } from '@skeletonlabs/skeleton';

	import { AppShell } from '@skeletonlabs/skeleton';
	import {
		available_files,
		available_images,
		custom_css_classes,
		file_server_port,
		is_loading_image_data,
		working_folder
	} from '$lib/stores';
	import { getAvailableFiles, getServedDir, selectFolder, saveCaptions } from '$lib/bindings';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import TagHelper from '$lib/components/TagHelper.svelte';
	import { initializeStores } from '@skeletonlabs/skeleton';
	import ModalCustomCss from '$lib/components/ModalCustomCSS.svelte';

	initializeStores();

	const toastStore = getToastStore();

	async function select_new_folder() {
		try {
			const folder = await selectFolder();
			$working_folder = folder;

			$is_loading_image_data = true;

			// Fetch all the files in the folder
			const files = await getAvailableFiles();

			// Set the files in the store
			$available_files = files;
		} catch {
			const t: ToastSettings = {
				message: 'No folder has been selected'
			};
			toastStore.trigger(t);
		}
	}

	let file_server_running = false;

	// On mount fetch the port for the server

	onMount(async () => {
		const data = await getServedDir();
		file_server_port.set(data[1]);

		// In an interval check if the server is running
		const intervalTimer = setInterval(async () => {
			const url = `http://localhost:${$file_server_port}/checkAlive`;

			// Do any fetch request, if the server responds with a 404 then it is running
			fetch(url)
				.then((res) => {
					file_server_running = true;
				})
				.catch((error) => {
					file_server_running = false;
				});
		}, 1000);
	});

	async function save_all_captions() {
		const new_captions: [string, string][] = [];

		get(available_images).forEach(async (image) => {
			new_captions.push([image.caption_file, image.caption]);
		});

		try {
			await saveCaptions(new_captions);
			const t: ToastSettings = {
				message: 'Captions saved'
			};
			toastStore.trigger(t);
		} catch (e) {
			const t: ToastSettings = {
				message: 'Error saving captions (check the console for more info)'
			};
			toastStore.trigger(t);
			console.error(e);
		}
	}

	const modalRegistry: Record<string, ModalComponent> = {
		// Set a unique modal ID, then pass the component reference
		cssModal: { ref: ModalCustomCss }
		// ...
	};

	const modalStore = getModalStore();

	// Key even handler to open modal when pressing alt + c
	const keyHandler = (event: KeyboardEvent) => {
		if (event.altKey && event.key === 'c') {
			modalStore.trigger({
				type: 'component',
				component: 'cssModal'
			});
		}
	};
</script>

<svelte:head>
	<!-- Dynamically insert a custom css style tag -->
	<svelte:element this={'style'} type="text/css">{$custom_css_classes}</svelte:element>
</svelte:head>

<svelte:window on:keyup={keyHandler} />

<Modal components={modalRegistry} />
<AppShell>
	<!-- (header) -->
	<svelte:fragment slot="sidebarLeft">
		<div
			class="h-full bg-surface-50-900-token border-r border-surface-500/30 w-[300px] overflow-hidden p-2 pt-2 flex flex-col"
		>
			<!-- Show connection status -->
			<div class="flex items-center space-x-2 mb-3">
				{#if file_server_running}
					<div class="w-3 h-3 rounded-full bg-success-500" />
					<span class="text-success-500">Connected (Port: {$file_server_port})</span>
				{:else}
					<div class="w-3 h-3 rounded-full bg-error-500" />
					<span class="text-error-500">Not Connected to fileserver.</span>
				{/if}
			</div>

			<label class="label">
				<span>Choose the base directory</span>
				<input
					class="input variant-form-material"
					type="search"
					disabled
					placeholder="Filepath"
					title="Choose the base directory"
					bind:value={$working_folder}
				/>
			</label>
			<div class="flex p-4">
				<button
					class="variant-filled-secondary btn variant-filled mx-auto w-full"
					on:click={select_new_folder}>Change</button
				>
			</div>

			<div class="flex-1" />

			<TagHelper />

			<div class="flex-1" />

			<label class="label text-center">
				<span>Apply all caption changes.</span>
				<button
					class="variant-filled-primary btn variant-filled mx-auto w-full"
					disabled={$available_images.length == 0}
					on:click={save_all_captions}>Save</button
				>
			</label>
		</div>
	</svelte:fragment>
	<!--<svelte:fragment slot="sidebarRight">Sidebar Right</svelte:fragment>-->
	<!--<svelte:fragment slot="pageHeader">Page Header</svelte:fragment>-->
	<!-- Router Slot -->
	<slot />
	<!-- ---- / ---- -->
	<!--<svelte:fragment slot="pageFooter">Page Footer</svelte:fragment>-->
	<!-- (footer) -->
</AppShell>

<Toast />
