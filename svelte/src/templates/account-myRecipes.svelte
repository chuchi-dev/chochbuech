<script module>
	/**
	 * @typedef {{
	 *   key: string,
	 *   title: string,
	 *   posted: string,
	 *   image: any,
	 *   view: string,
	 *   edit: string,
	 * }} Row
	 */
</script>

<script>
	import AccountLayout from '@/components/AccountLayout.svelte';
	import Button from '@/components/Button.svelte';
	import RoundArrow from '@/components/RoundArrow.svelte';
	import { getMock } from '@/lib/mock.svelte';
	import { getToasts } from '@/lib/toasts.svelte';

	let { entry } = $props();

	const mock = getMock();
	const toasts = getToasts();

	/** @type {string | null} */
	let openKey = $state(null);

	// the recipes written in this browser; the recipes of the user in zipp
	// come with entry authors
	/** @type {Row[]} */
	let rows = $derived(
		mock.recipes.map(l => ({
			key: l.id,
			title: l.title || 'Untitled',
			posted: formatDate(l.postedAt),
			image: null,
			view: `/my-recipes/${l.id}`,
			edit: `/recipes/edit/${l.id}`,
		})),
	);

	/** `July 29th, 2024` like zipp's postDate @param {number} ms */
	function formatDate(ms) {
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

	/** @param {Row} row */
	function remove(row) {
		if (!confirm(`Delete “${row.title}”? This cannot be undone.`)) return;
		mock.deleteRecipe(row.key);
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

{#snippet actions(/** @type {Row} */ row)}
	<a href={row.view}>View</a>
	<a href={row.edit}>Edit</a>
	<button type="button" onclick={() => remove(row)}>Delete</button>
{/snippet}

<AccountLayout title={entry.title}>
	{#snippet intro()}
		<Button href="/recipes/new">New Recipe</Button>
	{/snippet}

	<div class="narrow">
		{#if !rows.length}
			<p class="empty">
				{mock.localLoaded ? 'No recipes yet.' : 'Loading …'}
				{#if mock.localLoaded}<a href="/recipes/new">
						Write your first one.
					</a>{/if}
			</p>
		{:else}
			<table class="table">
				<thead>
					<tr>
						<th class="col-image">Image</th>
						<th>Title</th>
						<th class="col-date">Posted on</th>
						<th class="col-actions">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each rows as row (row.key)}
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
								<a href={row.view}>{row.title}</a>
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
				{#each rows as row (row.key)}
					{@const open = openKey === row.key}
					<li>
						<button
							type="button"
							class="toggle"
							aria-expanded={open}
							aria-controls="recipe-{row.key}"
							onclick={() => (openKey = open ? null : row.key)}
						>
							<span>{row.title}</span>
							<RoundArrow {open} />
						</button>
						{#if open}
							<div class="details" id="recipe-{row.key}">
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
