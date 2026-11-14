<script>
	import { getCrelte, getGlobal, getRouter } from 'crelte';
	import Button from '@/components/Button.svelte';
	import Join from '@/components/Join.svelte';
	import RecipeView from '@/components/recipe/RecipeView.svelte';
	import {
		MemberError,
		deleteEntry,
		errorMessage,
		forgetQueries,
		getEntry,
		getImages,
		publishError,
		setPublished,
	} from '@/lib/members';
	import { fromMember, imageIds } from '@/lib/recipe';
	import { getSession } from '@/lib/session.svelte';
	import { getToasts } from '@/lib/toasts.svelte';

	/** @import { MemberEntry } from '@/lib/members' */
	/** @import { ViewRecipe } from '@/lib/recipe' */

	// a recipe of the user from /members, private or public: a private one
	// (unpublished) is in no list and not in GraphQL, only its author sees
	// it here
	let { entry: page } = $props();

	const crelte = getCrelte();
	const session = getSession();
	const toasts = getToasts();
	const router = getRouter();
	const categories = getGlobal('categories');

	let entry = $state(/** @type {MemberEntry | null} */ (null));
	let recipe = $state(/** @type {ViewRecipe | null} */ (null));
	/** @type {string | null} */
	let problem = $state(null);
	let busy = $state(false);
	let started = false;

	let hasPhoto = $derived(!!entry?.fields.image?.length);
	let publicPath = $derived(
		entry?.published && entry.url ? new URL(entry.url).pathname : null,
	);

	$effect(() => {
		if (!session.user || started) return;
		started = true;
		load(session.user.name || null);
	});

	/** @param {string | null} cook */
	async function load(cook) {
		const id = page.recipeId;
		if (!/^\d+$/.test(id ?? '')) {
			problem = 'This recipe does not exist (anymore).';
			return;
		}
		try {
			const e = await getEntry(id);
			const images = await getImages(imageIds(e.fields));
			entry = e;
			recipe = fromMember(
				e,
				new Map(images.map(i => [i.id, i])),
				$categories ?? [],
				cook,
			);
		} catch (err) {
			problem =
				err instanceof MemberError && err.status === 404
					? 'This recipe does not exist (anymore).'
					: errorMessage(err);
		}
	}

	/** @param {boolean} published */
	async function publish(published) {
		if (!entry || busy) return;
		busy = true;
		try {
			const r = await setPublished(entry, published);
			forgetQueries(crelte);
			entry = r.entry;
			if (r.saved === 'draft') {
				toasts.show('Saved, an editor will publish it.');
			} else if (published) {
				toasts.show('The recipe has been published.');
				if (publicPath) router.open(publicPath);
			} else {
				toasts.show('The recipe is private now, only you see it.');
			}
		} catch (err) {
			toasts.error(publishError(err));
		} finally {
			busy = false;
		}
	}

	async function remove() {
		if (!entry || busy) return;
		if (!confirm(`Delete “${entry.title}”? This cannot be undone.`)) return;
		busy = true;
		try {
			await deleteEntry(entry.id);
			forgetQueries(crelte);
			toasts.show('The recipe has been deleted successfully.');
			router.open('/account');
		} catch (err) {
			toasts.error(errorMessage(err));
		} finally {
			busy = false;
		}
	}
</script>

{#if !session.user}
	<Join title="Your Recipes" signIn="/sign-in?next=/account" />
{:else if problem}
	<div class="note narrow">
		<h1 class="h1">Your Recipe</h1>
		<p class="lead">{problem}</p>
		<Button href="/account" look="secondary">My Recipes</Button>
	</div>
{:else if !entry || !recipe}
	<div class="note narrow">
		<h1 class="h1">Your Recipe</h1>
		<p class="lead">Loading …</p>
	</div>
{:else}
	<div class="page">
		<div class="bar">
			<div class="bar-inner">
				<p class="state small">
					<span class="badge">
						{entry.published ? 'Public' : 'Private'}
					</span>
					{entry.published
						? 'Everybody sees it.'
						: hasPhoto
							? 'Only you see it.'
							: 'Only you see it. Add your own photo to publish.'}
				</p>
				<div class="buttons">
					<Button look="secondary" disabled={busy} onclick={remove}>
						Delete
					</Button>
					<Button href="/recipes/edit/{entry.id}" look="secondary">
						Edit
					</Button>
					{#if entry.published}
						<Button
							look="secondary"
							disabled={busy}
							onclick={() => publish(false)}
						>
							Make private
						</Button>
					{:else}
						<Button disabled={busy} onclick={() => publish(true)}>
							Publish
						</Button>
					{/if}
				</div>
			</div>
		</div>
		<RecipeView {recipe} actions={false} />
	</div>
{/if}

<style lang="scss">
	.note {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 2rem;
		padding-block: 5rem var(--sy-section);
	}

	.page {
		display: flex;
		flex-direction: column;
	}

	// like the editor's: mobile at the bottom, else under the header
	.bar {
		position: sticky;
		bottom: 0;
		order: 1;
		z-index: 90;
		border-top: 1px solid var(--line);
		background: var(--bg);
	}

	.bar-inner {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
		padding: 0.75rem var(--sx-body);
	}

	.state {
		color: var(--accent-soft);
	}

	.badge {
		display: inline-block;
		margin-right: 0.5rem;
		padding: 0 0.5rem;
		border: 1px solid var(--accent-soft);
		border-radius: 999px;
	}

	.buttons {
		display: flex;
		gap: 0.5rem;
		width: 100%;

		> :global(*) {
			flex: 1;
		}
	}

	@include tablet {
		.bar {
			top: var(--topbar-height);
			bottom: auto;
			order: 0;
			border-top: 0;
			border-bottom: 1px solid var(--line);
		}

		.bar-inner {
			gap: 1rem;
		}

		.buttons {
			width: auto;
			gap: 1rem;

			> :global(*) {
				flex: none;
			}
		}
	}
</style>
