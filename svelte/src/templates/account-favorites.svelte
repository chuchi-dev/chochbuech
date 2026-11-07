<script module>
	import recipesByIdQuery from '@/queries/recipesById.graphql';

	// the favorites cookie, so the server renders them
	/** @type {import('crelte').LoadData} */
	export const loadData = async cr => {
		/** @type {string[]} */
		let ids = [];
		try {
			ids = JSON.parse(cr.cookies.get('chochbuech-favorites') ?? '[]');
		} catch {
			// none
		}
		if (!Array.isArray(ids) || !ids.length) return { recipes: [] };
		return cr.query(recipesByIdQuery, { ids });
	};
</script>

<script>
	import { getGlobal } from 'crelte';
	import AccountLayout from '@/components/AccountLayout.svelte';
	import Pills from '@/components/Pills.svelte';
	import RecipeGrid from '@/components/RecipeGrid.svelte';
	import { categoryPills } from '@/lib/lists.js';
	import { getMock } from '@/lib/mock.svelte';

	let { entry, recipes } = $props();

	const mock = getMock();
	const categories = getGlobal('categories');
	const recipesPage = getGlobal('recipesPage');

	let active = $state('');

	// the newest first, unfavorited ones go right away
	let favorites = $derived(
		mock.favorites
			.map(id => recipes.find(r => r.id === id))
			.filter(Boolean)
			.reverse(),
	);
	// the pills: the categories of the favorites
	let pills = $derived(
		categoryPills(
			($categories ?? []).filter(c =>
				favorites.some(r => r.category?.[0]?.id === c.id),
			),
		),
	);
	// All again once the last favorite of the category is gone
	let selected = $derived(pills.some(p => p.key === active) ? active : '');
	let shown = $derived(
		selected
			? favorites.filter(r => r.category?.[0]?.slug === selected)
			: favorites,
	);
</script>

<AccountLayout title={entry.title} note="Saved in this browser." guests>
	{#snippet intro()}
		{#if favorites.length}
			<Pills
				items={pills}
				active={selected}
				label="Categories"
				onselect={key => (active = key)}
			/>
		{/if}
	{/snippet}

	<div class="narrow">
		{#if favorites.length}
			<RecipeGrid
				recipes={shown}
				empty="No favorites in this category."
			/>
		{:else}
			<p class="empty">
				No favorites yet. Tap the heart on a recipe.
				<a href={$recipesPage?.url ?? '/recipes'}>Find recipes</a>
			</p>
		{/if}
	</div>
</AccountLayout>

<style lang="scss">
	.empty {
		color: var(--accent-soft);
		text-align: center;

		a {
			@include underline;
			display: block;
			margin-top: 1rem;
			color: var(--accent);
		}
	}

	@include hover {
		.empty a:hover {
			opacity: 0.6;
		}
	}
</style>
