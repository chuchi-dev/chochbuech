<script>
	import Join from '@/components/Join.svelte';

	// where Submit Recipe leads a guest: no sign up yet, accounts are by
	// invitation
	let { entry } = $props();
</script>

<svelte:head>
	<title>{entry.title} | Chuchi Recipes</title>
</svelte:head>

<Join title={entry.title} signIn="/sign-in?next=/recipes/new" />
