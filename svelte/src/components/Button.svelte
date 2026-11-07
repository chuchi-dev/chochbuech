<script lang="ts">
	import type { Snippet } from 'svelte';

	// the pill of the design with its thin ring around: an `a` with href,
	// else a button
	let {
		href = null,
		target = null,
		type = 'button',
		look = 'primary',
		size = 'md',
		disabled = false,
		onclick,
		children,
		...rest
	}: {
		href?: string | null;
		target?: string | null;
		type?: 'button' | 'submit';
		look?: 'primary' | 'secondary' | 'invert';
		size?: 'sm' | 'md';
		disabled?: boolean;
		onclick?: (e: MouseEvent) => void;
		children: Snippet;
		[attr: string]: unknown;
	} = $props();
</script>

<svelte:element
	this={href ? 'a' : 'button'}
	{href}
	target={href ? target : undefined}
	type={href ? undefined : type}
	disabled={href ? undefined : disabled}
	class="btn look-{look} size-{size}"
	{onclick}
	role={href ? undefined : 'button'}
	{...rest}
>
	<span class="inner">{@render children()}</span>
</svelte:element>

<style lang="scss">
	.btn {
		display: inline-flex;
		padding: 0.25rem;
		border: 1px solid var(--line);
		border-radius: 999px;
		cursor: pointer;
		transition: border-color 0.3s ease;

		&:disabled {
			opacity: 0.5;
			cursor: default;
		}
	}

	.inner {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border: 1px solid var(--accent);
		border-radius: 999px;
		line-height: 1;
		white-space: nowrap;
		transition:
			background-color 0.3s ease,
			color 0.3s ease;
	}

	.size-sm .inner {
		padding: 0.5rem 1rem;
	}

	.look-primary .inner {
		background: var(--accent);
		color: var(--text-invert);
	}

	.look-secondary .inner {
		color: var(--accent);
	}

	.look-invert {
		border-color: rgba(246, 246, 244, 0.4);

		.inner {
			border-color: var(--bg);
			background: var(--bg);
			color: var(--accent);
		}
	}

	@include hover {
		.btn:not(:disabled):hover {
			border-color: var(--accent);
		}

		.look-secondary:hover .inner {
			background: var(--accent);
			color: var(--text-invert);
		}

		.look-invert:hover {
			border-color: var(--bg);
		}
	}
</style>
