<script>
	import Richtext from '@/components/Richtext.svelte';

	// a text page: contact, imprint, privacy
	let { entry } = $props();
</script>

<article class="page narrow">
	<h1 class="h1"><i>{entry.title}</i></h1>
	<Richtext html={entry.text?.html} class="lead text" />
</article>

<style lang="scss">
	.page {
		max-width: calc(44rem + 2 * var(--sx-body));
		padding-block: 5rem var(--sy-section);
	}

	h1 {
		margin-bottom: 3rem;
		text-align: center;
	}

	@include desktop {
		.page {
			padding-top: 8.75rem;
		}
	}
</style>
