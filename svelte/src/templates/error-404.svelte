<script>
	import Button from '@/components/Button.svelte';
</script>

<svelte:head>
	<title>Not found | Chuchi Recipes</title>
</svelte:head>

<div class="notfound narrow">
	<h1 class="h1"><i>Nothing cooking here</i></h1>
	<p class="lead">This page does not exist (anymore).</p>
	<Button href="/">Back to the recipes</Button>
</div>

<style lang="scss">
	.notfound {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2rem;
		padding-block: 10rem;
		text-align: center;
	}
</style>
