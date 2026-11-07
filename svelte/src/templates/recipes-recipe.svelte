<script>
	import RecipeView from '@/components/recipe/RecipeView.svelte';
	import { fromZipp } from '@/lib/recipe';

	let { entry } = $props();

	let recipe = $derived(fromZipp(entry));
</script>

<RecipeView {recipe} />
