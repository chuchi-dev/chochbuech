<script>
	import { getRoute, getRouter } from 'crelte';
	import Button from '@/components/Button.svelte';
	import { AuthError, errorMessage, getSession } from '@/lib/session.svelte';
	import { getToasts } from '@/lib/toasts.svelte';

	// a new password: without a token it mails a link here, with the
	// `?token=` of the link (a reset or an invite) it sets the password and
	// logs in
	let { entry } = $props();

	const route = getRoute();
	const router = getRouter();
	const session = getSession();
	const toasts = getToasts();

	let token = $derived($route.getSearchParam('token'));
	let title = $derived(token ? 'Choose a Password' : 'Forgot your Password?');

	let email = $state('');
	let password = $state('');
	let repeat = $state('');
	let busy = $state(false);
	let sent = $state(false);
	// the link was used or is too old: ask for a new one
	let expired = $state(false);
	/** @type {string | null} */
	let error = $state(null);

	/** @param {SubmitEvent} e */
	async function requestLink(e) {
		e.preventDefault();
		busy = true;
		error = null;
		try {
			await session.requestReset(email.trim());
			sent = true;
		} catch (e) {
			error = errorMessage(e);
		} finally {
			busy = false;
		}
	}

	/** @param {SubmitEvent} e */
	async function setPassword(e) {
		e.preventDefault();
		if (password !== repeat) {
			error = 'The passwords are not the same.';
			return;
		}
		busy = true;
		error = null;
		try {
			await session.setPassword(token ?? '', password);
			toasts.show('Your password has been saved.');
			router.open('/account');
		} catch (e) {
			// zipp's only request error here: the link is used or expired
			if (e instanceof AuthError && e.type === 'REQUEST') expired = true;
			else error = errorMessage(e);
		} finally {
			busy = false;
		}
	}
</script>

<svelte:head>
	<title>{entry.title} | Chuchi Recipes</title>
</svelte:head>

<div class="reset">
	<h1 class="h1"><i>{expired ? 'Link Expired' : title}</i></h1>

	{#if token && expired}
		<p class="text">
			This link was used already or is too old. Ask for a new one, it
			comes by email.
		</p>
		<div class="buttons">
			<Button href="/reset-password">New Link</Button>
		</div>
	{:else if token}
		<form onsubmit={setPassword}>
			<label>
				<span>New password</span>
				<input
					type="password"
					bind:value={password}
					autocomplete="new-password"
					minlength="8"
					required
				/>
			</label>
			<label>
				<span>Repeat it</span>
				<input
					type="password"
					bind:value={repeat}
					autocomplete="new-password"
					minlength="8"
					required
				/>
			</label>
			<p class="small hint">At least 8 characters.</p>
			{#if error}
				<p class="error" role="alert">{error}</p>
			{/if}
			<div class="submit">
				<Button type="submit" disabled={busy}>Save and Sign In</Button>
			</div>
		</form>
	{:else if sent}
		<p class="text">
			If there is an account with <strong>{email}</strong>
			, we sent it a link to choose a new password. It works for an hour.
		</p>
		<div class="buttons">
			<Button href="/sign-in" look="secondary">Sign In</Button>
		</div>
	{:else}
		<p class="text">
			Enter the email of your account and we send you a link to choose a
			new password.
		</p>
		<form onsubmit={requestLink}>
			<label>
				<span>Email</span>
				<input
					type="email"
					bind:value={email}
					autocomplete="email"
					required
				/>
			</label>
			{#if error}
				<p class="error" role="alert">{error}</p>
			{/if}
			<div class="submit">
				<Button type="submit" disabled={busy}>Send Link</Button>
			</div>
		</form>
		<p class="back">
			<a href="/sign-in">Back to sign in</a>
		</p>
	{/if}
</div>

<style lang="scss">
	.reset {
		display: flex;
		flex-direction: column;
		align-items: center;
		max-width: calc(26rem + 2 * var(--sx-body));
		margin-inline: auto;
		padding: 7.5rem var(--sx-body) var(--sy-section);
		text-align: center;
	}

	.text {
		margin-top: 3rem;
	}

	form {
		display: flex;
		width: 100%;
		flex-direction: column;
		gap: 1.5rem;
		margin-top: 3rem;
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

	.hint {
		color: var(--accent-soft);
	}

	.error {
		color: var(--error);
	}

	.submit {
		display: flex;
		justify-content: center;
		margin-top: 1rem;
	}

	.buttons {
		display: flex;
		justify-content: center;
		margin-top: 3rem;
	}

	strong {
		font-weight: inherit;
		font-family: var(--font-serif);
		font-style: italic;
	}

	.back {
		margin-top: 2.5rem;

		a {
			@include underline;
		}
	}

	@include desktop {
		.reset {
			padding-top: 8.75rem;
		}
	}

	@include hover {
		.back a:hover {
			opacity: 0.6;
		}
	}
</style>
