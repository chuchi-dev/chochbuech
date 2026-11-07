<script lang="ts">
	// the round tabs of the design: the categories filter, the account
	// navigation. With href links, else buttons that call onselect.
	type Item = { key: string; label: string; href?: string };

	let {
		items,
		active,
		label,
		onselect,
	}: {
		items: Item[];
		active: string;
		label: string;
		onselect?: (key: string) => void;
	} = $props();
</script>

<nav class="pills" aria-label={label}>
	{#each items as item (item.key)}
		{#if item.href}
			<a
				href={item.href}
				class:active={item.key === active}
				aria-current={item.key === active ? 'page' : undefined}
				data-disable-scroll
			>
				{item.label}
			</a>
		{:else}
			<button
				type="button"
				class:active={item.key === active}
				aria-pressed={item.key === active}
				onclick={() => onselect?.(item.key)}
			>
				{item.label}
			</button>
		{/if}
	{/each}
</nav>

<style lang="scss">
	// the same space as between the visible circles of the slider arrows
	// (RecipeRow), also between wrapped lines
	.pills {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 0.5rem;
	}

	a,
	button {
		padding: 0.25rem 0.75rem;
		border: 1px solid var(--line);
		border-radius: 999px;
		cursor: pointer;
		transition:
			background-color 0.3s ease,
			color 0.3s ease,
			border-color 0.3s ease;

		&.active {
			border-color: var(--accent);
			background: var(--accent);
			color: var(--text-invert);
		}
	}

	@include hover {
		a:not(.active):hover,
		button:not(.active):hover {
			border-color: var(--accent);
		}
	}
</style>
