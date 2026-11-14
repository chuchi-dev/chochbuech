<script module>
	/**
	 * @typedef {{
	 *   id: number,
	 *   title: string,
	 *   posted: string,
	 *   image: any,
	 *   view: string | null,
	 *   edit: string,
	 *   private: boolean,
	 *   entry: import('@/lib/members').MemberEntry,
	 * }} Row
	 */
</script>

<script>
	import { getCrelte } from 'crelte';
	import imagesQuery from '@/queries/recipesById.graphql';
	import AccountLayout from '@/components/AccountLayout.svelte';
	import Button from '@/components/Button.svelte';
	import Pills from '@/components/Pills.svelte';
	import RoundArrow from '@/components/RoundArrow.svelte';
	import {
		deleteEntry,
		errorMessage,
		forgetQueries,
		getImages,
		listRecipes,
		publishError,
		setPublished,
	} from '@/lib/members';
	import { getSession } from '@/lib/session.svelte';
	import { getToasts } from '@/lib/toasts.svelte';

	// the recipes of the user in zipp (they are its author), loaded in the
	// browser through /members: public ones and private ones (unpublished,
	// only they see them, on /my-recipes/<id>)
	let { entry } = $props();

	const crelte = getCrelte();
	const session = getSession();
	const toasts = getToasts();

	let rows = $state(/** @type {Row[] | null} */ (null));
	/** @type {string | null} */
	let failed = $state(null);
	/** @type {string | null} */
	let openKey = $state(null);
	let filter = $state('all');
	let busy = $state(false);

	const FILTERS = [
		{ key: 'all', label: 'All' },
		{ key: 'private', label: 'Private' },
		{ key: 'public', label: 'Public' },
	];

	let shown = $derived(
		(rows ?? []).filter(
			r => filter === 'all' || (filter === 'private') === r.private,
		),
	);

	$effect(() => {
		if (!session.user) return;
		load();
	});

	async function load() {
		rows = null;
		failed = null;
		try {
			const { entries } = await listRecipes();
			// the images of the live recipes, zipp gives asset ids; the
			// private ones aren't in GraphQL, their photos come by id
			const ids = entries.filter(e => e.published).map(e => e.id);
			/** @type {any[]} */
			let live = [];
			if (ids.length) {
				const data = /** @type {any} */ (
					await crelte.query(imagesQuery, { ids })
				);
				live = data.recipes ?? [];
			}
			const photos = await getImages(
				entries
					.filter(e => !e.published && e.fields.image?.length)
					.map(e => e.fields.image[0]),
			);
			rows = entries.map(e => {
				const r = live.find(r => r.id === String(e.id));
				const photo = photos.find(p => p.id === e.fields.image?.[0]);
				return {
					id: e.id,
					title: e.title || 'Untitled',
					posted: e.published
						? (r?.postDate ?? formatDate(e.postDate))
						: 'Only you see it',
					private: !e.published,
					image: e.published ? (r?.image?.[0] ?? null) : photo,
					view: e.published
						? (r?.url ?? null)
						: `/my-recipes/${e.id}`,
					edit: `/recipes/edit/${e.id}`,
					entry: e,
				};
			});
		} catch (e) {
			failed = errorMessage(e);
		}
	}

	/** `July 29th, 2024` like zipp's postDate @param {number | null} ms */
	function formatDate(ms) {
		if (!ms) return '';
		const d = new Date(ms);
		const day = d.getDate();
		const th =
			day % 10 === 1 && day !== 11
				? 'st'
				: day % 10 === 2 && day !== 12
					? 'nd'
					: day % 10 === 3 && day !== 13
						? 'rd'
						: 'th';
		const month = d.toLocaleString('en', { month: 'long' });
		return `${month} ${day}${th}, ${d.getFullYear()}`;
	}

	/** Publishes a private recipe or makes a public one private. @param {Row} row */
	async function toggle(row) {
		if (busy) return;
		busy = true;
		try {
			await setPublished(row.entry, row.private);
			forgetQueries(crelte);
			toasts.show(
				row.private
					? 'The recipe has been published.'
					: 'The recipe is private now, only you see it.',
			);
			await load();
		} catch (e) {
			toasts.error(publishError(e));
		} finally {
			busy = false;
		}
	}

	/** @param {Row} row */
	async function remove(row) {
		if (!confirm(`Delete “${row.title}”? This cannot be undone.`)) return;
		try {
			await deleteEntry(row.id);
			forgetQueries(crelte);
		} catch (e) {
			toasts.error(errorMessage(e));
			return;
		}
		rows = rows?.filter(r => r.id !== row.id) ?? null;
		toasts.show('The recipe has been deleted successfully.');
	}
</script>

{#snippet thumb(/** @type {Row} */ row, /** @type {string} */ sizes)}
	{#if row.image}
		<img
			src={row.image.url}
			srcset={row.image.srcset}
			{sizes}
			width={row.image.width}
			height={row.image.height}
			alt={row.image.alt ?? ''}
			loading="lazy"
		/>
	{:else}
		<span class="placeholder" aria-hidden="true"></span>
	{/if}
{/snippet}

{#snippet badge()}
	<span class="badge small">Private</span>
{/snippet}

{#snippet actions(/** @type {Row} */ row)}
	{#if row.view}<a href={row.view}>View</a>{/if}
	<a href={row.edit}>Edit</a>
	<button type="button" disabled={busy} onclick={() => toggle(row)}>
		{row.private ? 'Publish' : 'Make private'}
	</button>
	<button type="button" onclick={() => remove(row)}>Delete</button>
{/snippet}

<AccountLayout title={entry.title}>
	{#snippet intro()}
		<Button href="/recipes/new">New Recipe</Button>
	{/snippet}

	<div class="narrow">
		{#if failed}
			<p class="empty">{failed}</p>
		{:else if !rows}
			<p class="empty">Loading …</p>
		{:else if !rows.length}
			<p class="empty">
				No recipes yet.
				<a href="/recipes/new">Write your first one.</a>
			</p>
		{:else}
			<div class="filters">
				<Pills
					items={FILTERS}
					active={filter}
					label="Show"
					onselect={k => (filter = k)}
				/>
			</div>
			{#if !shown.length}
				<p class="empty">
					{filter === 'private'
						? 'No private recipes.'
						: 'No public recipes yet.'}
				</p>
			{/if}
			<table class="table" class:hidden={!shown.length}>
				<thead>
					<tr>
						<th class="col-image">Image</th>
						<th>Title</th>
						<th class="col-date">Posted on</th>
						<th class="col-actions">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each shown as row (row.id)}
						<tr>
							<td class="col-image">
								<a
									href={row.view}
									class="thumb"
									tabindex="-1"
									aria-hidden="true"
								>
									{@render thumb(row, '8rem')}
								</a>
							</td>
							<td class="title">
								{#if row.view}
									<a href={row.view}>{row.title}</a>
								{:else}
									{row.title}
								{/if}
								{#if row.private}{@render badge()}{/if}
							</td>
							<td class="date">{row.posted}</td>
							<td>
								<div class="actions">
									{@render actions(row)}
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>

			<ul class="accordion">
				{#each shown as row (row.id)}
					{@const open = openKey === String(row.id)}
					<li>
						<button
							type="button"
							class="toggle"
							aria-expanded={open}
							aria-controls="recipe-{row.id}"
							onclick={() =>
								(openKey = open ? null : String(row.id))}
						>
							<span>
								{row.title}
								{#if row.private}{@render badge()}{/if}
							</span>
							<RoundArrow {open} />
						</button>
						{#if open}
							<div class="details" id="recipe-{row.id}">
								<p class="posted">Posted on: {row.posted}</p>
								<div class="actions">
									{@render actions(row)}
								</div>
								<a
									href={row.view}
									class="image"
									tabindex="-1"
									aria-hidden="true"
								>
									{@render thumb(row, '100vw')}
								</a>
							</div>
						{/if}
					</li>
				{/each}
			</ul>
		{/if}
	</div>
</AccountLayout>

<style lang="scss">
	.filters {
		margin-bottom: 2rem;
	}

	.hidden {
		display: none;
	}

	.badge {
		display: inline-block;
		margin-left: 0.5rem;
		padding: 0 0.5rem;
		border: 1px solid var(--accent-soft);
		border-radius: 999px;
		color: var(--accent-soft);
		vertical-align: middle;
		white-space: nowrap;
	}

	.empty {
		color: var(--accent-soft);
		text-align: center;

		a {
			@include underline;
			color: var(--accent);
		}
	}

	.table {
		display: none;
	}

	.accordion {
		list-style: none;
		border-top: 1px solid var(--line);

		li {
			border-bottom: 1px solid var(--line);
		}
	}

	.toggle {
		display: flex;
		width: 100%;
		min-height: 4rem;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding-block: 0.75rem;
		text-align: left;
		cursor: pointer;
	}

	.details {
		padding-bottom: 1.25rem;
	}

	.posted {
		color: var(--accent-soft);
	}

	.actions {
		display: flex;
		gap: 1rem;
		margin-top: 1.25rem;

		a,
		button {
			@include underline;
			cursor: pointer;
		}
	}

	.image {
		display: block;
		margin-top: 2rem;
		aspect-ratio: 4 / 3;
		overflow: hidden;
	}

	img,
	.placeholder {
		display: block;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.placeholder {
		background: var(--line);
	}

	@include desktop {
		.accordion {
			display: none;
		}

		.table {
			display: table;
			width: 100%;
			border-collapse: collapse;
			table-layout: fixed;
		}

		th {
			padding-bottom: 0.75rem;
			border-bottom: 1px solid var(--line);
			font-weight: inherit;
			text-align: left;
			opacity: 0.5;
		}

		td {
			padding-block: 1.5rem;
			border-bottom: 1px solid var(--line);
			vertical-align: top;
		}

		th,
		td {
			padding-right: 2.5rem;

			&:last-child {
				padding-right: 0;
			}
		}

		.col-image {
			width: 10.5rem;
		}

		.col-date {
			width: 18.5rem;
		}

		.col-actions {
			width: 8rem;
		}

		.thumb {
			display: block;
			width: 8rem;
			aspect-ratio: 3 / 4;
			overflow: hidden;
		}

		.title,
		.date {
			@include lead;
		}

		.actions {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.5rem;
			margin-top: 0.25rem;
		}
	}

	@include hover {
		.actions a:hover,
		.actions button:hover,
		.title a:hover,
		.empty a:hover {
			opacity: 0.6;
		}
	}
</style>
