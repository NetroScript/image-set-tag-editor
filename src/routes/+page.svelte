<script lang="ts">

	import { working_folder, available_images, is_loading_image_data, file_server_port } from '$lib/stores';
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { dragscroll } from '@svelte-put/dragscroll';
	import type { Action } from 'svelte/action';
	import { get } from 'svelte/store';

	$: current_file = 0;

	$: current_image_url = `http://localhost:${$file_server_port}/${encodeURI($available_images[current_file].image)}`
	$: background_css_string = `background-image: url('${current_image_url}')`;
	

	const horizontalWheelScroll: Action = (node: HTMLElement) => {
		node.style.scrollBehavior = "smooth";

		node.addEventListener("wheel", (evt) => {
			evt.preventDefault();
			node.scrollLeft += evt.deltaY;
		});

		return {
			destroy() {
				node.removeEventListener("wheel", (evt) => {
					evt.preventDefault();
					node.scrollLeft += evt.deltaY;
				});
			},
		};
		
	};

	const nextImage = () => {
		if (current_file < get(available_images).length - 1) {
			current_file = current_file + 1;
			$available_images[current_file].viewed = true;
			scrollToCurrentImage();
		}
	};

	const previousImage = () => {
		if (current_file > 0) {
			current_file = current_file - 1;
			$available_images[current_file].viewed = true;
			scrollToCurrentImage();
		}
	};

	const keydownHandler = (evt: KeyboardEvent) => {
		if (evt.key == "ArrowRight") {
			nextImage();
		} else if (evt.key == "ArrowLeft") {
			previousImage();
		}
	};

	let scrollContainer: HTMLElement;

	const scrollToCurrentImage = () => {
		const element = scrollContainer.children[current_file] as HTMLElement;
		scrollContainer.scrollTo({
			left: element.offsetLeft - 500,
			behavior: "smooth",
		});
	};

</script>


<svelte:window on:keydown={keydownHandler} />

{#if $working_folder.length == 0}

<div class="container h-full mx-auto flex justify-center items-center">
	
	<div class="space-y-5 p-6">
		<h1 class="h1 block flex-1">Select the directory you want to work in the sidebar.</h1>
	</div>
</div>

{:else if $is_loading_image_data}
<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-5 p-6 flex items-center flex-col">
		<ProgressRadial value={undefined} class="flex-1" />
		<h3 class="h3">Loading folder data.</h3>
	</div>
</div>


{:else}


<div class="h-full flex flex-col">

	<div class="flex-1">
		<div class="bg-contain w-full h-full bg-no-repeat bg-center" style={background_css_string}>

		</div>
	
	</div>


	<div class="px-2">
		<label class="label">
			<span>Caption</span>
			<textarea class="textarea" rows="2" placeholder="Write your caption for the image here." bind:value={$available_images[current_file].caption}/>
		</label>
	</div>

	<!-- Have a container at the bottom with all images selectable-->

	<div class="w-full overflow-x-scroll h-[180px] flex overflow-y-hidden space-x-1" bind:this={scrollContainer} use:dragscroll use:horizontalWheelScroll>
		{#each $available_images as image_data, index}

		<div class="min-w-[150px] h-full cursor-pointer" class:visited={image_data.viewed} class:hightlighted={index == current_file} on:dblclick={() => {
			current_file = index;
			$available_images[index].viewed = true;
		}}>
			<img class="object-cover w-full h-full" src="http://localhost:{$file_server_port}/{image_data.image}" alt={image_data.caption}>
		</div>

		{/each}
	</div>
</div>


{/if}

<style>
	.hightlighted {
		border: 2px solid #eeeeee;
		filter: brightness(1.2);
	}

	.visited {
		border-bottom: 3px solid rgb(62, 153, 62);
	}
</style>