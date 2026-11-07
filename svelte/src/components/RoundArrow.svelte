<script lang="ts">
	import Icon from './Icon.svelte';

	// the round arrow of the mobile dropdowns and accordions: outlined and
	// down when closed, filled and up when open. Only the look, it goes
	// inside a button.
	let { open = false }: { open?: boolean } = $props();
</script>

<span class="arrow" class:open aria-hidden="true">
	<Icon name={open ? 'up' : 'down'} />
</span>

<style lang="scss">
	.arrow {
		display: inline-flex;
		flex-shrink: 0;
		width: 2rem;
		height: 2rem;
		align-items: center;
		justify-content: center;
		border: 1px solid var(--accent);
		border-radius: 50%;
		color: var(--accent);
		font-size: 1rem;
		transition:
			background-color 0.3s ease,
			color 0.3s ease;

		&.open {
			background: var(--accent);
			color: var(--text-invert);
		}
	}
</style>
