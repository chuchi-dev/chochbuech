<script>
	// rich text of zipp as html, styled by sass/rt.scss
	let { html, class: klass = '' } = $props();
</script>

{#if html}
	<div class="rt {klass}">{@html html}</div>
{/if}
