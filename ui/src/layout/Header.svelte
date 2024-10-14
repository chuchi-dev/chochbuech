<script>
	import { getSession } from '@/lib/Session';
	import { getRouter } from '@/main';

	const session = getSession();
	const req = getRouter().currentRequest;

	let open = $state(false);

	$effect(() => {
		open = !$req;
	});
</script>

<div class="placeholder"></div>

<header>
	<div class="controls wrap wide">
		<a href="/" class="logo">Chochbuech</a>

		<button
			class="menu btn secondary small mob"
			onclick={() => (open = !open)}
		>
			{!open ? 'Menu' : 'Close'}
		</button>
	</div>

	<nav class="wrap wide" class:open>
		<div class="left">
			<a href="/appetisers">Appetisers</a>
			<a href="/mains">Mains</a>
			<a href="/desserts">Desserts</a>
			<a href="/search" class="mob">Search</a>
		</div>

		<div class="right mob">
			<a href="/search" class="desk">Search</a>
			{#if !$session.isLoggedIn()}
				<a href="/signin" class="signin btn secondary">Sign In</a>
			{:else}
				<a href="/me" class="signin">
					{$session.shortUser?.name ?? ''}
				</a>
			{/if}
			<a href="/submit" class="submit btn">Submit Recipe</a>
		</div>

		<div class="right desk">
			<a href="/search" class="desk">Search</a>
			{#if !$session.isLoggedIn()}
				<a href="/signin" class="signin">Sign In</a>
			{:else}
				<a href="/me" class="signin">
					{$session.shortUser?.name ?? ''}
				</a>
			{/if}
			<a href="/submit" class="submit btn small">Submit Recipe</a>
		</div>
	</nav>
</header>

<style lang="scss">
	.placeholder {
		height: 5rem;
	}

	header {
		position: fixed;
		top: 0;
		width: 100%;
	}

	.logo {
		display: block;
		font-family: var(--font-secondary);
		font-style: italic;
		text-decoration: none;
		font-size: 1.5rem;
		color: inherit;
		line-height: normal;
		margin-top: 0.7rem;
	}

	$bp: 870px;

	// mobile
	@media (max-width: $bp - 1px) {
		.desk {
			display: none !important;
		}

		.controls {
			display: flex;
			height: 5rem;
			align-items: center;
			justify-content: space-between;
			border-bottom: 1px solid var(--blue-15);
		}

		nav {
			display: none;

			&.open {
				display: flex;
				padding-top: 1rem;
				padding-bottom: 1rem;
				height: calc(100vh - 5rem);
				background-color: var(--white);
				flex-direction: column;
				justify-content: center;
			}

			.left,
			.right {
				display: flex;
				flex-direction: column;
				align-items: center;
			}

			.left {
				gap: 1.5rem;
			}

			.right {
				margin-top: 3.85rem;
				gap: 2rem;
			}

			a {
				display: block;

				&:not(.btn) {
					text-decoration: none;
					font-family: var(--font-secondary);
					font-size: 2.5rem;
					color: inherit;
					font-weight: 200;
					font-style: italic;
				}
			}
		}
	}

	// desktop
	@media (min-width: $bp) {
		.mob {
			display: none !important;
		}

		nav {
			display: flex;
			height: 5rem;
			align-items: center;
			justify-content: space-between;
			border-bottom: 1px solid var(--blue-15);

			a {
				display: block;

				&:not(.btn) {
					color: inherit;
					text-decoration: none;
				}
			}

			.left,
			.right {
				display: flex;
				gap: 2rem;
				align-items: center;
			}
		}

		.controls {
			position: absolute;
			left: 50%;
			display: flex;
			height: 5rem;
			align-items: center;
			transform: translateX(-50%);

			.logo {
				font-size: 2rem;
			}
		}
	}
</style>
