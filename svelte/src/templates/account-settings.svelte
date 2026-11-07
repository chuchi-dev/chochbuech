<script>
	import { getRouter } from 'crelte';
	import AccountLayout from '@/components/AccountLayout.svelte';
	import Button from '@/components/Button.svelte';
	import { errorMessage, getSession } from '@/lib/session.svelte';
	import { getToasts } from '@/lib/toasts.svelte';

	// the account of the user: zipp has no routes to change it from the
	// website yet, so it is only shown; a new password comes by mail
	let { entry } = $props();

	const session = getSession();
	const toasts = getToasts();
	const router = getRouter();

	async function newPassword() {
		if (!session.user) return;
		try {
			await session.requestReset(session.user.email);
			toasts.show('We sent you a link to choose a new password.');
		} catch (e) {
			toasts.error(errorMessage(e));
		}
	}

	async function logOut() {
		try {
			await session.logout();
		} catch (e) {
			toasts.error(errorMessage(e));
			return;
		}
		toasts.show('You have been logged out.');
		router.open('/');
	}
</script>

<AccountLayout title={entry.title}>
	<div class="settings">
		<dl>
			<dt>Name</dt>
			<dd>{session.user?.name || '–'}</dd>
			<dt>Email</dt>
			<dd>{session.user?.email}</dd>
		</dl>

		<p class="note small">
			Changing your name or email isn't possible here yet: write us at
			<a href="mailto:hello@chochbuech.com">hello@chochbuech.com</a>
			.
		</p>

		<div class="buttons">
			<Button look="secondary" onclick={newPassword}>New Password</Button>
			<Button onclick={logOut}>Log out</Button>
		</div>
	</div>
</AccountLayout>

<style lang="scss">
	.settings {
		max-width: calc(32rem + 2 * var(--sx-body));
		margin-inline: auto;
		padding-inline: var(--sx-body);
	}

	dl {
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 1rem 2rem;
		padding-block: 1.5rem;
		border-block: 1px solid var(--line);
	}

	dt {
		color: var(--accent-soft);
	}

	dd {
		overflow-wrap: anywhere;
	}

	.note {
		margin-top: 1.5rem;
		color: var(--accent-soft);

		a {
			@include underline;
		}
	}

	.buttons {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 1rem;
		margin-top: 3rem;
	}

	@include hover {
		.note a:hover {
			opacity: 0.6;
		}
	}
</style>
