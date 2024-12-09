<script>
	import GetNotified from '@/components/GetNotified.svelte';
</script>

<svelte:head>
	<title>Neues Rezept erstellen</title>
</svelte:head>

<div class="intro wrap">
	<div class="inner">
		<h1>Das ist leider noch nicht ganz fertig</h1>
		<p>Melde dich an, um benachrichtigt zu werden, sobald es losgeht.</p>

		<GetNotified />
	</div>
</div>

<style lang="scss">
	.intro {
		display: flex;
		height: 100%;
		justify-content: center;
		align-items: center;
	}

	.inner {
		text-align: center;
	}

	h1 {
		font-family: var(--font-secondary);
		font-weight: 200;
		line-height: 1.25;
		font-size: 3rem;
		margin-bottom: 1rem;
	}

	p {
		margin-bottom: 2rem;
		/* font-size: 1.25rem;
		max-width: 30rem; */
	}
</style>
