<script lang="ts">
	import Icon, { type IconName } from './Icon.svelte';

	// a round icon button with the ring of the design, filled or outlined
	let {
		icon,
		label,
		filled = true,
		active = false,
		size = 'md',
		href = null,
		onclick,
	}: {
		icon: IconName;
		label: string;
		filled?: boolean;
		active?: boolean;
		size?: 'sm' | 'md';
		href?: string | null;
		onclick?: (e: MouseEvent) => void;
	} = $props();
</script>

<svelte:element
	this={href ? 'a' : 'button'}
	{href}
	type={href ? undefined : 'button'}
	class="round size-{size}"
	class:filled
	aria-label={label}
	aria-pressed={href ? undefined : active}
	title={label}
	{onclick}
	role={href ? undefined : 'button'}
>
	<span class="inner"><Icon name={icon} filled={active} /></span>
</svelte:element>

<style lang="scss">
	.round {
		display: inline-flex;
		padding: 0.1875rem;
		border: 1px solid var(--line);
		border-radius: 50%;
		cursor: pointer;
		transition: border-color 0.3s ease;
	}

	.inner {
		display: flex;
		width: 3rem;
		height: 3rem;
		align-items: center;
		justify-content: center;
		border: 1px solid var(--accent);
		border-radius: 50%;
		color: var(--accent);
		font-size: 1.25rem;
		transition:
			background-color 0.3s ease,
			color 0.3s ease;
	}

	.size-sm .inner {
		width: 2rem;
		height: 2rem;
		font-size: 0.875rem;
	}

	.filled .inner {
		background: var(--accent);
		color: var(--text-invert);
	}

	@include hover {
		.round:hover {
			border-color: var(--accent);
		}

		.round:not(.filled):hover .inner {
			background: var(--accent);
			color: var(--text-invert);
		}
	}
</style>
