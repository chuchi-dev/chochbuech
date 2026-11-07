<script>
	import { getRoute, getRouter } from 'crelte';
	import { getMock } from '@/lib/mock.svelte';
	import { getSession } from '@/lib/session.svelte';
	import { getToasts } from '@/lib/toasts.svelte';
	import RoundButton from '../RoundButton.svelte';

	// favorite, rate and change a recipe: the buttons on the image's edge.
	// The star opens the five stars of the rating (Rating Toggle). The
	// sliders edit it for its author, else (admins too) they start a recipe
	// of the user from it.
	let { id, authorId } = $props();

	const mock = getMock();
	const session = getSession();
	const toasts = getToasts();
	const router = getRouter();
	const route = getRoute();

	let rating = $state(false);

	let favorite = $derived(id ? mock.isFavorite(id) : false);
	let rated = $derived(id ? (mock.ratings[id] ?? 0) : 0);
	let user = $derived(session.user);
	let mine = $derived(!!user && String(user.id) === authorId);
	let changeUrl = $derived(
		mine ? `/recipes/edit/${id}` : `/recipes/copy/${id}`,
	);

	function signInFirst() {
		if (session.user) return false;
		toasts.show('Sign in to keep your favorites and ratings.');
		router.open(`/sign-in?next=${encodeURIComponent($route.url.pathname)}`);
		return true;
	}

	function toggleFavorite() {
		if (signInFirst()) return;
		const on = mock.toggleFavorite(id);
		toasts.show(
			on ? 'Added to your favorites.' : 'Removed from your favorites.',
		);
	}

	/** @param {number} stars */
	function rate(stars) {
		if (signInFirst()) return;
		mock.rate(id, stars);
		rating = false;
		toasts.show(`Thank you! You rated it ${stars} of 5.`);
	}
</script>

<div class="actions">
	{#if rating}
		{#each [1, 2, 3, 4, 5] as star (star)}
			<RoundButton
				icon="star"
				label="{star} of 5"
				active={star <= rated}
				onclick={() => rate(star)}
			/>
		{/each}
	{:else}
		{#if id}
			<RoundButton
				icon="heart"
				label={favorite ? 'Remove from favorites' : 'Add to favorites'}
				active={favorite}
				onclick={toggleFavorite}
			/>
			<RoundButton
				icon="star"
				label={rated ? `You rated it ${rated} of 5` : 'Rate'}
				active={rated > 0}
				onclick={() => (signInFirst() ? null : (rating = true))}
			/>
		{/if}
		<RoundButton
			icon="sliders"
			label={mine ? 'Edit' : 'Make it your own'}
			href={user
				? changeUrl
				: `/sign-in?next=${encodeURIComponent(changeUrl)}`}
		/>
	{/if}
</div>

<style lang="scss">
	.actions {
		display: flex;
		justify-content: center;
		gap: 0.5rem;
	}
</style>
