<script>
	import Richtext from '../Richtext.svelte';
	import SearchField from '../SearchField.svelte';

	// the title with the search, up to three photos around it
	let { title, images, searchUrl } = $props();
</script>

<section class="hero">
	{#each images.slice(0, 1) as img (img.url)}
		<img
			class="photo photo-0"
			src={img.url}
			width={img.width}
			height={img.height}
			alt=""
			fetchpriority="high"
		/>
	{/each}
	<div class="center">
		<h1><Richtext html={title?.html} class="title" /></h1>
		<SearchField action={searchUrl} />
	</div>
	{#each images.slice(1, 3) as img, i (img.url)}
		<img
			class="photo photo-{i + 1}"
			src={img.url}
			width={img.width}
			height={img.height}
			alt=""
		/>
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

	.photo {
		object-fit: cover;
	}

	.photo-0 {
		width: 12rem;
		aspect-ratio: 3 / 2;
		align-self: flex-start;
		margin: -2rem 0 4rem calc(-1 * var(--sx-body));
	}

	.photo-1 {
		width: 8rem;
		aspect-ratio: 2 / 3;
		margin: -0.5rem 0 0 -1.5rem;
	}

	.photo-2 {
		display: none;
	}

	@include desktop {
		.hero {
			min-height: 57.5rem;
			justify-content: center;
			padding: 0 var(--sx-body);
		}

		.center {
			margin-top: -6rem;
		}

		.photo {
			position: absolute;
			display: block;
		}

		.photo-0 {
			top: 0;
			left: 0;
			width: 30rem;
			height: 20rem;
			margin: 0;
			aspect-ratio: auto;
		}

		.photo-1 {
			top: 27.5rem;
			left: 12rem;
			width: 20rem;
			height: 30rem;
			margin: 0;
			aspect-ratio: auto;
		}

		.photo-2 {
			top: 13.75rem;
			right: 0;
			width: 20rem;
			height: 30rem;
		}
	}
</style>
