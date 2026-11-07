<script>
	import RecipeCard from './RecipeCard.svelte';

	// the three columns of cards in the 1088px column of the design
	let { recipes, empty = 'No recipes found.' } = $props();
</script>

{#if recipes.length}
	<div class="grid">
		{#each recipes as recipe (recipe.id)}
			<RecipeCard {recipe} />
		{/each}
	</div>
{:else}
	<p class="empty">{empty}</p>
{/if}

<style lang="scss">
	.grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 3rem var(--card-gap);
	}

	.empty {
		color: var(--accent-soft);
		text-align: center;
	}

	@include tablet {
		.grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 4rem 2rem;
		}
	}

	@include desktop {
		.grid {
			grid-template-columns: repeat(3, 1fr);
			gap: 5rem var(--card-gap);
		}
	}
</style>
