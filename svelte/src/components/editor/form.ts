// the state of the recipe editor and its way to and from a LocalRecipe
import type { Chapter, LocalRecipe } from '@/lib/mock.svelte';
import { paragraphs } from '@/lib/recipe';

export type TimeUnit = 'hours' | 'minutes';

export type FormIngredient = {
	key: number;
	name: string;
	amount: number | null;
	unit: string;
};

export type FormChapter = {
	key: number;
	title: string;
	ingredients: FormIngredient[];
	// the steps, separated by blank lines
	text: string;
	open: boolean;
};

export type Form = {
	id: string;
	basedOn: string | null;
	postedAt: number | null;
	title: string;
	category: string | null;
	total: number | null;
	totalUnit: TimeUnit;
	active: number | null;
	activeUnit: TimeUnit;
	serves: number | null;
	chapters: FormChapter[];
};

export type Errors = { title?: string; serves?: string; chapters?: string };

export const TIME_UNITS: [TimeUnit, string][] = [
	['hours', 'hours'],
	['minutes', 'minutes'],
];

let nextKey = 1;
export const newKey = () => nextKey++;

export function newIngredient(): FormIngredient {
	return { key: newKey(), name: '', amount: null, unit: 'g' };
}

export function newChapter(): FormChapter {
	return {
		key: newKey(),
		title: '',
		ingredients: [newIngredient()],
		text: '',
		open: true,
	};
}

export function emptyForm(): Form {
	return {
		id: '',
		basedOn: null,
		postedAt: null,
		title: '',
		category: null,
		total: null,
		totalUnit: 'hours',
		active: null,
		activeUnit: 'minutes',
		serves: 4,
		chapters: [newChapter()],
	};
}

/** Whole hours are shown in hours, the rest in minutes. */
function fromMinutes(min: number | null): [number | null, TimeUnit] {
	if (!min) return [null, 'minutes'];
	if (min % 60 === 0) return [min / 60, 'hours'];
	return [min, 'minutes'];
}

function toMinutes(value: number | null, unit: TimeUnit): number | null {
	if (!value || value < 0) return null;
	return Math.round(unit === 'hours' ? value * 60 : value);
}

function chaptersToForm(chapters: Chapter[]): FormChapter[] {
	const list = chapters.map((c, i) => ({
		key: newKey(),
		title: c.title ?? '',
		ingredients: c.ingredients.map(ing => ({
			key: newKey(),
			name: ing.name ?? '',
			amount: ing.amount ?? null,
			unit: ing.unit ?? '',
		})),
		text: c.steps.join('\n\n'),
		open: i === 0,
	}));
	return list.length ? list : [newChapter()];
}

export function fromLocal(r: LocalRecipe): Form {
	const [total, totalUnit] = fromMinutes(r.totalTime);
	const [active, activeUnit] = fromMinutes(r.activeTime);
	return {
		id: r.id,
		basedOn: r.basedOn,
		postedAt: r.postedAt,
		title: r.title,
		category: r.category,
		total,
		totalUnit,
		active,
		activeUnit,
		serves: r.serves,
		chapters: chaptersToForm(r.chapters),
	};
}

const ENTITIES: Record<string, string> = {
	amp: '&',
	lt: '<',
	gt: '>',
	quot: '"',
	apos: "'",
	nbsp: ' ',
};

/** The plain text of a paragraph of html. */
function plain(html: string): string {
	return html
		.replace(/<br\s*\/?>/g, '\n')
		.replace(/<[^>]+>/g, '')
		.replace(/&(#\d+|[a-z]+);/g, (m, e: string) =>
			e[0] === '#'
				? String.fromCharCode(parseInt(e.slice(1), 10))
				: (ENTITIES[e] ?? m),
		)
		.trim();
}

/** A recipe of zipp (recipeForEdit.graphql) as a new local copy. */

export function fromZipp(r: any): Form {
	const chapters: Chapter[] = (r.chapters ?? []).map((c: any) => ({
		title: c.title ?? '',
		ingredients: c.ingredients ?? [],
		steps: (c.steps?.chunks ?? [])
			.filter((ch: { html?: string } | null) => ch?.html)
			.flatMap((ch: { html: string }) => paragraphs(ch.html))
			.map(plain)
			.filter(Boolean),
	}));
	return fromLocal({
		id: '',
		basedOn: String(r.id),
		postedAt: r.postDate ? Number(r.postDate) * 1000 : Date.now(),
		title: r.title ?? '',
		category: r.category?.[0]?.slug ?? null,
		totalTime: r.totalTime ?? null,
		activeTime: r.activeTime ?? null,
		serves: r.serves ?? 4,
		chapters,
	});
}

function cleanChapters(form: Form): Chapter[] {
	return form.chapters
		.map(c => ({
			title: c.title.trim(),
			ingredients: c.ingredients
				.filter(i => i.name.trim())
				.map(i => ({
					name: i.name.trim(),
					amount:
						typeof i.amount === 'number' && !isNaN(i.amount)
							? i.amount
							: null,
					unit: i.unit,
				})),
			steps: c.text
				.split(/\n\s*\n/)
				.map(s => s.trim())
				.filter(Boolean),
		}))
		.filter(c => c.ingredients.length || c.steps.length);
}

export function validate(form: Form): Errors {
	const errors: Errors = {};
	if (!form.title.trim()) errors.title = 'Please give your recipe a title.';
	if (!form.serves || form.serves < 1)
		errors.serves = 'For how many people is it?';
	const chapters = cleanChapters(form);
	if (!chapters.some(c => c.ingredients.length || c.steps.length))
		errors.chapters = 'Add at least one ingredient or step.';
	return errors;
}

export function toLocal(form: Form): LocalRecipe {
	return {
		id: form.id,
		basedOn: form.basedOn,
		title: form.title.trim(),
		category: form.category,
		totalTime: toMinutes(form.total, form.totalUnit),
		activeTime: toMinutes(form.active, form.activeUnit),
		serves: Math.round(form.serves ?? 1),
		chapters: cleanChapters(form),
		postedAt: form.postedAt ?? Date.now(),
	};
}
