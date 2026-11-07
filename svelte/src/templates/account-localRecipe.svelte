<script>
	import { getGlobal } from 'crelte';
	import Button from '@/components/Button.svelte';
	import RecipeView from '@/components/recipe/RecipeView.svelte';
	import { getMock } from '@/lib/mock.svelte';
	import { getSession } from '@/lib/session.svelte';
	import { fromLocal } from '@/lib/recipe';

	// a recipe of the editor: it only exists in this browser
	const mock = getMock();
	const session = getSession();
	const categories = getGlobal('categories');

	let { entry } = $props();

	let local = $derived(mock.localRecipe(entry.recipeId));
	let recipe = $derived(
		local
			? fromLocal(local, session.user?.name ?? null, $categories)
			: null,
	);
</script>

{#if recipe}
	<p class="note small">
		Only you see this recipe: it is saved in this browser for now.
	</p>
	<RecipeView {recipe} />
{:else if mock.localLoaded}
	<div class="missing narrow">
		<h1 class="h1"><i>Recipe not found</i></h1>
		<p>It was saved in another browser or deleted.</p>
		<Button href="/account">My Recipes</Button>
	</div>
{/if}

<style lang="scss">
	.note {
		padding: 0.5rem var(--sx-body);
		background: var(--accent);
		color: var(--text-invert);
		text-align: center;
	}

	.missing {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2rem;
		padding-block: 10rem;
		text-align: center;
	}
</style>
