<script>
	import { formatAmount } from '@/lib/recipe';
	import RoundButton from '../RoundButton.svelte';

	// the blue box: people with - and +, the ingredients of the chapters
	// scaled to them
	let { serves, chapters } = $props();

	// starts with the recipe's, then it is the visitor's
	// svelte-ignore state_referenced_locally
	let people = $state(serves || 1);
	let factor = $derived(people / (serves || 1));
	let groups = $derived(
		chapters
			.map((c, i) => ({
				title: c.title || (i === 0 ? 'Ingredients' : ''),
				ingredients: c.ingredients ?? [],
			}))
			.filter(g => g.ingredients.length),
	);
</script>

<aside class="box">
	<div class="people">
		<p aria-live="polite">{people} {people === 1 ? 'Person' : 'People'}</p>
		<div class="buttons">
			<RoundButton
				icon="minus"
				label="Fewer people"
				size="sm"
				onclick={() => (people = Math.max(1, people - 1))}
			/>
			<RoundButton
				icon="plus"
				label="More people"
				size="sm"
				onclick={() => (people += 1)}
			/>
		</div>
	</div>
	{#each groups as group, i (i)}
		<div class="group">
			<h3>{group.title || 'Ingredients'}</h3>
			<ul>
				{#each group.ingredients as ing, j (j)}
					<li>
						<span class="amount">
							{formatAmount(ing.amount, ing.unit, factor)}
						</span>
						<span>{ing.name}</span>
					</li>
				{/each}
			</ul>
		</div>
	{/each}
</aside>

<style lang="scss">
	.box {
		--line: rgba(246, 246, 244, 0.4);
		--focus-color: var(--bg);

		background: var(--accent);
		color: var(--text-invert);
	}

	.people {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1.25rem;
		border-bottom: 1px solid var(--text-invert);
		font-size: var(--lead);
	}

	.buttons {
		display: flex;
		gap: 0.25rem;

		:global(.inner) {
			border-color: var(--bg);
			background: var(--bg);
			color: var(--accent);
		}
	}

	.group {
		margin-inline: 1.25rem;
		padding-block: 1.5rem;

		& + .group {
			border-top: 1px solid var(--line);
		}

		&:last-child {
			padding-bottom: 2rem;
		}
	}

	h3 {
		margin-bottom: 0.5rem;
		font-size: var(--lead);
	}

	li {
		display: grid;
		grid-template-columns: 4rem 1fr;
		gap: 0.5rem;
		padding-block: 0.25rem;
	}

	@include desktop {
		.people,
		.group {
			padding-inline: 0.25rem;
		}

		.people {
			padding: 1rem 1.5rem;
		}

		.group {
			margin-inline: 1.5rem;
			padding-inline: 0;
		}
	}
</style>
