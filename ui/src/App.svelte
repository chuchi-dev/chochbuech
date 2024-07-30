<script>
	import './app.scss';

	const {
		/** @type {any} */
		page,
	} = $props();
</script>

<main>
	<svelte:component this={$page.component} {...$page.props} />
</main>
