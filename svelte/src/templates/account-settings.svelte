<script>
	import { getCrelte, getRouter } from 'crelte';
	import AccountLayout from '@/components/AccountLayout.svelte';
	import Button from '@/components/Button.svelte';
	import {
		errorMessage as uploadError,
		forgetQueries,
		uploadImage,
	} from '@/lib/members';
	import { errorMessage, getSession } from '@/lib/session.svelte';
	import { getToasts } from '@/lib/toasts.svelte';
	import cookQuery from '@/queries/cook.graphql';

	// the account of the user: their public profile (name, bio, photo; the
	// page /cooks/<slug> while a recipe of theirs is live) is theirs to
	// change, the email isn't (yet), a new password comes by mail
	let { entry } = $props();

	const session = getSession();
	const toasts = getToasts();
	const router = getRouter();
	const crelte = getCrelte();

	let name = $state(session.user?.name ?? '');
	let bio = $state(session.user?.fields?.bio ?? '');
	/** @type {number | null} */
	let photoId = $state(session.user?.fields?.photo?.[0] ?? null);
	// the preview: of an upload or of the public page
	/** @type {string | null} */
	let photoUrl = $state(null);
	// their public page, null while they have none
	/** @type {string | null} */
	let pageUrl = $state(null);
	let saving = $state(false);
	let uploading = $state(false);
	/** @type {HTMLInputElement} */
	let input;

	// the page (and the photo's url) only GraphQL knows
	$effect(() => {
		const slug = session.user?.slug;
		if (!slug) return;
		crelte
			.query(cookQuery, { slug })
			.then(data => {
				const cook = /** @type {any} */ (data).cook;
				pageUrl = cook?.url ?? null;
				if (!photoUrl) photoUrl = cook?.photo?.[0]?.url ?? null;
			})
			.catch(() => {});
	});

	async function upload() {
		const file = input.files?.[0];
		input.value = '';
		if (!file) return;
		uploading = true;
		try {
			const up = await uploadImage(file);
			photoId = up.id;
			photoUrl = up.url;
		} catch (e) {
			toasts.error(uploadError(e));
		} finally {
			uploading = false;
		}
	}

	function removePhoto() {
		photoId = null;
		photoUrl = null;
	}

	/** @param {SubmitEvent} e */
	async function save(e) {
		e.preventDefault();
		if (!name.trim()) {
			toasts.error('Please fill in your name.');
			return;
		}
		saving = true;
		try {
			await session.updateProfile(name.trim(), {
				bio: bio.trim(),
				photo: photoId ? [photoId] : [],
			});
			// the cards and the page show the new profile
			forgetQueries(crelte);
			toasts.show('Your profile has been saved.');
		} catch (e) {
			toasts.error(errorMessage(e));
		} finally {
			saving = false;
		}
	}

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
		<form onsubmit={save}>
			<label>
				<span>Name</span>
				<input
					type="text"
					autocomplete="name"
					required
					bind:value={name}
				/>
			</label>
			<label>
				<span>About you</span>
				<textarea
					rows="5"
					placeholder="What do you like to cook?"
					bind:value={bio}
				></textarea>
			</label>
			<div class="photo">
				<span id="photo-label">Photo</span>
				<div class="photo-row">
					<div class="preview">
						{#if photoUrl}
							<img src={photoUrl} alt="You" />
						{/if}
					</div>
					<div class="photo-side">
						<p class="hint small">
							{uploading
								? 'Uploading …'
								: photoId
									? 'Shown on your page.'
									: 'JPG, PNG or WebP, at most 8 MB.'}
						</p>
						<div class="photo-buttons">
							<Button
								look="secondary"
								size="sm"
								disabled={uploading}
								onclick={() => input.click()}
								aria-describedby="photo-label"
							>
								{photoId ? 'Another Photo' : 'Add a Photo'}
							</Button>
							{#if photoId}
								<Button
									look="secondary"
									size="sm"
									onclick={removePhoto}
								>
									Remove
								</Button>
							{/if}
						</div>
						<input
							bind:this={input}
							class="file"
							type="file"
							accept="image/jpeg,image/png,image/webp"
							tabindex="-1"
							aria-hidden="true"
							onchange={upload}
						/>
					</div>
				</div>
			</div>
			<p class="note small">
				{#if pageUrl}
					Your page:
					<a href={pageUrl}>{pageUrl.replace(/^https?:\/\//, '')}</a>
				{:else}
					Your page with your recipes is public once you share your
					first recipe.
				{/if}
			</p>
			<div class="submit">
				<Button type="submit" disabled={saving || uploading}>
					{saving ? 'Saving …' : 'Save'}
				</Button>
			</div>
		</form>

		<dl>
			<dt>Email</dt>
			<dd>{session.user?.email}</dd>
		</dl>

		<p class="note small">
			Changing your email isn't possible here yet: write us at
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

	form {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		margin-bottom: 3rem;
	}

	label,
	.photo {
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
		transition: border-color 0.3s ease;

		&:focus {
			border-color: var(--accent);
		}
	}

	textarea {
		resize: vertical;
	}

	.photo-row {
		display: flex;
		align-items: center;
		gap: 1.5rem;
	}

	.preview {
		flex: none;
		width: 5rem;
		height: 5rem;
		overflow: hidden;
		border: 1px solid var(--line);
		border-radius: 50%;
		background: var(--bg-card);

		img {
			width: 100%;
			height: 100%;
			object-fit: cover;
		}
	}

	.photo-side {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.hint {
		color: var(--accent-soft);
	}

	.photo-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.file {
		display: none;
	}

	.submit {
		display: flex;
		justify-content: center;
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
		color: var(--accent-soft);

		a {
			@include underline;
		}
	}

	dl + .note {
		margin-top: 1.5rem;
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
