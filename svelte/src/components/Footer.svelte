<script>
	import { getGlobal } from 'crelte';
	import Icon from './Icon.svelte';

	const footer = getGlobal('footer');
</script>

<footer>
	<p>© {$footer?.copyright ?? 'Chuchi Recipes'}</p>
	<p class="made small">
		Made with <span class="heart"><Icon name="heart" filled /></span>
		using
		<a href="https://github.com/chuchi-dev" target="_blank" rel="noopener">
			Chuchi
		</a>
	</p>
	<nav aria-label="Footer">
		{#each $footer?.links ?? [] as link (link.url)}
			<a href={link.url} target={link.target}>
				{link.label ?? link.defaultLabel}
			</a>
		{/each}
	</nav>
</footer>

<style lang="scss">
	footer {
		display: grid;
		grid-template-columns: auto 1fr;
		align-items: center;
		border-top: 1px solid var(--line);
	}

	footer > * {
		padding: 1.5rem var(--sx-body);
	}

	// on a phone: copyright and links in a row, the credit below
	.made {
		grid-column: 1 / -1;
		grid-row: 2;
		justify-content: center;
		border-top: 1px solid var(--line);
	}

	nav {
		justify-content: flex-end;
		padding-left: 0;
	}

	.made {
		display: flex;
		align-items: center;
		gap: 0.3em;
		color: var(--accent-soft);

		a {
			@include underline;
		}
	}

	.heart {
		display: inline-flex;
	}

	nav,
	footer > p:first-child {
		font-size: var(--small);
	}

	nav {
		display: flex;
		gap: 1rem;
	}

	@include tablet {
		footer {
			grid-template-columns: 1fr auto 1fr;
		}

		.made {
			grid-column: 2;
			grid-row: 1;
			padding-inline: 0;
			border-top: 0;
		}

		nav,
		footer > p:first-child {
			font-size: inherit;
		}

		nav {
			gap: 1.5rem;
		}
	}

	@include hover {
		nav a:hover {
			@include underline;
		}
	}
</style>
