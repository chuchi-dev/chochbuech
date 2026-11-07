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
	// the chip and the arrow share one height; the invisible select grows
	// to a finger's height without making the boxes taller
	.chip-select {
		--chip-h: 1.875rem;
		position: relative;
		display: inline-flex;
		flex-shrink: 0;
		align-items: center;
		font-size: 0.875rem;
		line-height: 1;
	}

	.chip {
		display: flex;
		height: var(--chip-h);
		align-items: center;
		padding: 0 0.75rem;
		border: 1px solid var(--accent);
		border-radius: 999px;
		white-space: nowrap;
	}

	.arrow {
		display: flex;
		width: var(--chip-h);
		height: var(--chip-h);
		align-items: center;
		justify-content: center;
		border: 1px solid var(--accent);
		border-radius: 50%;
		font-size: 1rem;
		transition:
			background-color 0.3s ease,
			color 0.3s ease;
	}

	select {
		position: absolute;
		top: 50%;
		left: 0;
		width: 100%;
		height: max(100%, 2.75rem);
		opacity: 0;
		cursor: pointer;
		font-size: 1rem;
		transform: translateY(-50%);
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

	@include desktop {
		.chip-select {
			--chip-h: 2.125rem;
			font-size: 1rem;
		}

		.arrow {
			font-size: 1.125rem;
		}
	}
</style>
