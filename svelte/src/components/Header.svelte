<script>
	import { getGlobal, getRoute, getSite, onRoute } from 'crelte';
	import { getMock } from '@/lib/mock.svelte';
	import { getSession } from '@/lib/session.svelte';
	import { liveOnly } from '@/lib/lists.js';
	import Button from './Button.svelte';

	const header = getGlobal('header');
	const recipesPage = getGlobal('recipesPage');
	const liveCategories = getGlobal('liveCategories');
	const site = getSite();
	const route = getRoute();
	const session = getSession();
	const mock = getMock();

	let open = $state(false);

	let listUrl = $derived($recipesPage?.url ?? '/recipes');
	// only the categories with a live recipe
	let categories = $derived(
		liveOnly($header?.categories, $liveCategories).map(c => ({
			...c,
			url: `${listUrl}?category=${c.slug}`,
		})),
	);
	let active = $derived($route.getSearchParam('category'));
	// a guest's favorites and planner are kept by the browser, without the
	// account
	let guestFavorites = $derived(!session.user && mock.favorites.length > 0);
	// guests can't sign up, they get the invitation of /join
	let submitUrl = $derived(session.user ? '/recipes/new' : '/join');

	onRoute(() => (open = false));
</script>

<svelte:window onkeydown={e => e.key === 'Escape' && (open = false)} />

<header class:open>
	<nav class="side left" aria-label="Categories">
		{#each categories as c (c.id)}
			<a href={c.url} class:active={active === c.slug}>{c.title}</a>
		{/each}
	</nav>

	<a class="logo" href={$site.url.href}>Chuchi Recipes</a>

	<nav class="side right" aria-label="Account">
		<a href={listUrl} data-focus-search>Search</a>
		{#if guestFavorites}
			<a href="/account/favorites">Favorites</a>
		{/if}
		{#if session.user}
			<a href="/account">{session.user.name || 'My Account'}</a>
		{:else}
			<a href="/sign-in">Sign In</a>
		{/if}
		<Button href={submitUrl} size="sm">Submit Recipe</Button>
	</nav>

	<div class="toggle">
		<Button
			look="secondary"
			size="sm"
			aria-expanded={open}
			aria-controls="menu"
			onclick={() => (open = !open)}
		>
			{open ? 'Close' : 'Menu'}
		</Button>
	</div>
</header>

{#if open}
	<nav id="menu" class="menu" aria-label="Menu">
		<div class="links">
			{#each categories as c (c.id)}
				<a href={c.url}>{c.title}</a>
			{/each}
			<a href={listUrl} data-focus-search>Search</a>
			{#if guestFavorites}
				<a href="/account/favorites">Favorites</a>
			{/if}
		</div>
		<div class="buttons">
			{#if session.user}
				<Button href="/account" look="secondary">
					{session.user.name || 'My Account'}
				</Button>
			{:else}
				<Button href="/sign-in" look="secondary">Sign in</Button>
			{/if}
			<Button href={submitUrl}>Submit Recipe</Button>
		</div>
	</nav>
{/if}

<style lang="scss">
	header {
		position: sticky;
		top: 0;
		z-index: 100;
		display: flex;
		height: var(--topbar-height);
		align-items: center;
		justify-content: space-between;
		padding-inline: var(--sx-body);
		border-bottom: 1px solid var(--line);
		background: var(--bg);
	}

	.logo {
		@include logo;
		white-space: nowrap;
	}

	.side {
		display: none;
	}

	.side a:not(:global(.btn)) {
		transition: opacity 0.3s ease;
	}

	// the design's bold italic serif; PP Editorial Old has no bold italic,
	// the browser makes it bolder
	.side .active {
		font-family: var(--font-serif);
		font-style: italic;
		font-weight: 700;
		font-size: 1.125rem;
		line-height: 1;
	}

	.menu {
		position: fixed;
		inset: var(--topbar-height) 0 0 0;
		z-index: 105;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 4rem;
		padding: var(--sx-body);
		background: var(--bg);
	}

	.links {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;

		a {
			@include h2;
			font-style: italic;
		}
	}

	.buttons {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
	}

	// the full header as soon as it fits, the menu below
	@media (min-width: $bpHeader) {
		.logo {
			position: absolute;
			left: 50%;
			transform: translateX(-50%);
		}

		.side {
			display: flex;
			align-items: center;
			gap: 2rem;
		}

		.toggle,
		.menu {
			display: none;
		}
	}

	@include hover {
		.side a:not(:global(.btn)):hover {
			opacity: 0.6;
		}
	}
</style>
