// the state of the recipe editor and its way to and from the fields of a
// zipp recipe (lib/members.ts)
import type { FieldError, ImportedRecipe, MemberEntry } from '@/lib/members';
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
	// the block's id in zipp, kept on a change
	id: string | null;
	title: string;
	ingredients: FormIngredient[];
	// the steps, separated by blank lines
	text: string;
	// the rich text it came from: unchanged steps keep their marks and
	// links, the step images stay after their step
	nodes: Node[];
	open: boolean;
};

export type FormImage = {
	id: number;
	url: string;
	width: number | null;
	height: number | null;
};

export type Form = {
	title: string;
	// the id of the category entry
	category: string | null;
	image: FormImage | null;
	total: number | null;
	totalUnit: TimeUnit;
	active: number | null;
	activeUnit: TimeUnit;
	serves: number | null;
	chapters: FormChapter[];
	// the page of another site it is based on (an import)
	source: string | null;
};

export type Errors = {
	title?: string;
	category?: string;
	image?: string;
	total?: string;
	active?: string;
	serves?: string;
	chapters?: string;
};

// a node of TipTap's JSON, how zipp stores rich text
type Node = { type: string; [k: string]: any };

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
		id: null,
		title: '',
		ingredients: [newIngredient()],
		text: '',
		nodes: [],
		open: true,
	};
}

export function emptyForm(): Form {
	return {
		title: '',
		category: null,
		image: null,
		total: null,
		totalUnit: 'hours',
		active: null,
		activeUnit: 'minutes',
		serves: 4,
		chapters: [newChapter()],
		source: null,
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

/** The plain text of a paragraph, its line breaks as `\n`. */
function textOf(p: Node): string {
	return (p.content ?? [])
		.map((n: Node) =>
			n.type === 'text' ? n.text : n.type === 'hardBreak' ? '\n' : '',
		)
		.join('')
		.trim();
}

function paragraph(text: string): Node {
	const content: Node[] = [];
	text.split('\n').forEach((line, i) => {
		if (i) content.push({ type: 'hardBreak' });
		if (line) content.push({ type: 'text', text: line });
	});
	return { type: 'paragraph', content };
}

function chapterToForm(
	id: string | null,
	title: string,
	ingredients: any[],
	nodes: Node[],
	open: boolean,
): FormChapter {
	return {
		key: newKey(),
		id,
		title: title ?? '',
		ingredients: (ingredients ?? []).map(ing => ({
			key: newKey(),
			name: ing.name ?? '',
			amount: ing.amount ?? null,
			unit: ing.unit ?? '',
		})),
		text: nodes
			.filter(n => n.type === 'paragraph')
			.map(textOf)
			.filter(Boolean)
			.join('\n\n'),
		nodes,
		open,
	};
}

function withChapters(form: Form, chapters: FormChapter[]): Form {
	form.chapters = chapters.length ? chapters : [newChapter()];
	return form;
}

/** A recipe of the user to change (GET /members/entries/{id}), `image`
 * the live one of GraphQL for the preview. */
export function fromEntry(e: MemberEntry, image: FormImage | null): Form {
	const f = e.fields;
	const form: Form = {
		title: e.title,
		category: f.category?.[0] != null ? String(f.category[0]) : null,
		image: imageOf(f.image?.[0] ?? null, image),
		...withUnits(f.totalTime ?? null, f.activeTime ?? null),
		serves: f.serves ?? 4,
		chapters: [],
		source: f.source?.value ?? null,
	};
	const chapters = (f.chapters ?? []).map((b: any, i: number) =>
		chapterToForm(
			b.id ?? null,
			b.fields?.title,
			b.fields?.ingredients,
			b.fields?.steps?.content ?? [],
			i === 0,
		),
	);
	return withChapters(form, chapters);
}

/** The image of the field; its preview only if it is the live one. */
function imageOf(id: number | null, live: FormImage | null): FormImage | null {
	if (id === null) return null;
	if (live?.id === id) return live;
	return { id, url: '', width: null, height: null };
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

/** Someone else's recipe (recipeForEdit.graphql) as the start of a new
 * one: the text without its marks and the site it is based on, never
 * their photos (the member adds their own to publish it). */
export function fromGraphql(r: any): Form {
	const form: Form = {
		title: r.title ?? '',
		category: r.category?.[0]?.id ?? null,
		image: null,
		...withUnits(r.totalTime ?? null, r.activeTime ?? null),
		serves: r.serves ?? 4,
		chapters: [],
		source: r.source?.url ?? null,
	};
	const chapters = (r.chapters ?? []).map((c: any, i: number) => {
		const nodes: Node[] = [];
		for (const chunk of c.steps?.chunks ?? []) {
			if (chunk.html === undefined) continue;
			for (const p of paragraphs(chunk.html)) {
				const text = plain(p);
				if (text) nodes.push(paragraph(text));
			}
		}
		return chapterToForm(null, c.title, c.ingredients, nodes, i === 0);
	});
	return withChapters(form, chapters);
}

/** A recipe of another site (`importRecipe`) as a new one, without a
 * photo: the member adds their own. `categories` are the site's, the
 * import names them by slug. */
export function fromImport(
	r: ImportedRecipe,
	categories: { id: string; slug: string }[],
): Form {
	const form: Form = {
		title: r.title,
		category: categories.find(c => c.slug === r.category)?.id ?? null,
		image: null,
		...withUnits(r.totalTime, r.activeTime),
		serves: r.serves ?? 4,
		chapters: [],
		source: r.source,
	};
	const chapters = r.chapters.map((c, i) => ({
		key: newKey(),
		id: null,
		title: c.title,
		ingredients: c.ingredients.map(ing => ({
			key: newKey(),
			name: ing.name,
			amount: ing.amount,
			unit: ing.unit ?? '',
		})),
		text: c.steps.join('\n\n'),
		nodes: [],
		open: i === 0,
	}));
	return withChapters(form, chapters);
}

function withUnits(totalTime: number | null, activeTime: number | null) {
	const [total, totalUnit] = fromMinutes(totalTime);
	const [active, activeUnit] = fromMinutes(activeTime);
	return { total, totalUnit, active, activeUnit };
}

function stepsOf(text: string): string[] {
	return text
		.split(/\n\s*\n/)
		.map(s => s.trim())
		.filter(Boolean);
}

/**
 * The rich text of a chapter's steps: a paragraph per step. A step whose
 * text didn't change keeps its paragraph (marks, links) and the images
 * after it; the images of a changed or removed step stay at its place.
 */
export function stepsDoc(text: string, nodes: Node[]): Node {
	// the original paragraphs, each with the images after it; the images
	// before the first paragraph go first
	const lead: Node[] = [];
	const groups: { text: string; para: Node; embeds: Node[] }[] = [];
	for (const n of nodes) {
		if (n.type === 'paragraph') {
			const t = textOf(n);
			if (t) groups.push({ text: t, para: n, embeds: [] });
		} else if (groups.length) {
			groups[groups.length - 1].embeds.push(n);
		} else {
			lead.push(n);
		}
	}

	const steps = stepsOf(text);
	const used = new Set<number>();
	const match = steps.map(s => {
		const g = groups.findIndex((g, j) => !used.has(j) && g.text === s);
		if (g >= 0) used.add(g);
		return g;
	});
	// the images of steps that changed: after the step at their place
	const moved: Node[][] = steps.map(() => []);
	const tail: Node[] = [];
	groups.forEach((g, j) => {
		if (used.has(j) || !g.embeds.length) return;
		if (steps.length)
			moved[Math.min(j, steps.length - 1)].push(...g.embeds);
		else tail.push(...g.embeds);
	});

	const content: Node[] = [...lead];
	steps.forEach((s, i) => {
		const g = match[i] >= 0 ? groups[match[i]] : null;
		content.push(
			g ? g.para : paragraph(s),
			...(g?.embeds ?? []),
			...moved[i],
		);
	});
	content.push(...tail);
	return { type: 'doc', content };
}

function chapterBlocks(form: Form) {
	return form.chapters
		.map(c => {
			const ingredients = c.ingredients
				.filter(i => i.name.trim())
				.map(i => {
					const row: Record<string, unknown> = {
						name: i.name.trim(),
					};
					if (typeof i.amount === 'number' && !isNaN(i.amount))
						row.amount = i.amount;
					if (i.unit) row.unit = i.unit;
					return row;
				});
			const steps = stepsDoc(c.text, c.nodes);
			const block: Record<string, unknown> = {
				type: 'chapter',
				fields: { title: c.title.trim(), ingredients, steps },
			};
			if (c.id) block.id = c.id;
			return block;
		})
		.filter(b => {
			const f = b.fields as { ingredients: unknown[]; steps: Node };
			return f.ingredients.length || f.steps.content.length;
		});
}

/** What is missing to publish, or with `draft` to save it unpublished:
 * a draft needs only its title. */
export function validate(form: Form, draft = false): Errors {
	const errors: Errors = {};
	if (!form.title.trim()) errors.title = 'Please give your recipe a title.';
	if (draft) return errors;
	if (!form.image) errors.image = 'Add your own photo to publish.';
	if (!form.category) errors.category = 'Please choose a category.';
	if (!form.serves || form.serves < 1)
		errors.serves = 'For how many people is it?';
	if (!chapterBlocks(form).length)
		errors.chapters = 'Add at least one ingredient or step.';
	return errors;
}

/** The fields of the recipe for zipp; its cook is its author. */
export function toFields(form: Form): Record<string, unknown> {
	return {
		category: form.category ? [Number(form.category)] : [],
		image: form.image ? [form.image.id] : [],
		serves: Math.round(form.serves ?? 1),
		totalTime: toMinutes(form.total, form.totalUnit),
		activeTime: toMinutes(form.active, form.activeUnit),
		chapters: chapterBlocks(form),
		source: form.source ? { type: 'url', value: form.source } : null,
	};
}

/** zipp's field errors (their paths) at the fields of the form. */
export function fieldErrors(list: FieldError[]): Errors {
	const errors: Errors = {};
	const keys: Record<string, keyof Errors> = {
		title: 'title',
		category: 'category',
		image: 'image',
		serves: 'serves',
		totalTime: 'total',
		activeTime: 'active',
		chapters: 'chapters',
	};
	for (const e of list) {
		// zipp's slugs are the titles, unique in the section (private
		// recipes too)
		if (e.path === 'slug' && e.message.includes('already used')) {
			errors.title ??=
				'There is already a recipe with this title, please give yours another one.';
			continue;
		}
		const key = keys[e.path.split('.')[0]] ?? 'chapters';
		errors[key] ??= e.message;
	}
	return errors;
}
