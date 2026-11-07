<script lang="ts">
	import Button from './Button.svelte';

	// there is no sign up: the invitation to write us instead, for guests
	// on /join, in the editor and on /sign-in?register. `signIn` is the
	// link for members who have an account already.
	let { title, signIn }: { title: string; signIn: string } = $props();
</script>

<div class="join">
	<h1 class="h1"><i>{title}</i></h1>
	<p class="lead">
		Want to share your recipes? Accounts are by invitation for now: write us
		at hello@chochbuech.com
	</p>
	<Button href="mailto:hello@chochbuech.com">Write us</Button>
	<p class="member">
		Already a member? <a href={signIn}>Sign in</a>
	</p>
</div>

<style lang="scss">
	.join {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2.5rem;
		max-width: calc(36rem + 2 * var(--sx-body));
		margin-inline: auto;
		padding: 7.5rem var(--sx-body) var(--sy-section);
		text-align: center;
	}

	.lead {
		text-wrap: balance;
	}

	.member {
		color: var(--accent-soft);

		a {
			@include underline;
			color: var(--accent);
		}
	}

	@include desktop {
		.join {
			padding-top: 8.75rem;
		}
	}

	@include hover {
		.member a:hover {
			opacity: 0.6;
		}
	}
</style>
