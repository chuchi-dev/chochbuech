// helpers for the recipes of zipp and of the editor

export const UNITS: [string, string][] = [
	['piece', 'pieces'],
	['g', 'grams'],
	['kg', 'kilograms'],
	['ml', 'milliliters'],
	['cl', 'centiliters'],
	['dl', 'deciliters'],
	['l', 'liters'],
	['tsp', 'teaspoons'],
	['tbsp', 'tablespoons'],
	['pinch', 'pinch'],
];

/** `2.5 dl`, `4` for pieces, nothing without an amount. */
export function formatAmount(
	amount: number | null,
	unit: string | null,
	factor = 1,
): string {
	if (amount === null || amount === undefined) return '';
	const n = Math.round(amount * factor * 100) / 100;
	const num = n.toLocaleString('en', { maximumFractionDigits: 2 });
	if (!unit || unit === 'piece') return num;
	return `${num} ${unit}`;
}

/** The paragraphs of an html chunk, one per step. */
export function paragraphs(html: string): string[] {
	return html
		.split(/<\/p>/)
		.map(p => p.replace(/^\s*<p[^>]*>/, '').trim())
		.filter(p => p);
}

/** `1 h 30 min` from minutes. */
export function formatTime(min: number | null): string {
	if (!min) return '';
	const h = Math.floor(min / 60);
	const m = min % 60;
	return [h ? `${h} h` : '', m ? `${m} min` : ''].filter(Boolean).join(' ');
}

/** What RecipeView shows. */
export type ViewRecipe = {
	// the zipp id, for favorites and ratings
	id: string;
	// the user who wrote it, they and admins edit it
	authorId: string | null;
	title: string;
	category: string | null;
	// the author's name and page, Anonymous without a public author
	cook: string | null;
	cookUrl: string | null;
	// the page of another site it is based on and that site, without www.
	source: { url: string; site: string } | null;
	rating: number | null;
	serves: number;
	totalTime: number | null;
	activeTime: number | null;
	image: any | null;
	chapters: {
		title: string;
		ingredients: { amount: number | null; unit: string; name: string }[];
		steps: { html: string; images: any[] }[];
	}[];
};

/** The steps of a chapter's rich text: a paragraph each, images join the
 * step before them. */
function zippSteps(chunks: any[] | undefined) {
	const steps: { html: string; images: any[] }[] = [];
	for (const chunk of chunks ?? []) {
		if (chunk.html !== undefined) {
			for (const html of paragraphs(chunk.html)) {
				steps.push({ html, images: [] });
			}
		} else if (chunk.image?.length) {
			if (!steps.length) steps.push({ html: '', images: [] });
			steps[steps.length - 1].images.push(...chunk.image);
		}
	}
	return steps;
}

/** The page an imported recipe is based on, only of another site. */
function sourceOf(url: string | null | undefined) {
	if (!url) return null;
	try {
		const u = new URL(url);
		if (u.protocol !== 'https:' && u.protocol !== 'http:') return null;
		return { url: u.href, site: u.hostname.replace(/^www\./, '') };
	} catch {
		return null;
	}
}

type Node = { type: string; [k: string]: any };

const escape = (s: string) =>
	s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');

/** The html of a paragraph of TipTap's JSON, its text without marks. */
function paragraphHtml(p: Node): string {
	return (p.content ?? [])
		.map((n: Node) =>
			n.type === 'text'
				? escape(n.text ?? '')
				: n.type === 'hardBreak'
					? '<br>'
					: '',
		)
		.join('');
}

/** The image ids of a member's recipe (its photo and step images), to
 * load with `getImages`. */
export function imageIds(fields: Record<string, any>): number[] {
	const ids: number[] = [...(fields.image ?? [])];
	for (const c of fields.chapters ?? []) {
		for (const n of c.fields?.steps?.content ?? []) {
			if (n.type === 'embed') ids.push(...(n.attrs?.fields?.image ?? []));
		}
	}
	return ids;
}

/**
 * A recipe of the user from /members (a private one isn't in GraphQL):
 * its fields, the images by id and the category by id.
 */
export function fromMember(
	entry: {
		id: number;
		title: string;
		fields: Record<string, any>;
	},
	images: Map<
		number,
		{ url: string; width: number | null; height: number | null }
	>,
	categories: { id: string; singular?: string; title: string }[],
	cook: string | null,
): ViewRecipe {
	const f = entry.fields;
	const img = (id: number) => {
		const i = images.get(id);
		return i ? { url: i.url, width: i.width, height: i.height } : null;
	};
	const category = categories.find(c => c.id === String(f.category?.[0]));
	return {
		id: String(entry.id),
		authorId: null,
		title: entry.title,
		category: category ? category.singular || category.title : null,
		cook,
		cookUrl: null,
		source: sourceOf(f.source?.value),
		rating: null,
		serves: f.serves ?? 4,
		totalTime: f.totalTime ?? null,
		activeTime: f.activeTime ?? null,
		image: f.image?.[0] != null ? img(f.image[0]) : null,
		chapters: (f.chapters ?? []).map((c: any) => {
			const steps: { html: string; images: any[] }[] = [];
			for (const n of (c.fields?.steps?.content ?? []) as Node[]) {
				if (n.type === 'paragraph') {
					const html = paragraphHtml(n);
					if (html) steps.push({ html, images: [] });
				} else if (n.type === 'embed') {
					const found = (n.attrs?.fields?.image ?? [])
						.map(img)
						.filter(Boolean);
					if (!steps.length) steps.push({ html: '', images: [] });
					steps[steps.length - 1].images.push(...found);
				}
			}
			return {
				title: c.fields?.title ?? '',
				ingredients: c.fields?.ingredients ?? [],
				steps,
			};
		}),
	};
}

export function fromZipp(entry: any): ViewRecipe {
	// the author is the cook while they are public (have a page) and
	// have a name
	const cook = entry.author?.url && entry.author.name ? entry.author : null;
	return {
		id: entry.id,
		authorId: entry.authorId ?? null,
		title: entry.title,
		category: entry.category?.[0]?.singular ?? null,
		cook: cook?.name ?? null,
		cookUrl: cook?.url ?? null,
		source: sourceOf(entry.source?.url),
		rating: entry.rating,
		serves: entry.serves ?? 4,
		totalTime: entry.totalTime,
		activeTime: entry.activeTime,
		image: entry.image?.[0] ?? null,
		chapters: (entry.chapters ?? []).map((c: any) => ({
			title: c.title ?? '',
			ingredients: c.ingredients ?? [],
			steps: zippSteps(c.steps?.chunks),
		})),
	};
}
