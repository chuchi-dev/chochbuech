<script module>
	import recipeQuery from '@/queries/recipeForEdit.graphql';

	// the recipe to change or to copy as it is live (its image, its
	// author); the fields to change come from /members in the component
	/** @type {import('crelte').LoadData} */
	export const loadData = (cr, entry) => {
		const id = entry?.recipeId ?? entry?.copyOf;
		if (!id || !/^\d+$/.test(id)) return null;
		return cr.query(recipeQuery, { id });
	};
</script>

<script>
	import { untrack } from 'svelte';
	import { getCrelte, getGlobal, getRoute, getRouter } from 'crelte';
	import Button from '@/components/Button.svelte';
	import Icon from '@/components/Icon.svelte';
	import Join from '@/components/Join.svelte';
	import ChapterCard from '@/components/editor/ChapterCard.svelte';
	import ImageField from '@/components/editor/ImageField.svelte';
	import NumberField from '@/components/editor/NumberField.svelte';
	import SortLine from '@/components/editor/SortLine.svelte';
	import { Sortable, moved } from '@/components/editor/sortable.svelte';
	import {
		TIME_UNITS,
		emptyForm,
		fieldErrors,
		fromEntry,
		fromGraphql,
		fromImport,
		newChapter,
		toFields,
		validate,
	} from '@/components/editor/form';
	import {
		MemberError,
		createRecipe,
		errorMessage,
		forgetQueries,
		getEntry,
		importRecipe,
		updateEntry,
	} from '@/lib/members';
	import { getSession } from '@/lib/session.svelte';
	import { getToasts } from '@/lib/toasts.svelte';

	/** @import { Form, Errors } from '@/components/editor/form' */

	const crelte = getCrelte();
	const session = getSession();
	const toasts = getToasts();
	const router = getRouter();
	const route = getRoute();
	const categories = getGlobal('categories');

	/** @type {{ entry: any, recipe?: any }} */
	let { entry, recipe = null } = $props();

	// the template is new for every url, the props are read once
	const live = untrack(() => recipe);
	/** @type {string | null} */
	const recipeId = untrack(() => entry.recipeId) ?? null;
	const copyOf = untrack(() => entry.copyOf) ?? null;
	const editing = !!recipeId;

	/** @type {Form | null} */
	let form = $state(
		copyOf
			? live
				? fromGraphql(live)
				: null
			: editing
				? null
				: emptyForm(),
	);
	// why the recipe can't be edited: missing, someone else's
	/** @type {string | null} */
	let problem = $state(copyOf && !live ? 'missing' : null);
	/** @type {Errors} */
	let errors = $state({});
	// zipp's answer to the last save
	/** @type {Errors} */
	let refused = $state({});
	let tried = $state(false);
	// a private save checks only the title
	let triedPrivate = $state(false);
	let saving = $state(false);
	let started = false;
	// whether the recipe is live; a private one (unpublished) only its
	// author sees
	let published = $state(false);
	// the address of another site's recipe to import
	let importUrl = $state('');
	let importing = $state(false);

	// the fields of the recipe to change
	$effect(() => {
		const user = session.user;
		if (!user || started) return;
		started = true;
		if (!editing) return;
		// only its author edits a recipe, admins too (zipp would let them)
		if (live && live.authorId !== String(user.id)) {
			problem = 'notYours';
			return;
		}
		getEntry(/** @type {string} */ (recipeId))
			.then(e => {
				const img = live?.image?.[0];
				published = e.published;
				form = fromEntry(
					e,
					img && {
						id: Number(img.id),
						url: img.url,
						width: img.width,
						height: img.height,
					},
				);
			})
			.catch(e => {
				if (e instanceof MemberError && e.status === 403)
					problem = 'notYours';
				else if (e instanceof MemberError && e.status === 404)
					problem = 'missing';
				else problem = errorMessage(e);
			});
	});

	// once tried, the errors follow the input
	$effect(() => {
		if (tried && form) errors = validate(form, triedPrivate);
	});

	let shown = $derived({ ...refused, ...errors });
	let title = $derived(editing ? 'Edit Recipe' : 'New Recipe');
	let signInUrl = $derived(
		'/sign-in?next=' + encodeURIComponent($route.url.pathname),
	);
	let cancelUrl = $derived(
		live?.url ?? (editing ? `/my-recipes/${recipeId}` : '/account'),
	);
	// copies and imports start private, private ones stay so
	let privateFirst = $derived(
		!!copyOf || (!published && (editing || !!form?.source)),
	);
	// the site of an imported recipe, without www.
	let sourceSite = $derived(siteOf(form?.source ?? null));

	/** @param {string | null} url */
	function siteOf(url) {
		if (!url) return null;
		try {
			return new URL(url).hostname.replace(/^www\./, '');
		} catch {
			return null;
		}
	}

	/** Whether the member wrote something already. @param {Form} f */
	function hasContent(f) {
		return (
			!!f.title.trim() ||
			f.chapters.some(
				c => c.text.trim() || c.ingredients.some(i => i.name.trim()),
			)
		);
	}

	/** Fills the form with the recipe of another site, unsaved. */
	async function runImport() {
		let url = importUrl.trim();
		if (!form || importing || !url) return;
		if (!/^https?:\/\//i.test(url)) url = 'https://' + url;
		if (
			hasContent(form) &&
			!confirm('Replace what you have written with the imported recipe?')
		)
			return;
		importing = true;
		try {
			const r = await importRecipe(url);
			form = fromImport(r, $categories ?? []);
			tried = false;
			errors = {};
			refused = {};
			importUrl = '';
			toasts.show(
				'The recipe is imported: check it and add your own photo.',
			);
		} catch (err) {
			toasts.error(errorMessage(err));
		} finally {
			importing = false;
		}
	}

	/** Enter imports instead of saving the recipe. @param {KeyboardEvent} e */
	function importKey(e) {
		if (e.key !== 'Enter') return;
		e.preventDefault();
		runImport();
	}

	/** @param {number} i @param {-1 | 1} dir */
	function moveChapter(i, dir) {
		if (!form) return;
		const cs = form.chapters;
		[cs[i], cs[i + dir]] = [cs[i + dir], cs[i]];
	}

	/** @type {HTMLElement | undefined} */
	let chapterList = $state();
	const chapterSort = new Sortable(
		() => chapterList,
		(from, to) => {
			if (form) form.chapters = moved(form.chapters, from, to);
		},
	);

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

	/** The path of a recipe's page, of the site's url; a private one's of
	 * its author. @param {any} e */
	function pathOf(e) {
		if (!e.published) return `/my-recipes/${e.id}`;
		if (!e.uri) return '/account';
		return e.url ? new URL(e.url).pathname : '/' + e.uri;
	}

	/** Enter saves it the way of the main button. @param {SubmitEvent} e */
	function submit(e) {
		e.preventDefault();
		save(privateFirst);
	}

	/** Saves it live, or with `asPrivate` unpublished: only its author
	 * sees it. @param {boolean} asPrivate */
	async function save(asPrivate) {
		if (!form || saving) return;
		tried = true;
		triedPrivate = asPrivate;
		refused = {};
		errors = validate(form, asPrivate);
		const missing = [
			errors.image && 'a photo',
			errors.title && 'the title',
			errors.category && 'the category',
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

		saving = true;
		const snap = $state.snapshot(form);
		const fields = toFields(snap);
		const privateNote = published
			? 'The recipe is private now, only you see it.'
			: snap.image
				? 'Saved privately, only you see it.'
				: 'Saved privately, only you see it. Add your own photo to publish it.';
		try {
			const title = snap.title.trim();
			if (editing) {
				const id = Number(recipeId);
				const r = await updateEntry(id, title, fields, !asPrivate);
				forgetQueries(crelte);
				toasts.show(
					asPrivate
						? privateNote
						: r.saved === 'draft'
							? 'Your changes are saved, an editor will publish them.'
							: published
								? 'The recipe has been saved.'
								: 'The recipe has been published.',
				);
				router.open(pathOf(r.entry));
			} else {
				const created = await createRecipe(title, fields, !asPrivate);
				forgetQueries(crelte);
				toasts.show(
					asPrivate ? privateNote : 'The recipe has been published.',
				);
				router.open(pathOf(created));
			}
		} catch (err) {
			if (err instanceof MemberError) refused = fieldErrors(err.fields);
			toasts.error(errorMessage(err));
		} finally {
			saving = false;
		}
	}
</script>

<svelte:head>
	<title>{title} | Chuchi Recipes</title>
</svelte:head>

{#if !session.user}
	<!-- the home's Submit your first Recipe and Make it your own too -->
	<Join title="Share your Recipes" signIn={signInUrl} />
{:else if problem === 'notYours'}
	<div class="note narrow">
		<h1 class="h1">{title}</h1>
		<p class="lead">
			This recipe is not yours, so you cannot change it. Make it your own:
			start a recipe of yours from it.
		</p>
		<div class="buttons">
			<Button href="/recipes/copy/{recipeId}">Make it your own</Button>
			<Button href={cancelUrl} look="secondary">Back</Button>
		</div>
	</div>
{:else if problem}
	<div class="note narrow">
		<h1 class="h1">{title}</h1>
		<p class="lead">
			{problem === 'missing'
				? 'This recipe does not exist (anymore).'
				: problem}
		</p>
		<Button href="/account" look="secondary">My Recipes</Button>
	</div>
{:else if !form}
	<div class="note narrow">
		<h1 class="h1">{title}</h1>
		<p class="lead">Loading …</p>
	</div>
{:else}
	<form class="editor" onsubmit={submit} novalidate>
		<div class="bar">
			<div class="bar-inner">
				<Button href={cancelUrl} look="secondary">Cancel</Button>
				{#if !form.image}
					<p class="need-photo small">
						Add your own photo to publish
					</p>
				{/if}
				<!-- the main button saves the way the recipe is meant: a
				copy, an import or a private one privately -->
				<div class="saves">
					<Button
						look="secondary"
						disabled={saving}
						onclick={() => save(!privateFirst)}
					>
						{privateFirst ? 'Publish' : 'Save privately'}
					</Button>
					<Button type="submit" disabled={saving}>
						{saving
							? 'Saving …'
							: privateFirst
								? 'Save privately'
								: 'Publish'}
					</Button>
				</div>
			</div>
		</div>

		<div class="content narrow">
			<h1 class="h1">{title}</h1>

			{#if !editing && !copyOf}
				<div class="import">
					<label for="import-url">Import from a website</label>
					<p class="small hint">
						The address of a recipe page: we read that one page and
						write the recipe anew for you to check. Photos are not
						imported, add your own.
					</p>
					<div class="import-row">
						<input
							id="import-url"
							class="box"
							type="url"
							inputmode="url"
							autocomplete="off"
							placeholder="https://"
							bind:value={importUrl}
							onkeydown={importKey}
							disabled={importing}
						/>
						<Button
							disabled={importing || !importUrl.trim()}
							onclick={runImport}
						>
							{importing ? 'Importing …' : 'Import'}
						</Button>
					</div>
				</div>
			{/if}

			{#if sourceSite && form.source}
				<p class="lead from">
					Check the ingredients and the steps before you publish it,
					it is based on a recipe from
					<a
						href={form.source}
						target="_blank"
						rel="nofollow noopener"
					>
						{sourceSite}
					</a>
				</p>
			{/if}

			{#if copyOf && live}
				<p class="lead from">
					Based on <a href={live.url}>{live.title}</a>
					: change what you like, it becomes a private recipe of yours.
					Their photos stay theirs, add your own to publish it.
				</p>
			{/if}

			<div class="fields">
				<div class="field wide">
					<ImageField bind:image={form.image} error={shown.image} />
				</div>

				<div class="field wide">
					<label for="title">Recipe Title:</label>
					<input
						id="title"
						class="box"
						class:invalid={!!shown.title}
						type="text"
						autocomplete="off"
						bind:value={form.title}
						aria-invalid={shown.title ? true : undefined}
						aria-describedby={shown.title
							? 'title-error'
							: undefined}
					/>
					{#if shown.title}
						<p class="error" id="title-error">{shown.title}</p>
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
									value={c.id}
									bind:group={form.category}
								/>
								<span>{c.singular || c.title}</span>
							</label>
						{/each}
					</div>
					{#if shown.category}
						<p class="error">{shown.category}</p>
					{/if}
				</fieldset>

				<NumberField
					id="total"
					label="Time spent totally:"
					bind:value={form.total}
					bind:unit={form.totalUnit}
					units={TIME_UNITS}
					error={shown.total}
				/>

				<NumberField
					id="active"
					label="Time spent actively:"
					bind:value={form.active}
					bind:unit={form.activeUnit}
					units={TIME_UNITS}
					error={shown.active}
				/>

				<NumberField
					id="serves"
					label="This recipe serves:"
					step="1"
					min={1}
					error={shown.serves}
					bind:value={form.serves}
					suffix="people"
				/>
			</div>

			<h2 class="h1 instructions">Instructions</h2>
			{#if shown.chapters}
				<p class="error chapters-error" role="alert">
					{shown.chapters}
				</p>
			{/if}

			<div class="chapters" bind:this={chapterList}>
				{#each form.chapters as chapter, i (chapter.key)}
					<ChapterCard
						bind:chapter={form.chapters[i]}
						index={i}
						count={form.chapters.length}
						sortable={chapterSort}
						onmove={dir => moveChapter(i, dir)}
						onremove={() => removeChapter(i)}
					/>
				{/each}
				<SortLine sortable={chapterSort} />

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

	.buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.from {
		margin-top: 2rem;
		color: var(--accent-soft);

		a {
			@include underline;
			color: var(--accent);
		}
	}

	.editor {
		display: flex;
		flex-direction: column;
	}

	.import {
		margin-top: 2rem;
		padding: 1.25rem;
		border: 1px dashed var(--accent-soft);

		label {
			display: block;
		}
	}

	.hint {
		margin-top: 0.5rem;
		color: var(--accent-soft);
	}

	.import-row {
		display: flex;
		gap: 0.5rem;
		margin-top: 1rem;

		.box {
			flex: 1;
			min-width: 0;
		}
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
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
		padding: 0.75rem var(--sx-body);
	}

	// mobile: the note and Cancel, then both saves on a line of their own
	.saves {
		display: flex;
		width: 100%;
		gap: 0.5rem;

		> :global(*) {
			flex: 1;
		}
	}

	.need-photo {
		flex: 1;
		text-align: right;
		color: var(--accent-soft);
	}

	// a flex item with auto margins would shrink to its content, so it
	// takes the whole column
	.content {
		width: 100%;
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
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
		margin-top: 2rem;
	}

	.add-chapter {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1.25rem;
		border: 1px dashed var(--accent-soft);
		text-align: left;
		cursor: pointer;
		transition:
			border-color 0.3s ease,
			background-color 0.3s ease;
	}

	// like the chapters' buttons, without the ring
	.plus {
		display: flex;
		padding: 0.1875rem;
		border: 1px solid transparent;
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

		.need-photo {
			flex: none;
		}

		.saves {
			width: auto;
			gap: 1rem;

			> :global(*) {
				flex: none;
			}
		}

		.import {
			padding: 2rem;
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
		.from a:hover {
			opacity: 0.6;
		}

		.pill:hover input:not(:checked) + span {
			border-color: var(--accent);
		}

		.add-chapter:hover {
			border-color: var(--accent);
			background: var(--bg-card);

			.plus-inner {
				background: var(--accent);
				color: var(--text-invert);
			}
		}
	}
</style>
