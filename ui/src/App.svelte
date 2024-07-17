<script>
	import './app.scss';

	export let component;
	export let props;
</script>

<main>
	<svelte:component this={component} {...props} />
</main>
