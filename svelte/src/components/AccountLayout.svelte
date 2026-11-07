<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getRoute, getRouter } from 'crelte';
	import { errorMessage, getSession } from '@/lib/session.svelte';
	import { getToasts } from '@/lib/toasts.svelte';
	import { ACCOUNT_NAV, GUEST_NAV } from '@/routes.js';
	import Button from './Button.svelte';
	import Pills from './Pills.svelte';
	import RoundArrow from './RoundArrow.svelte';

	// the frame of the account pages: the account navigation (pills on
	// desktop, a dropdown on mobile), the user name and the title, then the
	// page. Not signed in it asks to sign in, except on the pages of
	// `guests` (favorites and planner, kept by the browser): those show
	// without the navigation.
	let {
		title,
		note,
		guests = false,
		intro,
		children,
	}: {
		title: string;
		// a small line under the title
		note?: string;
		guests?: boolean;
		// under the title: a button, the filter
		intro?: Snippet;
		children: Snippet;
	} = $props();

	const route = getRoute();
	const router = getRouter();
	const session = getSession();
	const toasts = getToasts();

	let open = $state(false);

	let path = $derived($route.url.pathname);
	let items = $derived([
		...ACCOUNT_NAV.map(([href, label]) => ({ key: href, label, href })),
		{ key: 'logout', label: 'LogOut' },
	]);
	let current = $derived(
		ACCOUNT_NAV.find(([href]) => href === path)?.[1] ?? title,
	);

	async function logOut() {
		try {
			await session.logout();
		} catch (e) {
			toasts.error(errorMessage(e));
			return;
		}
		toasts.show('You have been logged out.');
		router.open('/');
	}
</script>

<svelte:head>
	<title>{title} | Chuchi Recipes</title>
</svelte:head>

{#snippet page()}
	<div class="account">
		<header>
			{#if session.user}
				<p class="user">{session.user.name || session.user.email}</p>
			{:else}
				<nav class="user guest" aria-label="Your browser">
					{#each GUEST_NAV as [href, label] (href)}
						<a {href} class:active={href === path}>{label}</a>
					{/each}
				</nav>
			{/if}
			<h1 class="h1"><i>{title}</i></h1>
			{#if note}
				<p class="note">{note}</p>
			{/if}
			{#if intro}
				<div class="intro">{@render intro()}</div>
			{/if}
		</header>

		{@render children()}
	</div>
{/snippet}

{#if session.user}
	<div class="nav-desktop">
		<Pills
			{items}
			active={path}
			label="Account"
			onselect={key => key === 'logout' && logOut()}
		/>
	</div>

	<nav class="nav-mobile" aria-label="Account">
		<button
			type="button"
			class="row toggle"
			aria-expanded={open}
			aria-controls="account-menu"
			onclick={() => (open = !open)}
		>
			<span>{current}</span>
			<RoundArrow {open} />
		</button>
		{#if open}
			<ul id="account-menu">
				{#each ACCOUNT_NAV as [href, label] (href)}
					{#if href !== path}
						<li><a class="row" {href}>{label}</a></li>
					{/if}
				{/each}
				<li>
					<button type="button" class="row" onclick={logOut}>
						LogOut
					</button>
				</li>
			</ul>
		{/if}
	</nav>

	{@render page()}
{:else if guests}
	{@render page()}
{:else}
	<div class="account">
		<header>
			<h1 class="h1"><i>{title}</i></h1>
			<p class="prompt">
				Sign in to see your recipes, favorites and planner.
			</p>
			<div class="intro">
				<Button href="/sign-in?next={encodeURIComponent(path)}">
					Sign In
				</Button>
			</div>
		</header>
	</div>
{/if}

<style lang="scss">
	.nav-desktop {
		display: none;
	}

	.nav-mobile {
		ul {
			list-style: none;
		}
	}

	.row {
		display: flex;
		width: 100%;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0.5rem var(--sx-body);
		border-bottom: 1px solid var(--line);
		text-align: left;
		cursor: pointer;
		min-height: 3rem;
	}

	.account {
		padding-bottom: var(--sy-section);
	}

	header {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 7.5rem var(--sx-body) 6rem;
		text-align: center;
	}

	.user {
		margin-bottom: 1rem;
	}

	.guest {
		display: flex;
		gap: 1.5rem;

		a {
			color: var(--accent-soft);
			transition: opacity 0.3s ease;
		}

		.active {
			@include underline;
			color: var(--accent);
		}
	}

	.prompt {
		margin-top: 1.5rem;
		color: var(--accent-soft);
	}

	.note {
		@include small;
		margin-top: 1.5rem;
		color: var(--accent-soft);
	}

	.intro {
		display: flex;
		justify-content: center;
		margin-top: 3rem;
	}

	@include desktop {
		.nav-desktop {
			display: flex;
			justify-content: center;
			padding: 2.5rem var(--sx-body) 0;
		}

		.nav-mobile {
			display: none;
		}

		header {
			padding-block: 7rem 5.5rem;
		}
	}

	@include hover {
		.row:hover,
		.guest a:hover {
			opacity: 0.6;
		}
	}
</style>
