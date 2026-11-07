<script lang="ts">
	import { tick } from 'svelte';
	import Icon from '../Icon.svelte';
	import type { Sortable } from './sortable.svelte';

	// the move handle of the design: dragging it moves its row (sortable),
	// a click opens "Move up / Move down", the arrow keys move directly
	let {
		label,
		look = 'round',
		canUp,
		canDown,
		onmove,
		sortable,
		index,
	}: {
		label: string;
		look?: 'round' | 'cell';
		canUp: boolean;
		canDown: boolean;
		onmove: (dir: -1 | 1) => void;
		sortable: Sortable;
		index: number;
	} = $props();

	let open = $state(false);
	let root: HTMLElement;
	let handle: HTMLButtonElement;

	// the row moved in the page, the handle lost the focus
	async function move(dir: -1 | 1) {
		open = false;
		onmove(dir);
		await tick();
		handle.focus();
	}

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'ArrowUp' && canUp) {
			e.preventDefault();
			move(-1);
		} else if (e.key === 'ArrowDown' && canDown) {
			e.preventDefault();
			move(1);
		} else if (e.key === 'Escape') {
			open = false;
		}
	}

	function onwindowclick(e: MouseEvent) {
		if (open && !root.contains(e.target as Node)) open = false;
	}
</script>

<svelte:window onclick={onwindowclick} />

<span class="move look-{look}" bind:this={root}>
	<button
		type="button"
		class="handle"
		aria-label={label}
		title="{label}: drag or click"
		aria-expanded={open}
		disabled={!canUp && !canDown}
		bind:this={handle}
		onpointerdown={e => sortable.start(e, index)}
		onclick={() => !sortable.dragged && (open = !open)}
		{onkeydown}
	>
		<span class="inner"><Icon name="move" /></span>
	</button>
	{#if open}
		<span class="menu">
			<button type="button" disabled={!canUp} onclick={() => move(-1)}>
				<Icon name="up" /> Move up
			</button>
			<button type="button" disabled={!canDown} onclick={() => move(1)}>
				<Icon name="down" /> Move down
			</button>
		</span>
	{/if}
</span>

<style lang="scss">
	.move {
		position: relative;
		display: inline-flex;
	}

	.handle {
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: grab;
		touch-action: none;

		&:disabled {
			opacity: 0.4;
			cursor: default;
		}
	}

	// like RoundButton without its ring: only the ring's space
	.look-round .handle {
		padding: 0.1875rem;
		border: 1px solid transparent;
		border-radius: 50%;

		.inner {
			display: flex;
			width: 1.75rem;
			height: 1.75rem;
			align-items: center;
			justify-content: center;
			border: 1px solid var(--accent);
			border-radius: 50%;
			font-size: 0.875rem;
			transition:
				background-color 0.3s ease,
				color 0.3s ease;
		}
	}

	.look-cell {
		width: 100%;
		height: 100%;

		.handle {
			width: 100%;
			height: 100%;
			font-size: 1rem;
		}
	}

	.menu {
		position: absolute;
		top: calc(100% + 0.25rem);
		left: 0;
		z-index: 20;
		display: flex;
		flex-direction: column;
		min-width: 9rem;
		border: 1px solid var(--line);
		background: var(--bg-card);

		button {
			display: flex;
			align-items: center;
			gap: 0.5rem;
			padding: 0.5rem 0.75rem;
			text-align: left;
			white-space: nowrap;
			cursor: pointer;
			transition:
				background-color 0.3s ease,
				color 0.3s ease;

			& + button {
				border-top: 1px solid var(--line);
			}

			&:disabled {
				opacity: 0.4;
				cursor: default;
			}
		}
	}

	@include hover {
		.look-round .handle:not(:disabled):hover .inner {
			background: var(--accent);
			color: var(--text-invert);
		}

		.look-cell .handle:not(:disabled):hover {
			background: var(--bg);
		}

		.menu button:not(:disabled):hover {
			background: var(--accent);
			color: var(--text-invert);
		}
	}
</style>
