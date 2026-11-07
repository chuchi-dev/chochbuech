<script>
	import { getGlobal, getRoute, getSite, onRoute } from 'crelte';
	import { getSession } from '@/lib/session.svelte';
	import Button from './Button.svelte';

	const header = getGlobal('header');
	const recipesPage = getGlobal('recipesPage');
	const site = getSite();
	const route = getRoute();
	const session = getSession();

	let open = $state(false);

	let listUrl = $derived($recipesPage?.url ?? '/recipes');
	let categories = $derived(
		($header?.categories ?? []).map(c => ({
			...c,
			url: `${listUrl}?category=${c.slug}`,
		})),
	);
	let active = $derived($route.getSearchParam('category'));
	let submitUrl = $derived(
		session.user ? '/recipes/new' : '/sign-in?next=/recipes/new',
	);

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
		<a href="{listUrl}#search">Search</a>
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
			<a href="{listUrl}#search">Search</a>
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

	.side .active {
		font-family: var(--font-serif);
		font-style: italic;
		font-weight: 400;
		font-size: 1.0625rem;
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

	@include desktop {
		.logo {
			position: absolute;
			left: 50%;
			transform: translateX(-50%);
		}

		.side {
			display: flex;
			align-items: center;
			gap: 1.5rem;
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
