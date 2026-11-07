<script>
	import Richtext from '../Richtext.svelte';

	// two columns of text, the right one starts lower like in the design
	let { left, right } = $props();
</script>

<section class="columns wrap">
	<Richtext html={left?.html} class="lead col" />
	<Richtext html={right?.html} class="lead col" />
</section>

<style lang="scss">
	.columns {
		display: grid;
		gap: 1.5em;
		max-width: calc(82rem + 2 * var(--sx-body));
		padding-block: var(--sy-section);
		border-top: 1px solid var(--line);
	}

	@include desktop {
		.columns {
			grid-template-columns: 1fr 1fr;
			gap: 2rem;

			:global(.col:last-child) {
				margin-top: 17rem;
			}
		}
	}
</style>
