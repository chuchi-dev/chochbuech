<script>
	import Marquee from '../Marquee.svelte';
	import RecipeCard from '../RecipeCard.svelte';
	import RoundButton from '../RoundButton.svelte';

	// recipes under a running line; as a slider they scroll sideways out
	// of the column to the right edge, the controls only when they don't
	// all fit. Without a live recipe there is no row.
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

	// mouse drag; touch scrolls natively. While dragging the snapping is off,
	// on release the track glides to the card nearest to where the throw ends
	/** @type {{ x: number, left: number, moved: boolean, t: number, v: number } | null} */
	let drag = null;
	let dragging = $state(false);
	let dragged = false;

	/** @param {PointerEvent} e */
	function down(e) {
		if (!track || e.pointerType !== 'mouse' || e.button !== 0) return;
		if (track.scrollWidth <= track.clientWidth) return;
		drag = {
			x: e.clientX,
			left: track.scrollLeft,
			moved: false,
			t: e.timeStamp,
			v: 0,
		};
		dragged = false;
	}

	/** @param {PointerEvent} e */
	function move(e) {
		if (!track || !drag) return;
		const dx = e.clientX - drag.x;
		if (!drag.moved) {
			if (Math.abs(dx) < 5) return;
			drag.moved = true;
			dragging = true;
			track.setPointerCapture(e.pointerId);
		}
		const left = drag.left - dx;
		const dt = Math.max(e.timeStamp - drag.t, 1);
		// px per ms of the scroll, smoothed
		drag.v = 0.8 * ((left - track.scrollLeft) / dt) + 0.2 * drag.v;
		drag.t = e.timeStamp;
		track.scrollLeft = left;
	}

	/** @param {PointerEvent} e */
	function up(e) {
		if (!track || !drag) return;
		const { moved, v, t } = drag;
		drag = null;
		if (!moved) return;
		dragged = true;
		// held still before letting go: no throw
		const throwV = e.timeStamp - t > 100 ? 0 : v;
		const target = track.scrollLeft + throwV * 300;
		// the first card sits at the scroll padding
		const cards = [...track.querySelectorAll('article')];
		const pad = cards[0]?.offsetLeft ?? 0;
		const max = track.scrollWidth - track.clientWidth;
		const stops = cards.map(a => Math.min(a.offsetLeft - pad, max));
		const left = stops.reduce(
			(a, b) => (Math.abs(b - target) < Math.abs(a - target) ? b : a),
			stops[0] ?? 0,
		);
		track.scrollTo({ left, behavior: 'smooth' });
		// snapping back on only after the glide, else it would jump
		const el = track;
		const done = () => {
			dragging = false;
			el.removeEventListener('scrollend', done);
		};
		el.addEventListener('scrollend', done);
		setTimeout(done, 800);
	}

	/** @param {MouseEvent} e */
	function click(e) {
		// a drag that ends on a card is no click on it
		if (!dragged) return;
		dragged = false;
		e.preventDefault();
		e.stopPropagation();
	}

	$effect(update);
</script>

<svelte:window onresize={update} />

{#if recipes?.length}
	<section class="row" class:slider>
		{#if marquee}
			<Marquee text={marquee} />
		{/if}
		<div
			class="track"
			class:scrolls={visible < 1}
			class:dragging
			bind:this={track}
			onscroll={update}
			onpointerdown={down}
			onpointermove={move}
			onpointerup={up}
			onpointercancel={up}
			onclickcapture={click}
			ondragstart={e => e.preventDefault()}
			role="group"
			aria-label="Recipes"
		>
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
						ring={false}
						onclick={() => scroll(-1)}
					/>
					<RoundButton
						icon="right"
						label="Next"
						filled={false}
						size="sm"
						ring={false}
						onclick={() => scroll(1)}
					/>
				</div>
			</div>
		{/if}
	</section>
{/if}

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

	.slider .track.scrolls {
		cursor: grab;
	}

	.track.dragging {
		scroll-snap-type: none;
		cursor: grabbing;
		user-select: none;
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
