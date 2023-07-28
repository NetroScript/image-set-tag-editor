import { writable, derived, get } from 'svelte/store';

export const working_folder = writable('');
export const file_server_port = writable(0);
export const available_files = writable<string[]>([]);

const VALID_IMAGE_FORMATS = ['.jpg', '.png', '.gif', '.bmp', '.webp', '.jpeg', '.avif']

// Have a derived store, which combines all available files into an image file, and a corresponding caption file (the first non image file with the same name)
export const available_images = writable<{image: string, caption: string, caption_file: string, viewed: boolean}[]>([]);
export const is_loading_image_data = writable(false);

const update_available_files = async () => {
    const image_map = new Map<string, {image: string, caption: string, caption_file: string, viewed: boolean}>();


    const unprocessed_files = [];
    
    for (const file of get(available_files)) {
        // Get the file without the extension
        const [file_name, file_extension] = file.split('.');
        if (VALID_IMAGE_FORMATS.includes(`.${file_extension.toLocaleLowerCase()}`)) {
            image_map.set(file_name, {
                image: file,
                caption: '',
                caption_file: '',
                viewed: false,
            });
        } else {
            unprocessed_files.push(file);
        }
    };

    for (const file of unprocessed_files) {
        const [file_name, file_extension] = file.split('.');
        const image = image_map.get(file_name);
        if (image && image.caption === '') {
            image.caption_file = file;

            // Fetch the text caption from our file server
            image.caption = await fetch(`http://localhost:${get(file_server_port)}/${file}`).then((response) => {
                return response.text();
            }).catch((error) => {
                console.error(error);
            }) || '';

        }
    }

    return Array.from(image_map.values());
}


// But we need to manually, asynchronously update the available images by subscribing to the available files store
available_files.subscribe(async (files) => {
    is_loading_image_data.set(true);
    available_images.set(await update_available_files());
    is_loading_image_data.set(false);
});


