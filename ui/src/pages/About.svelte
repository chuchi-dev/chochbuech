<script>
	import GetNotified from '@/components/GetNotified.svelte';
</script>

<svelte:head>
	<title>Was ist Chochbuech?</title>
	<meta
		name="description"
		content="Chochbuech ist eine Schweizer Rezepte-Plattform. Entdecke neue Rezepte, teile deine eigenen Kreationen und lasse dich von der Community inspirieren."
	/>
</svelte:head>

<div class="intro wrap">
	<div class="inner">
		<h1>
			Was ist <i>Chochbuech</i>
			?
		</h1>
		<p>Melde dich an, um benachrichtigt zu werden, sobald es losgeht.</p>

		<GetNotified />
	</div>
</div>

<div class="wrap">
	<div class="rt">
		<p>
			Chochbuech bedeutet Kochbuch auf Schweizerdeutsch – ein Name, der
			den Charme der Schweizer Küche und Sprache widerspiegelt.
		</p>
		<p>
			Diese Plattform ist für alle gedacht, die das Kochen lieben, sei es
			als Hobby oder Leidenschaft. Entdecke eine Vielfalt an Rezepten, von
			traditionellen Schweizer Klassikern bis hin zu modernen Kreationen
			aus aller Welt.
		</p>
		<p>
			Teile deine eigenen Lieblingsrezepte mit der Community und lasse
			dich inspirieren, Neues auszuprobieren. Egal, ob du ein Anfänger in
			der Küche bist oder ein erfahrener Koch – im Chochbuech ist für
			jeden etwas dabei.
		</p>
	</div>
</div>

<style lang="scss">
	.intro {
		display: flex;
		padding-top: 10rem;
		padding-bottom: 10rem;
		justify-content: center;
		align-items: center;
	}

	.intro p {
		margin-bottom: 2rem;
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
</style>
