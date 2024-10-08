<script context="module" lang="ts">
	export type Toast = {
		message: string;
		status: 'success' | 'error';
	};

	export function toast(obj: Toast) {
		if (import.meta.env.SSR) throw new Error('Cannot use toast in SSR');

		window.NEW_TOAST(obj);
	}
</script>

<script lang="ts">
	let toasts = $state<Toast[]>([
		{
			message: 'Welcome to the app!',
			status: 'success',
		},
		{
			message: 'You have been signed out.',
			status: 'error',
		},
	]);

	function init() {
		if (import.meta.env.SSR) return;

		// @ts-ignore
		window.NEW_TOAST = d => {
			toasts = [...toasts, d];
		};
	}
	init();
</script>

{#if toasts.length}
	<div class="toasts">
		{#each toasts as toast}
			<div class="toast status-{toast.status}">
				{toast.message}
			</div>
		{/each}
	</div>
{/if}

<style lang="scss">
	.toasts {
		position: fixed;
		display: flex;
		bottom: 1rem;
		left: 50%;
		z-index: 1000;
		max-width: 38rem;
		width: 100%;
		flex-direction: column;
		gap: 1rem;
		transform: translateX(-50%);
		padding: 0 var(--sx-body);

		.toast {
			padding: 1.25rem 1.5rem;

			&.status-success {
				background-color: var(--blue);
				color: var(--white);
			}

			&.status-error {
				background-color: var(--red);
				color: var(--white);
			}
		}
	}
</style>
