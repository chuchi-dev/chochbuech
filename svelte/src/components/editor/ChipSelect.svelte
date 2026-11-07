<script lang="ts">
	import Icon from '../Icon.svelte';

	// the small unit chip of the editor with its round arrow: a native
	// select lies invisibly over it, so it works with keyboard and touch
	let {
		value = $bindable(),
		options,
		label,
	}: {
		value: string;
		options: [string, string][];
		label: string;
	} = $props();

	let text = $derived(options.find(([v]) => v === value)?.[1] ?? '–');
</script>

<span class="chip-select">
	<span class="chip" aria-hidden="true">{text}</span>
	<span class="arrow" aria-hidden="true"><Icon name="down" /></span>
	<select bind:value aria-label={label}>
		{#each options as [v, l] (v)}
			<option value={v}>{l}</option>
		{/each}
	</select>
</span>

<style lang="scss">
	.chip-select {
		position: relative;
		display: inline-flex;
		flex-shrink: 0;
		align-items: center;
		font-size: 0.75rem;
		line-height: 1.5;
	}

	.chip {
		padding: 0 0.5rem;
		border: 1px solid var(--accent);
		border-radius: 999px;
		white-space: nowrap;
	}

	.arrow {
		display: flex;
		width: 1.125rem;
		height: 1.125rem;
		align-items: center;
		justify-content: center;
		border: 1px solid var(--accent);
		border-radius: 50%;
		font-size: 0.625rem;
		transition:
			background-color 0.3s ease,
			color 0.3s ease;
	}

	select {
		position: absolute;
		inset: 0;
		width: 100%;
		opacity: 0;
		cursor: pointer;
		font-size: 1rem;
	}

	.chip-select:has(select:focus-visible) {
		outline: 2px solid var(--focus-color);
		outline-offset: 2px;
		border-radius: 999px;
	}

	@include hover {
		.chip-select:hover .arrow {
			background: var(--accent);
			color: var(--text-invert);
		}
	}
</style>
