<script>
	// the big running line behind the recipe rows: "trending recipes"
	// over and over, every second one in italics
	let { text } = $props();
</script>

<div class="marquee" aria-hidden="true">
	<div class="track">
		{#each [0, 1] as half (half)}
			<span class="half">
				{#each [0, 1, 2] as i (i)}
					<span class:italic={i % 2 === 0}>{text}</span>
				{/each}
			</span>
		{/each}
	</div>
</div>

<style lang="scss">
	.marquee {
		overflow: hidden;
		white-space: nowrap;
		pointer-events: none;
	}

	.track {
		display: flex;
		width: max-content;
		animation: run 40s linear infinite;
	}

	.half {
		display: flex;
	}

	span span {
		@include display;
		padding-right: 0.3em;
		line-height: 1;
	}

	.italic {
		font-style: italic;
	}

	@keyframes run {
		to {
			transform: translateX(-50%);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.track {
			animation: none;
		}
	}
</style>
