<script>
	import { getRoute, getRouter } from 'crelte';
	import Button from '@/components/Button.svelte';
	import { getMock } from '@/lib/mock.svelte';
	import { getToasts } from '@/lib/toasts.svelte';

	// signing in and up, a mock: any email and password works and nothing
	// leaves the browser. `?register` shows sign up, `?next=` goes there after.
	let { entry } = $props();

	const route = getRoute();
	const router = getRouter();
	const mock = getMock();
	const toasts = getToasts();

	let register = $derived($route.getSearchParam('register') !== null);
	let next = $derived(safeNext($route.getSearchParam('next')));
	let title = $derived(register ? 'Sign Up' : 'Sign In');
	let switchUrl = $derived.by(() => {
		// eslint-disable-next-line svelte/prefer-svelte-reactivity -- a local temporary
		const params = new URLSearchParams();
		if (!register) params.set('register', '');
		if (next) params.set('next', next);
		const s = params.toString().replace('register=', 'register');
		return '/sign-in' + (s ? '?' + s : '');
	});

	let name = $state('');
	let email = $state('');
	let password = $state('');

	/** only paths of this site @param {string | null} p */
	function safeNext(p) {
		return p && p.startsWith('/') && !p.startsWith('//') ? p : null;
	}

	/** @param {SubmitEvent} e */
	function submit(e) {
		e.preventDefault();
		const user = {
			name: register ? name.trim() : 'Sören Meier',
			email: email.trim(),
			bio: '',
		};
		mock.signIn(user);
		toasts.show(`Welcome, ${user.name}!`);
		router.open(next ?? '/account');
	}

	function logOut() {
		mock.signOut();
		toasts.show('You have been logged out.');
	}
</script>

<svelte:head>
	<title>{title} | Chuchi Recipes</title>
</svelte:head>

<div class="sign-in">
	<h1 class="h1"><i>{mock.user ? entry.title : title}</i></h1>

	{#if mock.user}
		<p class="signed-in">
			You are signed in as <strong>{mock.user.name}</strong>
			({mock.user.email}).
		</p>
		<div class="buttons">
			<Button href="/account">My Recipes</Button>
			<Button look="secondary" onclick={logOut}>Log out</Button>
		</div>
	{:else}
		<form onsubmit={submit}>
			{#if register}
				<label>
					<span>Name</span>
					<input
						type="text"
						bind:value={name}
						autocomplete="name"
						required
					/>
				</label>
			{/if}
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
					autocomplete={register
						? 'new-password'
						: 'current-password'}
					required
				/>
			</label>
			<div class="submit">
				<Button type="submit">{title}</Button>
			</div>
		</form>

		<p class="switch">
			{#if register}
				Already have an account? <a href={switchUrl}>Sign in</a>
			{:else}
				New here? <a href={switchUrl}>Create an account</a>
			{/if}
		</p>

		<p class="note small">
			Accounts are a demo: any email and password works and nothing leaves
			your browser.
		</p>
	{/if}
</div>

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
		.switch a:hover {
			opacity: 0.6;
		}
	}
</style>
