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
	cook: string | null;
	cookUrl: string | null;
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

export function fromZipp(entry: any): ViewRecipe {
	const cook = entry.cook?.[0] ?? null;
	return {
		id: entry.id,
		authorId: entry.authorId ?? null,
		title: entry.title,
		category: entry.category?.[0]?.singular ?? null,
		cook: cook?.title ?? null,
		cookUrl: cook?.url ?? null,
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
