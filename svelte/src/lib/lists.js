// the recipes of the lists, 12 per page and Load More
export const PAGE = 12;

/**
 * The id of a category by its slug, from the global `categories`.
 *
 * @param {any[] | null} categories
 * @param {string | null} slug
 */
export function categoryId(categories, slug) {
	if (!slug) return null;
	return categories?.find(c => c.slug === slug)?.id ?? null;
}

/**
 * The pills of the category filter: All and the categories.
 *
 * @param {any[] | null} categories
 * @param {(slug: string | null) => string | null | undefined} [href]
 */
export function categoryPills(categories, href) {
	return [
		{ key: '', label: 'All', href: href?.(null) ?? undefined },
		...(categories ?? []).map(c => ({
			key: c.slug,
			label: c.title,
			href: href?.(c.slug) ?? undefined,
		})),
	];
}
