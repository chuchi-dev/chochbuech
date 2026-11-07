<script lang="ts">
	// ★★★★☆ like the design, the rating rounded to whole stars
	let { rating }: { rating: number | null } = $props();

	let full = $derived(Math.round(rating ?? 0));
</script>

{#if rating}
	<p class="stars" aria-label="Rated {rating} of 5">
		<span aria-hidden="true">{'★'.repeat(full)}{'☆'.repeat(5 - full)}</span>
	</p>
{/if}

<style lang="scss">
	.stars {
		font-size: 0.8125rem;
		letter-spacing: 0.1em;
	}
</style>
