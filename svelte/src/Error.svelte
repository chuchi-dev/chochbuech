<script>
	let { status, message } = $props();
</script>

<svelte:head>
	<title>Error</title>
</svelte:head>

<div class="error">
	<h1 class="h1"><i>Oh no</i></h1>
	<p>
		{status === 404
			? 'This page does not exist.'
			: message || 'An error occurred.'}
	</p>
	<a href="/">Back to the recipes</a>
</div>

<style lang="scss">
	.error {
		display: flex;
		min-height: 100vh;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		color: #3c67c0;
		font-family: sans-serif;
	}

	a {
		text-decoration: underline;
	}
</style>
