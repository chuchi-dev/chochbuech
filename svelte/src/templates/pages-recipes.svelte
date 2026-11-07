<script module>
	import recipesQuery from '@/queries/recipes.graphql';
	import { PAGE, categoryId } from '@/lib/lists.js';

	// the list with the search `?q=` and the filter `?category=<slug>`
	/** @type {import('crelte').LoadData} */
	export const loadData = cr => {
		const category = categoryId(
			cr.getGlobal('categories'),
			cr.req.getSearchParam('category'),
		);
		return cr.query(recipesQuery, {
			search: cr.req.getSearchParam('q') || null,
			category,
			limit: PAGE,
			offset: 0,
		});
	};
</script>

<script>
	import { getCrelte, getGlobal, getRoute } from 'crelte';
	import { urlWithSearch } from 'crelte/std/url';
	import Button from '@/components/Button.svelte';
	import Pills from '@/components/Pills.svelte';
	import RecipeGrid from '@/components/RecipeGrid.svelte';
	import SearchField from '@/components/SearchField.svelte';
	import { categoryPills } from '@/lib/lists.js';

	const crelte = getCrelte();
	const route = getRoute();
	const categories = getGlobal('categories');

	let { entry, recipes: first, total } = $props();

	let more = $state([]);
	let loading = $state(false);

	let recipes = $derived([...first, ...more]);
	let q = $derived($route.getSearchParam('q') ?? '');
	let active = $derived($route.getSearchParam('category') ?? '');
	let pills = $derived(
		categoryPills($categories, slug =>
			urlWithSearch(entry.url, { category: slug, q: q || null }),
		),
	);

	async function loadMore() {
		loading = true;
		/** @type {any} */
		const res = await crelte.query(recipesQuery, {
			search: q || null,
			category: $categories?.find(c => c.slug === active)?.id ?? null,
			limit: PAGE,
			offset: recipes.length,
		});
		more = [...more, ...res.recipes];
		loading = false;
	}
</script>

<svelte:head>
	<title>{q ? `${q} – ` : ''}Recipes | Chuchi Recipes</title>
</svelte:head>

<div class="list narrow">
	<h1 class="sr-only">{entry.title}</h1>
	<div class="filters">
		<SearchField action={entry.url} value={q} id="search" />
		<Pills items={pills} {active} label="Categories" />
	</div>

	{#if q}
		<p class="found">
			{total}
			{total === 1 ? 'recipe' : 'recipes'} for
			<i>{q}</i>
		</p>
	{/if}

	<RecipeGrid {recipes} />

	{#if recipes.length < total}
		<div class="more">
			<Button onclick={loadMore} disabled={loading}>
				{loading ? 'Loading …' : 'Load More'}
			</Button>
		</div>
	{/if}
</div>

<style lang="scss">
	.list {
		padding-block: 5rem var(--sy-section);
	}

	.filters {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 3rem;
		margin-bottom: 5rem;
	}

	.found {
		margin: -2rem 0 3rem;
		color: var(--accent-soft);
		text-align: center;
	}

	.more {
		display: flex;
		justify-content: center;
		margin-top: 5rem;
	}

	@include desktop {
		.list {
			padding-top: 8.75rem;
		}

		.filters {
			margin-bottom: 7.5rem;
		}
	}
</style>
