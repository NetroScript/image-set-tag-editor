# image-set-tag-editor

This is a UI application which allows you to quickly edit a caption file for an image. You can open a folder, get a preview of the image and then can quickly skip through the images and edit the captions for each image. 
Caption files can have any extension, but must have the same name as the image file.
If you have a folder with different types of captions (`.txt` and `.caption` for example), the extension which occurs most often will be chosen when saving the changes.

![Application Preview](https://github.com/NetroScript/image-set-tag-editor/assets/18115780/fb85cefb-7298-4807-8d1c-9e09ce22c9d4)


This application was mainly made to edit captions for images which are used in training Stable Diffusion fine-tunes.
If you need cropping, I suggest my other application [SpeedCrop](https://github.com/NetroScript/SpeedCrop).
Also it is not possible to bulk edit captions with this application. This is mostly for a final manual review step to check individual captions together with the image in a preview, and change them if necessary.
There is a rough estimate of token count being shown using CLIP, however as each diffusion model has its own tokenizer, this is not 100% accurate. It is just a rough estimate to see if the caption is too long or too short. ([Tokenizer used](https://github.com/instant-labs/instant-clip-tokenizer))


- [image-set-tag-editor](#image-set-tag-editor)
  - [Installing](#installing)
  - [Usage](#usage)
    - [1. Starting Up](#1-starting-up)
    - [2. Navigating Through Images](#2-navigating-through-images)
      - [Using Keyboard:](#using-keyboard)
      - [Using Mouse:](#using-mouse)
    - [3. Editing Image Captions](#3-editing-image-captions)
    - [4. Saving Captions](#4-saving-captions)
  - [Advanced Usage](#advanced-usage)
    - [Filtering Tags](#filtering-tags)
    - [Custom CSS](#custom-css)
  - [Developing](#developing)
    - [Prerequisites](#prerequisites)
    - [Setup](#setup)
    - [Running](#running)
    - [Building](#building)


## Installing

As an end-user, you can download the latest version from the [releases page](https://github.com/NetroScript/image-set-tag-editor/releases).

If you want to build the application yourself, follow the instructions in the [development section](#developing).

## Usage

### 1. Starting Up

When you first open the application:

1. Click on the **Select Folder** button to pick a directory containing your images. The native OS folder dialog will pop up, allowing you to navigate to the desired location on your system.
2. After selecting the folder, the application will load the images and display the first one in the main viewing area.

### 2. Navigating Through Images

There are several ways you can move between images:

#### Using Keyboard:

- **Left and Right Arrow Keys**: If the caption editing field is **not** in focus, simply press the left or right arrow keys on your keyboard to navigate to the previous or next image, respectively.
  
- **Numpad Keys**: If the caption editing field **is** in focus, you can alternatively use the `Numpad 4` key to move to the previous image and `Numpad 6` key to move to the next one.

#### Using Mouse:

- **Image Preview Bar**: The bottom bar showcases smaller previews of all your loaded images. To navigate to a specific image, simply double-click on its preview. Images that have been viewed at least once will have a green bar at the bottom of their preview. Note: This green bar only indicates that the image was viewed, not that its caption was saved.

- **Navigation Arrows**: At the top of the application, you'll find two arrow icons. Click on the left arrow to go to the previous image, and the right arrow to move to the next image.

### 3. Editing Image Captions

1. To edit an image caption, click on the caption field below the displayed image to bring it into focus.
2. Once selected, you can type in or modify the existing caption as needed.

On the left side there will be an overview with all tags in the current image data set. By checking or removing the checkmark, the corresponding tags will either be added or removed from the current image. Tags are prepended, however it does not matter where they are for removal.

The count of each tag only updates when you change to another image.

### 4. Saving Captions

To save your edited captions, click on the **Save** button located in the bottom left corner of the application. This will overwrite (or create) all captions with the current data. Ensure you've made all necessary changes before saving.

You can also see a video of the application in action here:

https://github.com/NetroScript/image-set-tag-editor/assets/18115780/e4a1e39d-183c-429e-a7f2-d6d0478ed139

## Advanced Usage

### Filtering Tags

The left side with tags allows quickly adding and removing tags from the currently open image.
However, only the 50 most frequent tags are shown there. For this reason, it is to show a subset of tags, by using certain filtering options.

Writing any text will filter the tags using just a simple string search (case sensitive).
Entering `red` for example will only show all tags which contain the string `red` in them.
Should you want multiple tags, you can separate them with `|` (pipe).
So if you want to see for example all tags which contain tree, grass, building or sky, you can enter `tree|grass|building|sky`.

It is also possible to exclude tags from the results by prepending a `-` (minus) to the tag.
If you want to see all tags which contain tree, grass building or sky, but no tags which contain car, you can enter `tree|grass|building|sky|-car`.

The *"dataset"* I use for the screenshots for example is just the generated descriptions from Stable Diffusion images. So to not show any generation settings in the text, I can filter using the following string:

`-Seed|-Negative|-CFG|-Size|-Sampler|-Model|-ENSD`

If you need even more complex filtering, it is possible to use regex. To do so, you can either prepend `^` (caret) to the search string, or append `$` (dollar sign) to it. This will then be parsed as regex instead of a simple string search.

So to exclude all tags which start with a `(` for example, you can enter `-^\(`. To take this apart:

* `-` will exclude the results
* `^` will tell the application to parse the search string as regex
* `\(` will match a literal `(`


### Custom CSS

It is possible to insert custom CSS into the application. This can be done by pressing `Alt + C`. This will open a dialog where you can enter your custom CSS. This CSS will be applied to the whole application, so you can change the background color for example. However the most useful use is to highlight certain tags which are used, this works, as each tag on the left side has a `data-tag` attribute, which contains the tag itself. So you can use this to highlight certain tags.

An example CSS would be the following:

```css
/* If the tag contains the keyword red, the tag font color will be red */ 
.tag-display[data-tag*='red'] {
		color: rgb(255, 50, 5);
}

/* Every tag which has a ( has a light white background. */
.tag-display[data-tag*='('] {
		background: rgba(255, 255, 255, 0.1);
}

/* Every tag starting with ( has a strong white background. */
.tag-display[data-tag^='('] {
		background: rgba(255, 255, 255, 0.3);
}
```

This would look like the following:

Pressing `Alt + C`:

![image-set-tag-editor_ibLtOf19WM](https://github.com/NetroScript/image-set-tag-editor/assets/18115780/afc7ce21-1d64-4b0f-a3a9-15b0b22a8022)

After entering the CSS shown above, the left side will look like this:

<p align="center">
<img src="https://github.com/NetroScript/image-set-tag-editor/assets/18115780/4cc17084-d57c-48dd-8c43-63e6a7a5964b">
</p>

**Note:** The custom CSS is not saved, so if you close the application, you will have to re-enter it.

## Developing

This app uses [SvelteKit](https://kit.svelte.dev/) for the UI and [tauri](https://tauri.app/) for the native wrapper and file system access.

### Prerequisites

- [Node.js](https://nodejs.org/en/)
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install)

### Setup

1. Clone the repository and navigate to the project directory.
2. Install the dependencies with `pnpm install`.

### Running

To start a development server:

```bash
pnpm tauri dev
```

### Building

To create a production version of your app:

```bash
pnpm tauri build
```
