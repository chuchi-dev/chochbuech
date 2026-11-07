<script>
	import Marquee from '../Marquee.svelte';
	import RecipeCard from '../RecipeCard.svelte';
	import RoundButton from '../RoundButton.svelte';

	// recipes under a running line; as a slider they scroll sideways out
	// of the column to the right edge
	let { marquee, recipes, slider } = $props();

	/** @type {HTMLElement | undefined} */
	let track = $state();
	let progress = $state(0);
	let visible = $state(1);

	function update() {
		if (!track) return;
		const max = track.scrollWidth - track.clientWidth;
		progress = max > 0 ? track.scrollLeft / max : 0;
		visible = track.clientWidth / track.scrollWidth;
	}

	/** @param {number} dir */
	function scroll(dir) {
		const card = track?.querySelector('article');
		if (!track || !card) return;
		track.scrollBy({
			left: dir * (card.clientWidth + 64),
			behavior: 'smooth',
		});
	}

	$effect(update);
</script>

<svelte:window onresize={update} />

<section class="row" class:slider>
	{#if marquee}
		<Marquee text={marquee} />
	{/if}
	<div class="track" bind:this={track} onscroll={update}>
		{#each recipes as recipe (recipe.id)}
			<RecipeCard {recipe} sizes="20rem" />
		{/each}
	</div>
	{#if slider && visible < 1}
		<div class="controls">
			<div class="bar">
				<span
					style:width="{visible * 100}%"
					style:left="{progress * (1 - visible) * 100}%"
				></span>
			</div>
			<div class="arrows">
				<RoundButton
					icon="left"
					label="Back"
					filled={false}
					size="sm"
					onclick={() => scroll(-1)}
				/>
				<RoundButton
					icon="right"
					label="Next"
					filled={false}
					size="sm"
					onclick={() => scroll(1)}
				/>
			</div>
		</div>
	{/if}
</section>

<style lang="scss">
	.row {
		padding-block: var(--sy-section) 0;
	}

	.track {
		position: relative;
		display: grid;
		grid-auto-flow: column;
		grid-auto-columns: 75%;
		gap: 1.5rem;
		margin-top: calc(var(--display) * -0.45);
		padding-inline: var(--sx-body);
		overflow-x: auto;
		scroll-snap-type: x mandatory;
		scroll-padding-inline: var(--sx-body);
		scrollbar-width: none;

		&::-webkit-scrollbar {
			display: none;
		}

		:global(article) {
			scroll-snap-align: start;
		}
	}

	.controls {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		width: 15rem;
		margin: 3rem auto 0;
	}

	.bar {
		position: relative;
		height: 1px;
		background: var(--line);

		span {
			position: absolute;
			top: -1px;
			height: 3px;
			background: var(--accent);
			transition: left 0.2s ease;
		}
	}

	.arrows {
		display: flex;
		gap: 0;
	}

	@include tablet {
		.track {
			grid-auto-columns: calc((100% - 2 * var(--sx-body) - 2rem) / 2.3);
			gap: 2rem;
		}
	}

	@include desktop {
		.track {
			grid-auto-columns: 20rem;
			gap: var(--card-gap);
			padding-left: max(
				var(--sx-body),
				calc((100% - var(--narrow-width)) / 2)
			);
			scroll-padding-left: max(
				var(--sx-body),
				calc((100% - var(--narrow-width)) / 2)
			);
		}

		.row:not(.slider) .track {
			grid-auto-flow: row;
			grid-template-columns: repeat(3, 1fr);
			max-width: calc(var(--narrow-width) + 2 * var(--sx-body));
			margin-inline: auto;
			padding-inline: var(--sx-body);
			overflow: visible;
		}

		.controls {
			margin-left: calc(50% - var(--narrow-width) / 2 + 24rem);
		}
	}
</style>
