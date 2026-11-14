<script>
	// "There's too much sugar in this cake." with the hand written
	// corrections of the design over it; the words are fixed, the editor
	// turns them on with the feature's "doodles"
	let { html } = $props();
</script>

<span class="sentence">
	<span class="sr-only">{@html html}</span>
	<span aria-hidden="true">
		There’s <span class="crossed">
			too much
			<span class="note above">not enough</span>
			<svg
				class="scribble"
				viewBox="0 0 200 20"
				preserveAspectRatio="none"
			>
				<path
					d="M2 16 L8 4 L14 16 L20 4 L26 16 L32 4 L38 16 L44 4 L50 16 L56 4 L62 16 L68 4 L74 16 L80 4 L86 16 L92 4 L98 16 L104 4 L110 16 L116 4 L122 16 L128 4 L134 16 L140 4 L146 16 L152 4 L158 16 L164 4 L170 16 L176 4 L182 16 L188 4 L194 16"
				/>
			</svg>
		</span>
		sugar
		<span class="note arrow">
			↑
			<br />
			250g
		</span>
		in this
		<span class="note side">(chocolate)</span>
		cake.
	</span>
</span>

<style lang="scss">
	.sentence {
		position: relative;
		display: inline-block;
	}

	.crossed {
		position: relative;
		display: inline-block;
	}

	.note {
		position: absolute;
		color: var(--doodle);
		font-family: var(--font-hand);
		font-weight: 600;
		font-size: 0.5em;
		font-style: normal;
		line-height: 1;
		letter-spacing: -0.03em;
		white-space: nowrap;
		pointer-events: none;
	}

	.above {
		top: -0.45em;
		left: 0.6em;
		transform: rotate(-5deg);
	}

	.scribble {
		position: absolute;
		top: 0.5em;
		left: -0.1em;
		width: calc(100% + 0.2em);
		height: 0.3em;
		fill: none;
		stroke: var(--doodle);
		stroke-width: 1.5px;
		vector-effect: non-scaling-stroke;
		transform: rotate(4deg);
		pointer-events: none;

		path {
			vector-effect: non-scaling-stroke;
		}
	}

	.arrow {
		position: relative;
		display: inline-block;
		width: 0;
		top: 1.3em;
		left: -1.6em;
		text-align: center;
	}

	.side {
		position: relative;
		display: inline-block;
		width: 0;
		top: 0.2em;
		left: -0.4em;
		transform: rotate(-70deg);
		transform-origin: 0 0;
	}
</style>
