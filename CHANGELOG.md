## 0.1.2

### Added

* Calculate CLIP token count for the current caption
* Add custom CSS by pressing `Alt + C`

### Changed

* Correct tag filtering with only excluding tags
* Fixed removal and detection of tags (do not include seperator in tag name, instead operate on tag list)
* Bumped package versions (both NodeJS and Rust)


## 0.1.1

* Added automatic counting and adding and removing of individual reoccuring tags
  * The most frequent tags are shown on the side, however it is possible to filter for tags (it can be filtered for multiple tags at once by separating them with `|`, if the first character is a `-`, the tag will be excluded from the results, if the first character (after the optional `-`) is `^` or the last character is `$` the current search term will be parsed as regex instead of a simple string search)
  * Commonly in the context of Stable Diffusion for example tags are comma separated, so this is also the default string which is used for splitting the tags