<script>
	import { fly } from 'svelte/transition';
	import { getToasts } from '@/lib/toasts.svelte';
	import Icon from './Icon.svelte';

	const toasts = getToasts();
</script>

<div class="toasts" aria-live="polite">
	{#each toasts.list as toast (toast.id)}
		<div class="toast" class:error={toast.error} transition:fly={{ y: 20 }}>
			<p>{toast.text}</p>
			<button
				type="button"
				aria-label="Close"
				onclick={() => toasts.close(toast.id)}
			>
				<Icon name={toast.error ? 'thumbsDown' : 'thumbsUp'} />
			</button>
		</div>
	{/each}
</div>

<style lang="scss">
	.toasts {
		position: fixed;
		left: var(--sx-body);
		right: var(--sx-body);
		bottom: var(--sx-body);
		z-index: 120;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		pointer-events: none;
	}

	.toast {
		display: flex;
		width: 100%;
		max-width: 30rem;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 1rem 1rem 1rem 1.5rem;
		background: var(--accent);
		color: var(--text-invert);
		pointer-events: auto;

		&.error {
			background: var(--error);
		}
	}

	button {
		display: flex;
		width: 2rem;
		height: 2rem;
		flex-shrink: 0;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		background: var(--bg);
		color: var(--accent);
		cursor: pointer;
	}

	.error button {
		color: var(--error);
	}

	@include desktop {
		.toast {
			padding-block: 0.75rem;
		}
	}
</style>
