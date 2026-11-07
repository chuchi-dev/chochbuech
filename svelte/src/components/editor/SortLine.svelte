<script lang="ts">
	import type { Sortable } from './sortable.svelte';

	// where a dragged row would go: a line in the container of the rows,
	// which has position relative
	let { sortable }: { sortable: Sortable } = $props();
</script>

{#if sortable.line !== null}
	<div class="sort-line" style:top="{sortable.line - 1}px"></div>
{/if}

<style>
	.sort-line {
		position: absolute;
		right: 0;
		left: 0;
		z-index: 10;
		height: 2px;
		background: var(--accent);
		pointer-events: none;
	}
</style>
