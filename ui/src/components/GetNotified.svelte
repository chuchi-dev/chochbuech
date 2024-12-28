<script>
	import { addWaitlist } from '@/api/waitlist';

	let email = $state('');
	let success = $state(false);

	async function onSubmit(e) {
		e.preventDefault();

		try {
			await addWaitlist(email);
			success = true;

			// @ts-ignore
			if (typeof sa_event === 'function') sa_event('add_waitlist');
		} catch (error) {
			console.error(error);
		}
	}
</script>

{#if success}
	<p class="notified">
		<span>
			Danke für dein Interesse! Wir halten dich auf dem Laufenden.
		</span>
	</p>
{:else}
	<form onsubmit={onSubmit} class="input-form get-notified">
		<input
			type="email"
			name="email"
			placeholder="Deine E-Mail-Adresse"
			bind:value={email}
		/>
		<button>→</button>
	</form>
{/if}

<style lang="scss">
	form {
		display: block;
		width: 100%;
		max-width: 32rem;
		margin: 0 auto;
	}

	.notified {
		display: flex;
		align-items: center;
		padding: 0 1rem;
		height: 4rem;
		max-width: 32rem;
		margin: 0 auto;
		border: 1px solid var(--blue);
		border-radius: 5rem;

		span {
			display: block;
			width: 100%;
		}
	}
</style>
