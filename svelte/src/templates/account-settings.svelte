<script>
	import { getRouter } from 'crelte';
	import AccountLayout from '@/components/AccountLayout.svelte';
	import Button from '@/components/Button.svelte';
	import { getMock } from '@/lib/mock.svelte';
	import { getToasts } from '@/lib/toasts.svelte';

	// name, email and bio of the (mock) user, and deleting the account
	let { entry } = $props();

	const mock = getMock();
	const toasts = getToasts();
	const router = getRouter();

	let name = $state(mock.user?.name ?? '');
	let email = $state(mock.user?.email ?? '');
	let bio = $state(mock.user?.bio ?? '');

	/** @param {SubmitEvent} e */
	function save(e) {
		e.preventDefault();
		mock.updateUser({
			name: name.trim(),
			email: email.trim(),
			bio: bio.trim(),
		});
		toasts.show('Your account has been saved.');
	}

	function deleteAccount() {
		const ok = confirm(
			'Delete your account? This signs you out and deletes the recipes you wrote in this browser.',
		);
		if (!ok) return;
		for (const r of [...mock.recipes]) mock.deleteRecipe(r.id);
		mock.signOut();
		toasts.show(
			'Your account has been deleted (a demo: it only lived in this browser).',
		);
		router.open('/');
	}
</script>

<AccountLayout title={entry.title}>
	<div class="settings">
		<form onsubmit={save}>
			<label>
				<span>Name</span>
				<input
					type="text"
					bind:value={name}
					autocomplete="name"
					required
				/>
			</label>
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
				<span>Bio</span>
				<textarea
					bind:value={bio}
					rows="5"
					placeholder="A few words about you and your cooking"
				></textarea>
			</label>
			<div class="submit">
				<Button type="submit">Save</Button>
			</div>
		</form>

		<div class="danger">
			<p class="small">
				Accounts are a demo: they only live in this browser. Deleting
				signs you out and removes the recipes you wrote here.
			</p>
			<button type="button" class="delete" onclick={deleteAccount}>
				Delete account
			</button>
		</div>
	</div>
</AccountLayout>

<style lang="scss">
	.settings {
		max-width: calc(32rem + 2 * var(--sx-body));
		margin-inline: auto;
		padding-inline: var(--sx-body);
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	label {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	input,
	textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		border: 1px solid var(--line);
		background: var(--bg-card);
		color: var(--accent);
		font: inherit;
		transition: border-color 0.3s ease;

		&:focus {
			border-color: var(--accent);
		}
	}

	textarea {
		resize: vertical;
	}

	::placeholder {
		color: var(--accent-soft);
	}

	.submit {
		display: flex;
		justify-content: center;
		margin-top: 1rem;
	}

	.danger {
		margin-top: 5rem;
		padding-top: 2rem;
		border-top: 1px solid var(--line);
		text-align: center;

		p {
			color: var(--accent-soft);
		}
	}

	.delete {
		@include underline;
		margin-top: 1rem;
		color: var(--error);
		cursor: pointer;
	}

	@include hover {
		.delete:hover {
			opacity: 0.6;
		}
	}
</style>
