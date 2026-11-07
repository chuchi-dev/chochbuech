<script>
	import Icon from './Icon.svelte';

	// the search of the hero and the list: a plain form, the list reads `q`
	let { action, value = '', autofocus = false, id = undefined } = $props();
</script>

<form {action} role="search" class="search" {id}>
	<label class="sr-only" for="{id ?? 'search'}-q">Search for a Recipe</label>
	<!-- svelte-ignore a11y_autofocus -->
	<input
		id="{id ?? 'search'}-q"
		type="search"
		name="q"
		{value}
		{autofocus}
		placeholder="Search for a Recipe …"
		autocomplete="off"
	/>
	<button type="submit" aria-label="Search"><Icon name="right" /></button>
</form>

<style lang="scss">
	.search {
		display: flex;
		width: 100%;
		max-width: 24.5rem;
		padding: 0.25rem;
		border: 1px solid var(--line);
		border-radius: 999px;
		transition: border-color 0.3s ease;

		&:focus-within {
			border-color: var(--accent);
		}
	}

	input {
		flex: 1;
		min-width: 0;
		padding: 0.625rem 0 0.625rem 1.5rem;
		border: 1px solid var(--accent);
		border-right: 0;
		border-radius: 999px 0 0 999px;
		background: transparent;
		color: var(--accent);
		font-size: var(--lead);
		outline: none;

		&::placeholder {
			color: var(--accent-soft);
		}

		&::-webkit-search-cancel-button {
			display: none;
		}
	}

	button {
		display: flex;
		align-items: center;
		padding: 0.25rem 0.25rem 0.25rem 0;
		border: 1px solid var(--accent);
		border-left: 0;
		border-radius: 0 999px 999px 0;
		cursor: pointer;

		:global(svg) {
			box-sizing: content-box;
			padding: 0.6rem;
			border-radius: 50%;
			background: var(--accent);
			color: var(--text-invert);
		}
	}
</style>
