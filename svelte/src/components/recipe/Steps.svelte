<script>
	// the chapters' steps: "Step 1" and its text, an image belongs to the
	// step above it. A chapter is { title, steps: [{ html, image }] }.
	let { chapters } = $props();

	let shown = $derived(
		chapters
			.map((c, i) => ({
				title: c.title || (i === 0 ? 'Instructions' : ''),
				steps: c.steps,
			}))
			.filter(c => c.steps.length),
	);
</script>

<div class="steps">
	{#each shown as chapter, i (i)}
		<section class="chapter">
			<h2>{chapter.title || 'Instructions'}</h2>
			<ol>
				{#each chapter.steps as step, j (j)}
					<li>
						<span class="label">Step {j + 1}</span>
						<div class="body">
							<p>{@html step.html}</p>
							{#each step.images ?? [] as img (img.url)}
								<img
									src={img.url}
									srcset={img.srcset}
									sizes="(max-width: 1110px) 100vw, 52rem"
									width={img.width}
									height={img.height}
									alt={img.alt ?? ''}
									loading="lazy"
								/>
							{/each}
						</div>
					</li>
				{/each}
			</ol>
		</section>
	{/each}
</div>

<style lang="scss">
	.chapter + .chapter {
		margin-top: 4rem;
	}

	h2 {
		padding-bottom: 1rem;
		border-bottom: 1px solid var(--line);
		font-size: var(--lead);
	}

	li {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
		padding-block: 1.5rem 3.5rem;

		& + li {
			border-top: 1px solid var(--line);
		}
	}

	.label {
		color: var(--accent-soft);
	}

	.body {
		display: flex;
		flex-direction: column;
		gap: 2rem;
		max-width: 48rem;
		font-size: var(--lead);
	}

	img {
		width: 100%;
		max-width: none;
	}

	@include desktop {
		.chapter + .chapter {
			margin-top: 6rem;
		}

		li {
			display: grid;
			grid-template-columns: 6rem 1fr;
			gap: 0;
			padding-block: 1.25rem 4.5rem;
		}

		.label {
			padding-top: 0.25rem;
			font-size: var(--small);
		}

		.body {
			max-width: none;

			p {
				max-width: 46rem;
			}
		}
	}
</style>
