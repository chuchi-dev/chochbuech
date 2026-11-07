<script module>
	import plannerQuery from '@/queries/plannerRecipes.graphql';

	// all recipes to pick from, with their ingredients for the shopping list
	/** @type {import('crelte').LoadData} */
	export const loadData = cr => cr.query(plannerQuery);
</script>

<script>
	import AccountLayout from '@/components/AccountLayout.svelte';
	import { getMock } from '@/lib/mock.svelte';
	import { getSession } from '@/lib/session.svelte';
	import { formatAmount } from '@/lib/recipe';

	// the week: a recipe per day (the cookie `chochbuech-planner`), and the
	// ingredients of all of them summed up as a shopping list
	let { entry, recipes } = $props();

	const mock = getMock();
	const session = getSession();

	const DAYS = [
		['monday', 'Monday'],
		['tuesday', 'Tuesday'],
		['wednesday', 'Wednesday'],
		['thursday', 'Thursday'],
		['friday', 'Friday'],
		['saturday', 'Saturday'],
		['sunday', 'Sunday'],
	];

	/** @type {string[]} */
	let checked = $state([]);

	let favorites = $derived(recipes.filter(r => mock.isFavorite(r.id)));
	let own = $derived(
		recipes.filter(r => r.authorId === String(session.user?.id)),
	);

	/** @param {string} id */
	function recipeOf(id) {
		return recipes.find(r => r.id === id) ?? null;
	}

	let planned = $derived(
		DAYS.map(([day]) => mock.planner[day])
			.filter(Boolean)
			.map(recipeOf)
			.filter(Boolean),
	);

	/** @type {{key: string, name: string, amount: string}[]} */
	let shopping = $derived.by(() => {
		/** @type {Map<string, {key: string, name: string, unit: string | null, amount: number | null}>} */
		// eslint-disable-next-line svelte/prefer-svelte-reactivity -- a local temporary
		const items = new Map();
		for (const recipe of planned) {
			for (const chapter of recipe.chapters ?? []) {
				for (const ing of chapter.ingredients ?? []) {
					if (!ing.name?.trim()) continue;
					const unit = ing.unit || null;
					const key = `${ing.name.trim().toLowerCase()}|${unit ?? ''}`;
					const item = items.get(key) ?? {
						key,
						name: ing.name.trim(),
						unit,
						amount: null,
					};
					if (ing.amount !== null && ing.amount !== undefined) {
						item.amount = (item.amount ?? 0) + ing.amount;
					}
					items.set(key, item);
				}
			}
		}
		return [...items.values()]
			.sort((a, b) => a.name.localeCompare(b.name))
			.map(i => ({
				key: i.key,
				name: i.name,
				amount: formatAmount(i.amount, i.unit),
			}));
	});

	/** @param {string} day @param {Event} e */
	function pick(day, e) {
		const value = /** @type {HTMLSelectElement} */ (e.currentTarget).value;
		mock.plan(day, value || null);
	}

	/** @param {string} key */
	function toggle(key) {
		checked = checked.includes(key)
			? checked.filter(k => k !== key)
			: [...checked, key];
	}
</script>

<AccountLayout title={entry.title} note="Saved in this browser." guests>
	<div class="planner">
		<section aria-labelledby="week">
			<h2 id="week" class="sr-only">Your week</h2>
			<ul class="days">
				{#each DAYS as [day, label] (day)}
					{@const id = mock.planner[day] ?? ''}
					{@const recipe = id ? recipeOf(id) : null}
					<li class="day">
						<label for="day-{day}" class="name">
							<i>{label}</i>
						</label>
						<div class="pick">
							<select
								id="day-{day}"
								value={id}
								onchange={e => pick(day, e)}
							>
								<option value="">Nothing planned</option>
								{#if favorites.length}
									<optgroup label="Favorites">
										{#each favorites as r (r.id)}
											<option value={r.id}>
												{r.title}
											</option>
										{/each}
									</optgroup>
								{/if}
								{#if own.length}
									<optgroup label="My recipes">
										{#each own as r (r.id)}
											<option value={r.id}>
												{r.title}
											</option>
										{/each}
									</optgroup>
								{/if}
								<optgroup label="All recipes">
									{#each recipes as r (r.id)}
										<option value={r.id}>{r.title}</option>
									{/each}
								</optgroup>
							</select>
							{#if recipe}
								<a class="open small" href={recipe.url}>
									Open recipe
									<span class="sr-only">
										: {recipe.title}
									</span>
								</a>
							{/if}
						</div>
					</li>
				{/each}
			</ul>
		</section>

		<section class="shopping" aria-labelledby="shopping">
			<h2 id="shopping" class="h2"><i>Shopping List</i></h2>
			{#if shopping.length}
				<p class="hint">
					The ingredients of the week as in the recipes, for
					{planned.length}
					{planned.length === 1 ? 'meal' : 'meals'}.
				</p>
				<ul class="list">
					{#each shopping as item (item.key)}
						<li>
							<label class:done={checked.includes(item.key)}>
								<input
									type="checkbox"
									checked={checked.includes(item.key)}
									onchange={() => toggle(item.key)}
								/>
								<span class="amount">{item.amount}</span>
								<span class="ingredient">{item.name}</span>
							</label>
						</li>
					{/each}
				</ul>
			{:else}
				<p class="hint">
					Pick a recipe for a day and its ingredients show up here.
				</p>
			{/if}
		</section>
	</div>
</AccountLayout>

<style lang="scss">
	.planner {
		max-width: calc(48rem + 2 * var(--sx-body));
		margin-inline: auto;
		padding-inline: var(--sx-body);
	}

	.days,
	.list {
		list-style: none;
		border-top: 1px solid var(--line);
	}

	.day {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding-block: 1.25rem;
		border-bottom: 1px solid var(--line);
	}

	.name {
		font-family: var(--font-serif);
		font-weight: 200;
		font-size: 1.75rem;
		line-height: 1.25;
	}

	.pick {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	select {
		min-width: 0;
		flex: 1;
		padding: 0.75rem 2.5rem 0.75rem 1rem;
		border: 1px solid var(--line);
		border-radius: 0;
		background: var(--bg-card);
		color: var(--accent);
		appearance: none;
		background-image:
			linear-gradient(45deg, transparent 50%, currentColor 50%),
			linear-gradient(135deg, currentColor 50%, transparent 50%);
		background-position:
			calc(100% - 1.25rem) 50%,
			calc(100% - 0.9rem) 50%;
		background-size: 0.35rem 0.35rem;
		background-repeat: no-repeat;
		transition: border-color 0.3s ease;

		&:focus {
			border-color: var(--accent);
		}
	}

	.open {
		@include underline;
		flex-shrink: 0;
	}

	.shopping {
		margin-top: 6rem;
	}

	h2 {
		text-align: center;
	}

	.hint {
		margin: 1rem 0 3rem;
		color: var(--accent-soft);
		text-align: center;
	}

	.list label {
		display: flex;
		align-items: baseline;
		gap: 1rem;
		padding-block: 0.875rem;
		border-bottom: 1px solid var(--line);
		cursor: pointer;
		transition: opacity 0.3s ease;

		&.done {
			opacity: 0.5;

			.ingredient {
				text-decoration: line-through;
			}
		}
	}

	.list li:last-child label {
		border-bottom: 0;
	}

	.list {
		border-bottom: 1px solid var(--line);
	}

	input {
		flex-shrink: 0;
		width: 1rem;
		height: 1rem;
		accent-color: var(--accent);
		transform: translateY(0.125rem);
	}

	.amount {
		flex-shrink: 0;
		width: 5.5rem;
	}

	@include tablet {
		.day {
			flex-direction: row;
			align-items: center;
			justify-content: space-between;
		}

		.pick {
			width: 28rem;
		}
	}

	@include desktop {
		.shopping {
			margin-top: 8rem;
		}
	}

	@include hover {
		.open:hover,
		.list label:hover {
			opacity: 0.6;
		}

		select:hover {
			border-color: var(--accent);
		}
	}
</style>
