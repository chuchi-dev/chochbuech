<script>
	import { getGlobal } from 'crelte';
	import Columns from '@/components/home/Columns.svelte';
	import Feature from '@/components/home/Feature.svelte';
	import Hero from '@/components/home/Hero.svelte';
	import RecipeRow from '@/components/home/RecipeRow.svelte';

	const recipesPage = getGlobal('recipesPage');

	let { entry } = $props();

	const BLOCKS = { recipeRow: RecipeRow, feature: Feature, columns: Columns };
</script>

<Hero
	title={entry.heroTitle}
	images={entry.heroImages}
	searchUrl={$recipesPage?.url ?? '/recipes'}
/>

{#each entry.content as block, i (i)}
	{@const Block = BLOCKS[block.typeHandle]}
	{#if Block}
		<Block {...block} />
	{/if}
{/each}
