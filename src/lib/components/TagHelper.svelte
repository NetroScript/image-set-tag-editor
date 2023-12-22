<script lang="ts">
	import { single_tag_delimiter, active_image, tagCounts, sortedTags } from '$lib/stores';

	let tag_filter_string = '';
	$: tag_white_list = tag_filter_string
		.split('|')
		.filter((tag) => tag != '' && !tag.startsWith('-'))
		.map((tag) => {
			if (tag.startsWith('^') || tag.endsWith('$')) {
				return new RegExp(tag);
			}
			return tag;
		});
	$: tag_black_list = tag_filter_string
		.split('|')
		.filter((tag) => tag != '' && tag.startsWith('-'))
		.map((tag) => {
			tag = tag.slice(1);
			if (tag.startsWith('^') || tag.endsWith('$')) {
				return new RegExp(tag);
			}
			return tag;
		});

	$: fitting_tags = $sortedTags
		.filter((tag) => {
			// Or when the filter string is empty
			if (tag_filter_string == '') return true;
			// Only return if any keyword in the white list is in the tag
			return (
				(tag_white_list.some((keyword) => {
					if (keyword instanceof RegExp) {
						return keyword.test(tag);
					}
					return tag.includes(keyword);
				}) ||
					tag_white_list.length == 0) &&
				!tag_black_list.some((keyword) => {
					if (keyword instanceof RegExp) {
						return keyword.test(tag);
					}
					return tag.includes(keyword);
				})
			);
		})
		.slice(0, 50)
		.sort((a, b) => a.localeCompare(b));
</script>

<hr class="my-2" />
<div>
	<label class="label">
		<span>Single Tag Delimiter</span>
		<input
			class="input variant-form-material"
			type="search"
			placeholder="Single Tag Delimiter"
			title="Single Tag Delimiter"
			on:keydown={(event) => {
				event.stopPropagation();
			}}
			bind:value={$single_tag_delimiter}
		/>
	</label>
</div>
<div>
	<label class="label">
		<span>Filter tags</span>
		<input
			class="input variant-form-material"
			type="search"
			placeholder="Filter tags"
			title="Filter tags"
			on:keydown={(event) => {
				event.stopPropagation();
			}}
			bind:value={tag_filter_string}
		/>
	</label>
</div>
<div class="overflow-y-scroll pl-1">
	{#each fitting_tags as tag}
		<label class="flex items-center space-x-2">
			<input
				class="checkbox"
				type="checkbox"
				checked={($active_image?.caption.split($single_tag_delimiter) || [])
					.map((tag) => tag.trim())
					.includes(tag)}
				on:click={(event) => {
					if ($active_image) {
						if (
							($active_image?.caption.split($single_tag_delimiter) || [])
								.map((tag) => tag.trim())
								.includes(tag)
						) {
							$active_image.caption = $active_image.caption
								.split($single_tag_delimiter)
								.filter((current_tag) => current_tag.trim() != tag)
								.join($single_tag_delimiter);
						} else {
							console.log('Adding tag ' + tag);
							$active_image.caption = tag + $single_tag_delimiter + ' ' + $active_image.caption;
							$active_image = $active_image;
						}
					}
				}}
			/>
			<div class="tag-display" data-tag={tag}>
				{tag}
				<div class="inline text-gray opacity-60">{$tagCounts[tag]}</div>
			</div>
		</label>
	{/each}
</div>
<hr class="my-2" />
