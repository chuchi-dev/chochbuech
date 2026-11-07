<script lang="ts">
	import { RATINGS } from '@/lib/features';

	// ★★★★☆ like the design, the rating rounded to whole stars; hidden while
	// ratings aren't real (lib/features.ts)
	let { rating }: { rating: number | null } = $props();

	let full = $derived(Math.round(rating ?? 0));
</script>

{#if RATINGS && rating}
	<p class="stars" aria-label="Rated {rating} of 5">
		<span aria-hidden="true">{'★'.repeat(full)}{'☆'.repeat(5 - full)}</span>
	</p>
{/if}

<style lang="scss">
	.stars {
		font-size: 1.125rem;
		letter-spacing: 0.1em;
	}
</style>
