<script>
	import Button from '../Button.svelte';
	import Richtext from '../Richtext.svelte';
	import Doodles from './Doodles.svelte';

	// a title, a text and a button; blue as a full band
	let { title, text, link, look, doodles } = $props();
</script>

<section class="feature look-{look}" class:doodles>
	<div class="inner narrow">
		<h2 class="title">
			{#if doodles}
				<Doodles html={title?.html} />
			{:else}
				<Richtext html={title?.html} />
			{/if}
		</h2>
		<div class="body">
			<Richtext html={text?.html} class="lead" />
			{#if link?.url}
				<div class="button">
					<Button
						href={link.url}
						target={link.target}
						look={look === 'blue' ? 'invert' : 'primary'}
					>
						{link.label ?? link.defaultLabel}
					</Button>
				</div>
			{/if}
		</div>
	</div>
</section>

<style lang="scss">
	.feature {
		padding-block: var(--sy-section);
	}

	.inner {
		display: grid;
		gap: 2rem;
	}

	.title {
		@include h2;

		:global(p + p) {
			margin: 0;
		}
	}

	.button {
		margin-top: 2.5rem;
	}

	.look-blue {
		background: var(--accent);
		color: var(--text-invert);
		--line: rgba(246, 246, 244, 0.4);
		--focus-color: var(--bg);
	}

	@include desktop {
		.inner {
			grid-template-columns: 24rem 1fr;
			gap: 2.5rem 0;
		}

		// plain: title and text in the second column
		.look-plain:not(.doodles) .title {
			grid-column: 2;
			max-width: 50rem;
		}

		.look-blue .title {
			grid-column: 1 / -1;
			max-width: 50rem;
		}

		.doodles .title {
			grid-column: 1 / -1;
			white-space: nowrap;
		}

		.body {
			grid-column: 2;
			max-width: 44rem;
		}

		.look-blue {
			padding-block: 10rem 6rem;

			.button {
				margin-top: 8rem;
			}
		}
	}
</style>
