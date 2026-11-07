<script>
	import Stars from './Stars.svelte';

	// a recipe of the lists: image 3:4, title, category and cook, rating
	let { recipe, sizes = '(max-width: 767px) 100vw, 20rem' } = $props();

	let img = $derived(recipe.image?.[0] ?? null);
	let category = $derived(recipe.category?.[0] ?? null);
	let cook = $derived(recipe.cook?.[0] ?? null);
</script>

<article class="card">
	<a href={recipe.url} class="image" tabindex="-1" aria-hidden="true">
		{#if img}
			<img
				src={img.url}
				srcset={img.srcset}
				{sizes}
				width={img.width}
				height={img.height}
				alt={img.alt ?? ''}
				loading="lazy"
			/>
		{/if}
	</a>
	<h3 class="title"><a href={recipe.url}>{recipe.title}</a></h3>
	<p class="meta">
		{category?.singular ?? category?.title ?? 'Recipe'} by
		{#if cook}
			<a href={cook.url}>{cook.title}</a>
		{:else}
			Anonymous
		{/if}
	</p>
	<Stars rating={recipe.rating} />
</article>

<style lang="scss">
	.card {
		display: flex;
		flex-direction: column;
	}

	.image {
		display: block;
		margin-bottom: 1.5rem;
		overflow: hidden;
		aspect-ratio: 3 / 4;
		// the faded image blends with the page's color, nothing shows through
		background: var(--bg);
	}

	img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0.6;
		transition:
			transform 0.6s ease,
			opacity 0.3s ease;
	}

	.title {
		@include lead;
		max-width: 15em;
		text-wrap: balance;
	}

	.meta {
		margin: 0.25rem 0 1.5rem;
		color: var(--accent-soft);

		a {
			@include underline;
		}
	}

	@include hover {
		.card:hover img {
			opacity: 1;
			transform: scale(1.03);
		}

		.meta a:hover {
			color: var(--accent);
		}
	}
</style>
