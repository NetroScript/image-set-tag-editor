<script lang="ts">
	// The ordering of these imports is critical to your app working properly
	import '@skeletonlabs/skeleton/themes/theme-crimson.css';
	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';

	import { AppShell, Toast, type ToastSettings } from '@skeletonlabs/skeleton';
	import {
		available_files,
		available_images,
		file_server_port,
		is_loading_image_data,
		working_folder
	} from '$lib/stores';
	import { getAvailableFiles, getServedDir, selectFolder, saveCaptions } from '$lib/bindings';
	import { toastStore } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';

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
</script>

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
