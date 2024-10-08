<footer>
	<div class="wrap wide top">
		<div class="copyright">© {new Date().getFullYear()}</div>

		<nav class="footer">
			<a href="/contact">Contact</a>
			<a href="/imprint">Imprint</a>
			<a href="/privacy">Privacy</a>
		</nav>
	</div>

	<div class="wrap wide made-width">
		<p>
			Made with <span class="heart">❤</span>
			using
			<a href="https://chuchi.dev">Chuchi</a>
		</p>
	</div>
</footer>

<style lang="scss">
	footer {
		position: relative;
		font-size: 0.8125rem;
	}

	.wrap {
		padding-top: 1.25rem;
		padding-bottom: 1.25rem;
		border-top: 1px solid var(--blue-15);
	}

	.top {
		display: flex;
		justify-content: space-between;

		nav.footer {
			display: flex;
			gap: 1rem;
		}

		a {
			display: block;
			color: inherit;
			text-decoration: none;
		}
	}

	.made-width {
		text-align: center;
		color: var(--blue-50);

		a {
			color: inherit;
			text-underline-offset: 0.125rem;
		}
	}

	@media (min-width: 680px) {
		.made-width {
			// well this is not really nice, but mh...
			position: absolute;
			border-top: 0;
			bottom: 0;
			left: 50%;
			transform: translateX(-50%);
		}
	}
</style>
