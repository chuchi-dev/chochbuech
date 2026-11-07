<script module>
	import recipesQuery from '@/queries/recipes.graphql';

	/** @type {import('crelte').LoadData} */
	export const loadData = (cr, entry) =>
		cr.query(recipesQuery, {
			author: entry.cook.id,
			limit: 100,
			offset: 0,
		});
</script>

<script>
	import RecipeGrid from '@/components/RecipeGrid.svelte';

	// the public page of a cook (a user with a live recipe, see routes.js):
	// who, since when and their recipes
	let { entry, recipes } = $props();

	let cook = $derived(entry.cook);
	let photo = $derived(cook.photo?.[0] ?? null);
</script>

<svelte:head>
	<title>{cook.name} | Chuchi Recipes</title>
</svelte:head>

<div class="cook narrow">
	<header>
		{#if photo}
			<img class="photo" src={photo.url} alt={photo.alt ?? ''} />
		{/if}
		<h1 class="h1"><i>{cook.name}</i></h1>
		{#if cook.memberSince}
			<p class="since">Community Member since {cook.memberSince}</p>
		{/if}
		{#if cook.bio}
			<p class="bio lead">{cook.bio}</p>
		{/if}
	</header>

	<RecipeGrid {recipes} empty="No recipes yet." />
</div>

<style lang="scss">
	.cook {
		padding-block: 5rem var(--sy-section);
	}

	header {
		display: flex;
		flex-direction: column;
		align-items: center;
		margin-bottom: 4rem;
		text-align: center;
	}

	.photo {
		width: 7.5rem;
		height: 7.5rem;
		margin-bottom: 2rem;
		border-radius: 50%;
		object-fit: cover;
	}

	.since {
		margin-top: 1rem;
		color: var(--accent-soft);
	}

	.bio {
		max-width: 30rem;
		margin-top: 2rem;
		white-space: pre-line;
	}

	@include desktop {
		.cook {
			padding-top: 8.75rem;
		}
	}
</style>
