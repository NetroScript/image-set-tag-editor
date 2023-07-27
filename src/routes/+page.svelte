<script lang="ts">

	import {selectFolder } from '../lib/bindings';

	let working_directory: string | undefined = undefined;

	const select_working_directory = async () => {
		working_directory = await selectFolder();
		console.log(working_directory);
	};
	

</script>

{#if !working_directory}
<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-5">
		<h1 class="h1">Select the directory you want to work in:</h1>
		<p>Start by exploring:</p>
		<ul>
			<li><code class="code">/src/routes/+layout.svelte</code> - barebones layout, the CSS import order is critical!</li>
			<li><code class="code">/src/app.postcss</code> - minimal css to make the page full screen, may not be relevant for your project</li>
			<li><code class="code">/src/routes/+page.svelte</code> - this page, you can replace the contents</li>
		</ul>
	</div>
</div>

{:else}

{/if}

