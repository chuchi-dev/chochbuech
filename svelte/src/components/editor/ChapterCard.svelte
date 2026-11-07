<script lang="ts">
	import { tick } from 'svelte';
	import { UNITS } from '@/lib/recipe';
	import Icon from '../Icon.svelte';
	import RoundButton from '../RoundButton.svelte';
	import ChipSelect from './ChipSelect.svelte';
	import MoveButton from './MoveButton.svelte';
	import SortLine from './SortLine.svelte';
	import { newIngredient, type FormChapter } from './form';
	import { Sortable, moved } from './sortable.svelte';

	// a chapter of the editor: its head with the handles, open with its
	// title, ingredients and steps under it
	let {
		chapter = $bindable(),
		index,
		count,
		sortable,
		onmove,
		onremove,
	}: {
		chapter: FormChapter;
		index: number;
		count: number;
		/** the chapters' */
		sortable: Sortable;
		onmove: (dir: -1 | 1) => void;
		onremove: () => void;
	} = $props();

	// ingredients without a unit (salt, pepper) get the empty one
	const UNIT_OPTIONS: [string, string][] = [...UNITS, ['', 'no unit']];

	let id = $derived(`chapter-${chapter.key}`);
	let name = $derived(
		`Chapter ${index + 1}` +
			(chapter.title.trim() ? `: ${chapter.title}` : ''),
	);
	let list: HTMLElement | undefined = $state();
	const sort = new Sortable(
		() => list,
		(from, to) =>
			(chapter.ingredients = moved(chapter.ingredients, from, to)),
	);

	async function addIngredient() {
		chapter.ingredients.push(newIngredient());
		await tick();
		const inputs = list?.querySelectorAll<HTMLInputElement>('.name input');
		inputs?.[inputs.length - 1]?.focus();
	}

	function moveIngredient(i: number, dir: -1 | 1) {
		const ings = chapter.ingredients;
		[ings[i], ings[i + dir]] = [ings[i + dir], ings[i]];
	}

	function removeIngredient(i: number) {
		chapter.ingredients.splice(i, 1);
	}
</script>

<section
	class="chapter"
	class:open={chapter.open}
	class:dragging={sortable.dragging === index}
	aria-label={name}
	data-sort
>
	<div class="head">
		<MoveButton
			label="Move {name}"
			canUp={index > 0}
			canDown={index < count - 1}
			{sortable}
			{index}
			{onmove}
		/>
		<button
			type="button"
			class="name-button"
			aria-expanded={chapter.open}
			onclick={() => (chapter.open = !chapter.open)}
		>
			{name}
		</button>
		<div class="buttons">
			<RoundButton
				icon="x"
				label="Remove {name}"
				filled={false}
				size="sm"
				ring={false}
				onclick={onremove}
			/>
			<RoundButton
				icon={chapter.open ? 'up' : 'down'}
				label={chapter.open ? `Collapse ${name}` : `Expand ${name}`}
				filled={false}
				size="sm"
				ring={false}
				onclick={() => (chapter.open = !chapter.open)}
			/>
		</div>
	</div>

	{#if chapter.open}
		<div class="body">
			<div class="title-field">
				<label for="{id}-title">Chapter Title</label>
				<input
					id="{id}-title"
					class="box"
					type="text"
					placeholder="e.g. Dough"
					bind:value={chapter.title}
				/>
			</div>

			<div class="ingredients">
				<h3 id="{id}-ingredients">Ingredients</h3>
				<div class="list">
					<ul aria-labelledby="{id}-ingredients" bind:this={list}>
						{#each chapter.ingredients as ing, i (ing.key)}
							<li
								class="row"
								class:dragging={sort.dragging === i}
								data-sort
							>
								<span class="cell icon grip">
									<MoveButton
										look="cell"
										label="Move ingredient {i + 1}"
										canUp={i > 0}
										canDown={i <
											chapter.ingredients.length - 1}
										sortable={sort}
										index={i}
										onmove={dir => moveIngredient(i, dir)}
									/>
								</span>
								<span class="cell name">
									<input
										type="text"
										placeholder="Ingredient"
										aria-label="Ingredient {i + 1}"
										bind:value={ing.name}
									/>
								</span>
								<span class="cell amount">
									<input
										type="number"
										inputmode="decimal"
										step="any"
										min="0"
										placeholder="Amount"
										aria-label="Amount of ingredient {i +
											1}"
										bind:value={ing.amount}
									/>
									<ChipSelect
										bind:value={ing.unit}
										options={UNIT_OPTIONS}
										label="Unit of ingredient {i + 1}"
									/>
								</span>
								<span class="cell icon">
									<button
										type="button"
										class="remove"
										aria-label="Remove ingredient {i + 1}"
										title="Remove ingredient"
										onclick={() => removeIngredient(i)}
									>
										<Icon name="x" />
									</button>
								</span>
							</li>
						{/each}
					</ul>
					<SortLine sortable={sort} />
				</div>
				<button
					type="button"
					class="add"
					aria-label="Add an ingredient"
					title="Add an ingredient"
					onclick={addIngredient}
				>
					<Icon name="plus" />
				</button>
			</div>

			<div class="steps">
				<label for="{id}-steps">Instructions / Description</label>
				<textarea
					id="{id}-steps"
					class="box"
					rows="10"
					placeholder="Start typing …&#10;An empty line starts the next step."
					bind:value={chapter.text}
				></textarea>
			</div>
		</div>
	{/if}
</section>

<style lang="scss">
	.chapter {
		background: var(--bg-card);
		transition: opacity 0.3s ease;
	}

	.dragging {
		opacity: 0.4;
	}

	// the handle on the left, its circle in line with the fields
	.head {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem 0.75rem calc(1.25rem - 0.25rem);
	}

	.buttons {
		display: flex;
		flex-shrink: 0;
		gap: 0.125rem;
		margin-right: -0.25rem;
	}

	.body {
		display: flex;
		flex-direction: column;
		gap: 2rem;
		padding: 0.5rem 1.25rem 1.25rem;
	}

	label,
	h3 {
		display: block;
		margin-bottom: 1rem;
		font: inherit;
	}

	.box {
		display: block;
		width: 100%;
		padding: 0.75rem 1rem;
		border: 1px solid var(--line);
		background: var(--bg-card);
		color: inherit;
		font: inherit;
		outline: none;
		transition: border-color 0.3s ease;

		&:focus {
			border-color: var(--accent);
		}
	}

	input::placeholder,
	textarea::placeholder {
		color: var(--accent-soft);
		opacity: 1;
	}

	.list {
		position: relative;
	}

	ul {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	// mobile: the handle on the left, the name above the amount
	.row {
		display: grid;
		grid-template-columns: 3rem 1fr 3rem;
		border: 1px solid var(--line);
		background: var(--bg-card);
		transition: opacity 0.3s ease;
	}

	.cell {
		display: flex;
		min-width: 0;
		height: 3rem;
		align-items: center;
		transition: background-color 0.3s ease;

		&:focus-within {
			background: var(--bg);
		}
	}

	.name {
		grid-column: 2 / -1;
		border-bottom: 1px solid var(--line);
	}

	.amount {
		gap: 0.5rem;
		padding-right: 0.75rem;
	}

	.icon {
		justify-content: center;
		border-left: 1px solid var(--line);
	}

	.grip {
		height: auto;
		grid-row: span 2;
		border-right: 1px solid var(--line);
		border-left: 0;
	}

	.cell input {
		width: 100%;
		min-width: 0;
		height: 100%;
		flex: 1;
		padding-inline: 1rem;
		border: 0;
		background: transparent;
		color: inherit;
		font: inherit;
		outline: none;
		-moz-appearance: textfield;
		appearance: textfield;

		&::-webkit-inner-spin-button,
		&::-webkit-outer-spin-button {
			appearance: none;
			margin: 0;
		}
	}

	.remove {
		display: flex;
		width: 100%;
		height: 100%;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: background-color 0.3s ease;
	}

	.add {
		display: flex;
		width: 3rem;
		height: 3rem;
		align-items: center;
		justify-content: center;
		margin-top: 0.5rem;
		border: 1px solid var(--line);
		font-size: 1.25rem;
		cursor: pointer;
		transition:
			border-color 0.3s ease,
			background-color 0.3s ease,
			color 0.3s ease;
	}

	textarea {
		min-height: 20rem;
		resize: vertical;
	}

	.name-button {
		min-width: 0;
		flex: 1;
		padding-block: 0.5rem;
		overflow: hidden;
		text-align: left;
		text-overflow: ellipsis;
		white-space: nowrap;
		cursor: pointer;
	}

	@include tablet {
		.head {
			gap: 1rem;
			padding: 1.25rem 2.5rem 1.25rem calc(2.5rem - 0.25rem);
		}

		.body {
			padding: 0.75rem 2.5rem 2.5rem;
		}

		.title-field {
			width: calc(50% - 1.5rem);
		}

		.title-field .box {
			padding: 0.75rem;
		}

		.name-button {
			@include lead;
		}

		ul {
			gap: 0;
		}

		// desktop: one table, the rows share their lines
		.row {
			grid-template-columns: 3rem 1fr 1fr 3rem;

			& + .row {
				border-top: 0;
			}
		}

		.name {
			grid-column: auto;
			border-bottom: 0;
		}

		.grip {
			height: 3rem;
			grid-row: auto;
		}

		.amount {
			border-left: 1px solid var(--line);
		}

		.cell input {
			padding-inline: 0.75rem;
		}

		.add {
			margin-top: 0;
			border-top: 0;
		}

		textarea {
			min-height: 15rem;
			padding: 1rem 1.25rem;
		}
	}

	@include hover {
		.remove:hover {
			background: var(--bg);
		}

		.add:hover {
			border-color: var(--accent);
			background: var(--accent);
			color: var(--text-invert);
		}

		.name-button:hover {
			@include underline;
		}
	}
</style>
