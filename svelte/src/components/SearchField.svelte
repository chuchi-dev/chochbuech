<script>
	import Icon from './Icon.svelte';

	// the search of the hero and the list: a plain form, the list reads `q`
	let {
		action,
		value = '',
		autofocus = false,
		id = undefined,
		input = $bindable(),
	} = $props();
</script>

<form {action} role="search" class="search" {id}>
	<label class="sr-only" for="{id ?? 'search'}-q">Search for a Recipe</label>
	<!-- svelte-ignore a11y_autofocus -->
	<input
		bind:this={input}
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
	// the pill with the ring around it; the round button sits in it with
	// the same space at the top, the bottom and the right
	.search {
		position: relative;
		width: 100%;
		max-width: 33rem;
		padding: 0.5rem;
		border: 1px solid var(--line);
		border-radius: 999px;
		transition: border-color 0.3s ease;

		&:focus-within {
			border-color: var(--accent);
		}
	}

	input {
		display: block;
		width: 100%;
		height: 3.5rem;
		padding: 0 3.5rem 0 1.5rem;
		border: 1px solid var(--accent);
		border-radius: 999px;
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
		position: absolute;
		top: 1rem;
		right: 1rem;
		display: flex;
		width: 2.5rem;
		height: 2.5rem;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		background: var(--accent);
		color: var(--text-invert);
		font-size: 1rem;
		cursor: pointer;
	}

	@include desktop {
		input {
			height: 4rem;
			padding: 0 4.5rem 0 2rem;
		}

		button {
			width: 3rem;
			height: 3rem;
		}
	}
</style>
