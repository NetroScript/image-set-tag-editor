import { writable, derived, get } from 'svelte/store';

export const working_folder = writable('');
export const file_server_port = writable(0);
export const available_files = writable<string[]>([]);
export const single_tag_delimiter = writable(',');

const VALID_IMAGE_FORMATS = ['.jpg', '.png', '.gif', '.bmp', '.webp', '.jpeg', '.avif'];

// Have a derived store, which combines all available files into an image file, and a corresponding caption file (the first non image file with the same name)
export const available_images = writable<
	{ image: string; caption: string; caption_file: string; viewed: boolean }[]
>([]);
export const is_loading_image_data = writable(false);
export const active_image = writable<
	{ image: string; caption: string; caption_file: string; viewed: boolean } | undefined
>(undefined);

export const tagCounts = derived(
	[available_images, single_tag_delimiter],
	([$available_images, $single_tag_delimiter]) => {
		console.log('counting tags');
		const data: Record<string, number> = {};
		$available_images.forEach((image) => {
			const tags = (image.caption.split($single_tag_delimiter) || []).map(
				(tag) => tag.trim()
			);

			tags.forEach((tag) => {
				if (data[tag]) {
					data[tag] += 1;
				} else {
					data[tag] = 1;
				}
			});
		});
		return data;
	}
);

export const sortedTags = derived(tagCounts, ($tagCounts) => {
	console.log('sorting tags');
	return Object.keys($tagCounts).sort((a, b) => $tagCounts[b] - $tagCounts[a]);
});
export const current_caption = writable('');

const update_available_files = async () => {
	const image_map = new Map<
		string,
		{ image: string; caption: string; caption_file: string; viewed: boolean }
	>();

	const unprocessed_files = [];

	for (const file of get(available_files)) {
		// Get the file without the extension
		const [file_name, file_extension] = file.split('.');
		if (file_extension && VALID_IMAGE_FORMATS.includes(`.${file_extension.toLowerCase()}`)) {
			image_map.set(file_name, {
				image: file,
				caption: '',
				caption_file: '',
				viewed: false
			});
		} else {
			unprocessed_files.push(file);
		}
	}

	const caption_counts: Map<string, number> = new Map();

	for (const file of unprocessed_files) {
		const [file_name, file_extension] = file.split('.');

		// If a file extension exists
		if (file_extension !== undefined) {
			// Count how often we see the different caption extensions
			if (caption_counts.has(file_extension.toLowerCase())) {
				caption_counts.set(
					file_extension.toLocaleLowerCase(),
					caption_counts.get(file_extension.toLowerCase())! + 1
				);
			} else {
				caption_counts.set(file_extension.toLowerCase(), 1);
			}
		}

		const image = image_map.get(file_name);
		if (image && image.caption === '') {
			image.caption_file = file;

			// Fetch the text caption from our file server, disable caching
			image.caption =
				(await fetch(`http://localhost:${get(file_server_port)}/${file}`, { cache: 'no-store' })
					.then((response) => {
						return response.text();
					})
					.catch((error) => {
						console.error(error);
					})) || '';
		}
	}

	let final_caption_extension = 'txt';

	if (caption_counts.size > 0) {
		// Find the most common caption extension
		const most_common_caption_extensions = [...caption_counts.entries()].reduce((a, e) =>
			e[1] > a[1] ? e : a
		);
		if (most_common_caption_extensions.length > 0) {
			[final_caption_extension] = most_common_caption_extensions;
		}
	}

	// Iterate all images, and set the caption file if it is not already set (using the same name as the image, but with the most common caption extension)
	for (const image of image_map.values()) {
		if (image.caption_file === '') {
			image.caption_file = `${image.image.split('.')[0]}.${final_caption_extension}`;
		}
	}

	return Array.from(image_map.values());
};

// But we need to manually, asynchronously update the available images by subscribing to the available files store
available_files.subscribe(async (files) => {
	is_loading_image_data.set(true);
	available_images.set(await update_available_files());
	is_loading_image_data.set(false);
});


// Have a store for custom CSS classes if wanted 
export const custom_css_classes = writable('');