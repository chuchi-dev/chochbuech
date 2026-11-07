<script>
	import { getRoute, getRouter } from 'crelte';
	import { getMock } from '@/lib/mock.svelte';
	import { getToasts } from '@/lib/toasts.svelte';
	import RoundButton from '../RoundButton.svelte';

	// favorite, rate and change a recipe: the buttons on the image's edge.
	// The star opens the five stars of the rating (Rating Toggle).
	let { id, editUrl } = $props();

	const mock = getMock();
	const toasts = getToasts();
	const router = getRouter();
	const route = getRoute();

	let rating = $state(false);

	let favorite = $derived(id ? mock.isFavorite(id) : false);
	let rated = $derived(id ? (mock.ratings[id] ?? 0) : 0);

	function signInFirst() {
		if (mock.user) return false;
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
			label="Make it your own"
			href={mock.user
				? editUrl
				: `/sign-in?next=${encodeURIComponent(editUrl)}`}
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
