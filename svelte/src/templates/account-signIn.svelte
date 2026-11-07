<script>
	import { getRoute, getRouter } from 'crelte';
	import Button from '@/components/Button.svelte';
	import Join from '@/components/Join.svelte';
	import { errorMessage, getSession } from '@/lib/session.svelte';
	import { getToasts } from '@/lib/toasts.svelte';

	// logging in with a zipp account; `?next=` goes there after. There is no
	// sign up yet, we create the accounts: `?register` (the home's Sign up
	// now) shows a guest the invitation instead of the form.
	let { entry } = $props();

	const route = getRoute();
	const router = getRouter();
	const session = getSession();
	const toasts = getToasts();

	let next = $derived(safeNext($route.getSearchParam('next')));
	let register = $derived($route.getSearchParam('register') !== null);

	let email = $state('');
	let password = $state('');
	let busy = $state(false);
	/** @type {string | null} */
	let error = $state(null);

	/** only paths of this site @param {string | null} p */
	function safeNext(p) {
		return p && p.startsWith('/') && !p.startsWith('//') ? p : null;
	}

	/** @param {SubmitEvent} e */
	async function submit(e) {
		e.preventDefault();
		busy = true;
		error = null;
		try {
			const user = await session.login(email.trim(), password);
			toasts.show(user.name ? `Welcome, ${user.name}!` : 'Welcome!');
			router.open(next ?? '/account');
		} catch (e) {
			// a wrong password, the wait after too many: zipp says it
			error = errorMessage(e);
			password = '';
		} finally {
			busy = false;
		}
	}

	async function logOut() {
		try {
			await session.logout();
			toasts.show('You have been logged out.');
		} catch (e) {
			toasts.error(errorMessage(e));
		}
	}
</script>

<svelte:head>
	<title>{entry.title} | Chuchi Recipes</title>
</svelte:head>

{#if register && !session.user}
	<Join title="Share your Recipes" signIn="/sign-in" />
{:else}
	<div class="sign-in">
		<h1 class="h1"><i>{entry.title}</i></h1>

		{#if session.user}
			<p class="signed-in">
				You are signed in as <strong>{session.user.name}</strong>
				({session.user.email}).
			</p>
			<div class="buttons">
				<Button href="/account">My Recipes</Button>
				<Button look="secondary" onclick={logOut}>Log out</Button>
			</div>
		{:else}
			<form onsubmit={submit}>
				<label>
					<span>Email</span>
					<input
						type="email"
						bind:value={email}
						autocomplete="email"
						required
					/>
				</label>
				<label>
					<span>Password</span>
					<input
						type="password"
						bind:value={password}
						autocomplete="current-password"
						required
					/>
				</label>
				{#if error}
					<p class="error" role="alert">{error}</p>
				{/if}
				<div class="submit">
					<Button type="submit" disabled={busy}>Sign In</Button>
				</div>
			</form>

			<p class="switch">
				<a href="/reset-password">Forgot your password?</a>
			</p>

			<p class="note small">
				Want to share your recipes? Accounts are by invitation for now:
				write us at
				<a href="mailto:hello@chochbuech.com">hello@chochbuech.com</a>
			</p>
		{/if}
	</div>
{/if}

<style lang="scss">
	.sign-in {
		display: flex;
		flex-direction: column;
		align-items: center;
		max-width: calc(26rem + 2 * var(--sx-body));
		margin-inline: auto;
		padding: 7.5rem var(--sx-body) var(--sy-section);
		text-align: center;
	}

	form {
		display: flex;
		width: 100%;
		flex-direction: column;
		gap: 1.5rem;
		margin-top: 4rem;
		text-align: left;
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	input {
		width: 100%;
		padding: 0.75rem 1rem;
		border: 1px solid var(--line);
		background: var(--bg-card);
		color: var(--accent);
		transition: border-color 0.3s ease;

		&:focus {
			border-color: var(--accent);
		}
	}

	.submit {
		display: flex;
		justify-content: center;
		margin-top: 1rem;
	}

	.switch {
		margin-top: 2.5rem;

		a {
			@include underline;
		}
	}

	.note {
		margin-top: 1.5rem;
		color: var(--accent-soft);

		a {
			@include underline;
		}
	}

	.error {
		color: var(--error);
	}

	.signed-in {
		margin-top: 3rem;
	}

	strong {
		font-weight: inherit;
		font-family: var(--font-serif);
		font-style: italic;
	}

	.buttons {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 1rem;
		margin-top: 3rem;
	}

	@include desktop {
		.sign-in {
			padding-top: 8.75rem;
		}
	}

	@include hover {
		.switch a:hover,
		.note a:hover {
			opacity: 0.6;
		}
	}
</style>
