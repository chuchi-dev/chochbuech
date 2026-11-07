<script>
	import { formatTime } from '@/lib/recipe';
	import Stars from '../Stars.svelte';
	import Actions from './Actions.svelte';
	import Ingredients from './Ingredients.svelte';
	import Steps from './Steps.svelte';

	// a recipe page, of zipp or of the editor (see lib/recipe.ts for the
	// shape): who, title, rating, the actions, the image, the ingredients
	// and the steps
	let { recipe } = $props();

	let img = $derived(recipe.image);
	let times = $derived(
		[
			recipe.totalTime ? `${formatTime(recipe.totalTime)} in total` : '',
			recipe.activeTime ? `${formatTime(recipe.activeTime)} active` : '',
		]
			.filter(Boolean)
			.join(' · '),
	);
</script>

<svelte:head>
	<title>{recipe.title} | Chuchi Recipes</title>
</svelte:head>

<article class="recipe">
	<header class="wrap">
		<p class="by">
			Recipe by
			{#if recipe.cookUrl}
				<a href={recipe.cookUrl}>{recipe.cook}</a>
			{:else}
				{recipe.cook ?? 'Anonymous'}
			{/if}
		</p>
		<h1 class="h1"><i>{recipe.title}</i></h1>
		<Stars rating={recipe.rating} />
		{#if times}
			<p class="times small">{times}</p>
		{/if}
	</header>

	<div class="media wrap">
		<div class="actions">
			<Actions id={recipe.id} authorId={recipe.authorId} />
		</div>
		{#if img}
			<img
				src={img.url}
				srcset={img.srcset}
				sizes="(max-width: 1110px) 100vw, 92rem"
				width={img.width}
				height={img.height}
				alt={img.alt ?? ''}
				fetchpriority="high"
			/>
		{:else}
			<div class="placeholder"></div>
		{/if}
	</div>

	<div class="content wrap">
		<div class="side">
			<Ingredients serves={recipe.serves} chapters={recipe.chapters} />
		</div>
		<Steps chapters={recipe.chapters} />
	</div>
</article>

<style lang="scss">
	header {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		padding-block: 5rem 4rem;
		text-align: center;
	}

	.by a {
		@include underline;
	}

	h1 {
		margin-bottom: 1rem;
		text-wrap: balance;
	}

	.times {
		color: var(--accent-soft);
	}

	.media {
		position: relative;
	}

	.actions {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		z-index: 1;
		transform: translateY(-50%);
	}

	img,
	.placeholder {
		width: 100%;
		aspect-ratio: 3 / 4;
		object-fit: cover;
	}

	.placeholder {
		background: var(--line);
	}

	.content {
		display: flex;
		flex-direction: column;
		gap: 5rem;
		padding-block: 1.25rem var(--sy-section);
	}

	@include tablet {
		img,
		.placeholder {
			aspect-ratio: 3 / 2;
		}
	}

	@include desktop {
		header {
			padding-block: 8.75rem 8.5rem;
		}

		img,
		.placeholder {
			aspect-ratio: 92 / 60;
		}

		.content {
			display: grid;
			grid-template-columns: 30rem 1fr;
			gap: 4rem;
			padding-top: 8rem;
		}

		.side {
			align-self: start;
			position: sticky;
			top: calc(var(--topbar-height) + 2rem);
		}
	}
</style>
