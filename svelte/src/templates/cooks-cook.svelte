<script module>
	import recipesQuery from '@/queries/recipes.graphql';

	/** @type {import('crelte').LoadData} */
	export const loadData = (cr, entry) =>
		cr.query(recipesQuery, { cook: entry.id, limit: 100, offset: 0 });
</script>

<script>
	import RecipeGrid from '@/components/RecipeGrid.svelte';

	// the public page of a cook: who, since when and their recipes
	let { entry, recipes } = $props();
</script>

<div class="cook narrow">
	<header>
		<h1 class="h1"><i>{entry.title}</i></h1>
		{#if entry.memberSince}
			<p class="since">Community Member since {entry.memberSince}</p>
		{/if}
		{#if entry.bio}
			<p class="bio lead">{entry.bio}</p>
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

	.since {
		margin-top: 1rem;
		color: var(--accent-soft);
	}

	.bio {
		max-width: 30rem;
		margin-top: 2rem;
	}

	@include desktop {
		.cook {
			padding-top: 8.75rem;
		}
	}
</style>
