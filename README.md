# image-set-tag-editor

This is a UI application which allows you to quickly edit a caption file for a image. You can open a folder, get a preview of the image and then can quickly skip through the images and edit the captions for each image. 
Caption files can have any extension, but must have the same name as the image file.
If you have a folder with different types of captions (`.txt` and `.caption` for example), the extension which occurs most often will be chosen when saving the changes.

![Code_-_Insiders_iOD0uIeWcG](https://github.com/NetroScript/image-set-tag-editor/assets/18115780/575bb2c6-889c-4f65-83d1-34916e4a8f71)

This application was mainly made to edit captions for images which are used in training Stable Diffusion fine-tunes.
If you need cropping, I suggest my other application [SpeedCrop](https://github.com/NetroScript/SpeedCrop).
Also it is not possible to bulk edit captions with this application. This is mostly for a final manual review step to check individual captions together with the image in a preview, and change them if necessary.


- [image-set-tag-editor](#image-set-tag-editor)
  - [Installing](#installing)
  - [Usage](#usage)
    - [1. Starting Up](#1-starting-up)
    - [2. Navigating Through Images](#2-navigating-through-images)
      - [Using Keyboard:](#using-keyboard)
      - [Using Mouse:](#using-mouse)
    - [3. Editing Image Captions](#3-editing-image-captions)
    - [4. Saving Captions](#4-saving-captions)
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

### 4. Saving Captions

To save your edited captions, click on the **Save** button located in the bottom left corner of the application. This will overwrite (or create) all captions with the current data. Ensure you've made all necessary changes before saving.

You can also see a video of the application in action here:

https://github.com/NetroScript/image-set-tag-editor/assets/18115780/d91b32d6-5059-4149-aa97-ae9c495fb47c

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
