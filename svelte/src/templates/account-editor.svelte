<script module>
	import recipeQuery from '@/queries/recipeForEdit.graphql';

	// a recipe of zipp to edit comes from zipp, a local one from the
	// browser (in the component)
	/** @type {import('crelte').LoadData} */
	export const loadData = (cr, entry) => {
		const id = entry?.recipeId;
		if (!id || !/^\d+$/.test(id)) return null;
		return cr.query(recipeQuery, { id });
	};
</script>

<script>
	import { untrack } from 'svelte';
	import { getGlobal, getRoute, getRouter } from 'crelte';
	import Button from '@/components/Button.svelte';
	import Icon from '@/components/Icon.svelte';
	import ChapterCard from '@/components/editor/ChapterCard.svelte';
	import NumberField from '@/components/editor/NumberField.svelte';
	import {
		TIME_UNITS,
		emptyForm,
		fromLocal,
		fromZipp,
		newChapter,
		toLocal,
		validate,
	} from '@/components/editor/form';
	import { getMock } from '@/lib/mock.svelte';
	import { getToasts } from '@/lib/toasts.svelte';

	/** @import { Form, Errors } from '@/components/editor/form' */

	const mock = getMock();
	const toasts = getToasts();
	const router = getRouter();
	const route = getRoute();
	const categories = getGlobal('categories');

	/** @type {{ entry: any, recipe?: any }} */
	let { entry, recipe = null } = $props();

	// the template is new for every url, the props are read once
	const zipp = untrack(() => recipe);
	/** @type {string | null} */
	const recipeId = untrack(() => entry.recipeId) ?? null;
	const isLocal = !!recipeId?.startsWith('local-');
	const editing = !!recipeId;

	/** @type {Form | null} */
	let form = $state(
		!editing ? emptyForm() : !isLocal && zipp ? fromZipp(zipp) : null,
	);
	let notFound = $state(editing && !isLocal && !zipp);
	// a local one, or the local copy of a zipp recipe, after the mount
	let checkedLocal = false;
	/** @type {Errors} */
	let errors = $state({});
	let tried = $state(false);

	$effect(() => {
		if (!mock.localLoaded || checkedLocal || !editing) return;
		checkedLocal = true;
		const local = isLocal
			? mock.localRecipe(recipeId)
			: mock.recipes.find(r => r.basedOn === recipeId);
		if (local) {
			form = fromLocal(local);
			notFound = false;
		} else if (isLocal) {
			notFound = true;
		}
	});

	// once tried, the errors follow the input
	$effect(() => {
		if (tried && form) errors = validate(form);
	});

	let title = $derived(editing ? 'Edit Recipe' : 'New Recipe');
	let signInUrl = $derived(
		'/sign-in?next=' + encodeURIComponent($route.url.pathname),
	);
	let cancelUrl = $derived(
		isLocal ? `/my-recipes/${recipeId}` : (zipp?.url ?? '/account'),
	);

	/** @param {number} i @param {-1 | 1} dir */
	function moveChapter(i, dir) {
		if (!form) return;
		const cs = form.chapters;
		[cs[i], cs[i + dir]] = [cs[i + dir], cs[i]];
	}

	/** @param {number} i */
	function removeChapter(i) {
		if (!form) return;
		form.chapters.splice(i, 1);
	}

	function addChapter() {
		if (!form) return;
		for (const c of form.chapters) c.open = false;
		form.chapters.push(newChapter());
	}

	/** @param {SubmitEvent} e */
	function save(e) {
		e.preventDefault();
		if (!form) return;
		tried = true;
		errors = validate(form);
		const missing = [
			errors.title && 'the title',
			errors.serves && 'the number of people',
			errors.chapters && 'at least one ingredient or step',
		].filter(Boolean);
		if (missing.length) {
			const list =
				missing.length > 1
					? missing.slice(0, -1).join(', ') + ' and ' + missing.at(-1)
					: missing[0];
			toasts.error(`Please fill in ${list}.`);
			// the first chapter with its error is shown
			if (errors.chapters && !form.chapters.some(c => c.open))
				form.chapters[0].open = true;
			return;
		}
		const saved = mock.saveRecipe(toLocal($state.snapshot(form)));
		toasts.show('The recipe has been saved.');
		router.open('/my-recipes/' + saved.id);
	}
</script>

<svelte:head>
	<title>{title} | Chuchi Recipes</title>
</svelte:head>

{#if !mock.user}
	<div class="note narrow">
		<h1 class="h1">{title}</h1>
		<p class="lead">Sign in to write your own recipes.</p>
		<Button href={signInUrl}>Sign In</Button>
	</div>
{:else if notFound}
	<div class="note narrow">
		<h1 class="h1">{title}</h1>
		<p class="lead">This recipe does not exist (anymore).</p>
		<Button href="/account" look="secondary">My Recipes</Button>
	</div>
{:else if !form}
	<div class="note narrow">
		<h1 class="h1">{title}</h1>
		<p class="lead">Loading …</p>
	</div>
{:else}
	<form class="editor" onsubmit={save} novalidate>
		<div class="bar">
			<div class="bar-inner">
				<Button href={cancelUrl} look="secondary">Cancel</Button>
				<Button type="submit">Save &amp; Publish Recipe</Button>
			</div>
		</div>

		<div class="content narrow">
			<h1 class="h1">{title}</h1>

			<div class="fields">
				<div class="field wide">
					<label for="title">Recipe Title:</label>
					<input
						id="title"
						class="box"
						class:invalid={!!errors.title}
						type="text"
						autocomplete="off"
						bind:value={form.title}
						aria-invalid={errors.title ? true : undefined}
						aria-describedby={errors.title
							? 'title-error'
							: undefined}
					/>
					{#if errors.title}
						<p class="error" id="title-error">{errors.title}</p>
					{/if}
				</div>

				<fieldset class="field wide">
					<legend>Category:</legend>
					<div class="categories">
						{#each $categories ?? [] as c (c.id)}
							<label class="pill">
								<input
									type="radio"
									name="category"
									value={c.slug}
									bind:group={form.category}
								/>
								<span>{c.singular || c.title}</span>
							</label>
						{/each}
					</div>
				</fieldset>

				<NumberField
					id="total"
					label="Time spent totally:"
					bind:value={form.total}
					bind:unit={form.totalUnit}
					units={TIME_UNITS}
				/>

				<NumberField
					id="active"
					label="Time spent actively:"
					bind:value={form.active}
					bind:unit={form.activeUnit}
					units={TIME_UNITS}
				/>

				<NumberField
					id="serves"
					label="This recipe serves:"
					step="1"
					min={1}
					error={errors.serves}
					bind:value={form.serves}
					suffix="people"
				/>
			</div>

			<h2 class="h1 instructions">Instructions</h2>
			{#if errors.chapters}
				<p class="error chapters-error" role="alert">
					{errors.chapters}
				</p>
			{/if}

			<div class="chapters">
				{#each form.chapters as chapter, i (chapter.key)}
					<ChapterCard
						bind:chapter={form.chapters[i]}
						index={i}
						count={form.chapters.length}
						onmove={dir => moveChapter(i, dir)}
						onremove={() => removeChapter(i)}
					/>
				{/each}

				<button type="button" class="add-chapter" onclick={addChapter}>
					<span>add Chapter</span>
					<span class="plus" aria-hidden="true">
						<span class="plus-inner"><Icon name="plus" /></span>
					</span>
				</button>
			</div>
		</div>
	</form>
{/if}

<style lang="scss">
	.note {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 2rem;
		padding-block: 5rem var(--sy-section);
	}

	.editor {
		display: flex;
		flex-direction: column;
	}

	// mobile: the buttons stick to the bottom of the screen until the end
	// of the form
	.bar {
		position: sticky;
		bottom: 0;
		order: 1;
		z-index: 90;
		border-top: 1px solid var(--line);
		background: var(--bg);
	}

	.bar-inner {
		display: flex;
		justify-content: space-between;
		gap: 0.5rem;
		padding: 0.75rem var(--sx-body);
	}

	.content {
		padding-block: 5rem var(--sy-section);
	}

	.fields {
		display: grid;
		gap: 2rem;
		margin-top: 2rem;
	}

	fieldset {
		margin: 0;
		padding: 0;
		border: 0;
		min-width: 0;
	}

	.field > label,
	legend {
		display: block;
		margin-bottom: 1rem;
		padding: 0;
	}

	.box {
		display: block;
		width: 100%;
		height: 3rem;
		padding-inline: 1rem;
		border: 1px solid var(--line);
		background: var(--bg-card);
		color: inherit;
		font: inherit;
		outline: none;
		transition: border-color 0.3s ease;

		&:focus {
			border-color: var(--accent);
		}

		&.invalid {
			border-color: var(--error);
		}
	}

	.categories {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.pill {
		position: relative;
		cursor: pointer;

		input {
			position: absolute;
			opacity: 0;
			pointer-events: none;
		}

		span {
			display: block;
			padding: 0.25rem 0.75rem;
			border: 1px solid var(--line);
			border-radius: 999px;
			transition:
				background-color 0.3s ease,
				color 0.3s ease,
				border-color 0.3s ease;
		}

		input:checked + span {
			border-color: var(--accent);
			background: var(--accent);
			color: var(--text-invert);
		}

		input:focus-visible + span {
			outline: 2px solid var(--focus-color);
			outline-offset: 2px;
		}
	}

	.error {
		@include small;
		margin-top: 0.5rem;
		color: var(--error);
	}

	.instructions {
		margin-top: 5rem;
	}

	.chapters-error {
		margin: 0 0 1rem;
	}

	.chapters {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
		margin-top: 2rem;
	}

	.add-chapter {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		border: 1px solid var(--bg-card);
		text-align: left;
		cursor: pointer;
		transition: border-color 0.3s ease;
	}

	.plus {
		display: flex;
		padding: 0.1875rem;
		border: 1px solid var(--line);
		border-radius: 50%;
	}

	.plus-inner {
		display: flex;
		width: 1.75rem;
		height: 1.75rem;
		align-items: center;
		justify-content: center;
		border: 1px solid var(--accent);
		border-radius: 50%;
		font-size: 0.875rem;
		transition:
			background-color 0.3s ease,
			color 0.3s ease;
	}

	@include tablet {
		// tablet and desktop: under the header, right aligned
		.bar {
			top: var(--topbar-height);
			bottom: auto;
			order: 0;
			border-top: 0;
			border-bottom: 1px solid var(--line);
		}

		.bar-inner {
			justify-content: flex-end;
			gap: 1rem;
		}

		.fields {
			grid-template-columns: 1fr 1fr;
			column-gap: 2rem;
		}

		.wide {
			grid-column: 1 / -1;
		}

		.add-chapter {
			@include lead;
			padding: 1.25rem 2.5rem;
		}
	}

	@include desktop {
		.content {
			padding-top: 7.5rem;
		}

		.fields {
			margin-top: 3rem;
		}

		.box {
			height: 4rem;
			padding-inline: 2rem;
			font-size: 1.25rem;
		}

		.instructions {
			margin-top: 7.5rem;
		}

		.chapters {
			margin-top: 3rem;
		}
	}

	@include hover {
		.pill:hover input:not(:checked) + span {
			border-color: var(--accent);
		}

		.add-chapter:hover {
			border-color: var(--accent);

			.plus-inner {
				background: var(--accent);
				color: var(--text-invert);
			}
		}
	}
</style>
