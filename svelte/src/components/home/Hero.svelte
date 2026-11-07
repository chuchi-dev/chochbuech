<script>
	import Richtext from '../Richtext.svelte';
	import SearchField from '../SearchField.svelte';

	// the title with the search, up to three photos around it, behind it
	let { title, images, searchUrl } = $props();

	// the page's scroll, the photos move by a factor of it (parallax)
	let scroll = $state(0);
	let frame = 0;
	/** @type {MediaQueryList | null} */
	let reduced = null;

	function onscroll() {
		if (frame || reduced?.matches) return;
		frame = requestAnimationFrame(() => {
			frame = 0;
			// past the hero the photos are out of sight
			scroll = Math.min(window.scrollY, window.innerHeight * 1.5);
		});
	}

	$effect(() => {
		reduced = matchMedia('(prefers-reduced-motion: reduce)');
		onscroll();
		return () => cancelAnimationFrame(frame);
	});
</script>

<svelte:window {onscroll} />

<section class="hero" style="--scroll: {scroll}">
	{#each images.slice(0, 1) as img (img.url)}
		<div class="photo photo-0">
			<img
				src={img.url}
				width={img.width}
				height={img.height}
				alt=""
				fetchpriority="high"
			/>
		</div>
	{/each}
	<div class="center">
		<h1><Richtext html={title?.html} class="title" /></h1>
		<SearchField action={searchUrl} />
	</div>
	{#each images.slice(1, 3) as img, i (img.url)}
		<div class="photo photo-{i + 1}">
			<img src={img.url} width={img.width} height={img.height} alt="" />
		</div>
	{/each}
</section>

<style lang="scss">
	.hero {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 2rem var(--sx-body) 4rem;
	}

	.center {
		position: relative;
		z-index: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2.5rem;
		text-align: center;
	}

	h1 :global(.title) {
		@include h1;
		text-wrap: balance;
	}

	h1 :global(p + p) {
		margin: 0;
	}

	// faded on the page's color, never see-through
	.photo {
		background: var(--bg);
		transform: translate3d(0, calc(var(--scroll) * var(--speed) * 1px), 0);
		will-change: transform;

		img {
			display: block;
			width: 100%;
			height: 100%;
			object-fit: cover;
			opacity: 0.6;
			transition: opacity 0.3s ease;
		}
	}

	.photo-0 {
		--speed: -0.1;
		width: 12rem;
		aspect-ratio: 3 / 2;
		align-self: flex-start;
		margin: -2rem 0 4rem calc(-1 * var(--sx-body));
	}

	.photo-1 {
		--speed: -0.3;
		width: 8rem;
		aspect-ratio: 2 / 3;
		margin: -0.5rem 0 0 -1.5rem;
	}

	.photo-2 {
		--speed: -0.2;
		display: none;
	}

	// the design's composition at 1600px, scaled down with the viewport
	@media (min-width: $bpHeader) {
		.hero {
			min-height: min(57.5rem, 57.5vw);
			justify-content: center;
			padding: 0 var(--sx-body);
		}

		.center {
			margin-top: max(-6rem, -6vw);
		}

		h1 :global(.title) {
			font-size: min(var(--h1), 5vw);
		}

		.photo {
			position: absolute;
			display: block;
			margin: 0;
			aspect-ratio: auto;
		}

		.photo-0 {
			top: 0;
			left: 0;
			width: min(30rem, 30vw);
			height: min(20rem, 20vw);
		}

		.photo-1 {
			top: min(27.5rem, 27.5vw);
			left: min(12rem, 12vw);
			width: min(20rem, 20vw);
			height: min(30rem, 30vw);
		}

		.photo-2 {
			top: min(13.75rem, 13.75vw);
			right: 0;
			width: min(20rem, 20vw);
			height: min(30rem, 30vw);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.photo {
			transform: none;
		}
	}

	@include hover {
		.photo:hover img {
			opacity: 1;
		}
	}
</style>
