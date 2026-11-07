/**
 * Dragging the items of a list by their handle (MoveButton), with the
 * mouse and on touch, like the studio of zipp: the rows are the children
 * with `data-sort` of the container, which has `position: relative` and
 * shows the line where the dragged row would go.
 */
export class Sortable {
	/** the row that is dragged */
	dragging = $state<number | null>(null);
	/** where it would go: the top of the line, relative to the container */
	line = $state<number | null>(null);
	/** a drag just ended: the click that follows isn't one */
	dragged = false;
	private to: number | null = null;

	constructor(
		private container: () => HTMLElement | undefined,
		private onmove: (from: number, to: number) => void,
	) {}

	private rows(): HTMLElement[] {
		const c = this.container();
		if (!c) return [];
		return [...c.querySelectorAll<HTMLElement>(':scope > [data-sort]')];
	}

	/** Starts dragging row `from` when the pointer moves a little. */
	start(e: PointerEvent, from: number) {
		if (e.button !== 0) return;
		const x0 = e.clientX;
		const y0 = e.clientY;
		let moving = false;
		let y = y0;
		let frame = 0;

		// near the viewport's edges the page scrolls while the pointer
		// stays, so the line follows each frame
		const scroll = () => {
			const by =
				y < SCROLL_EDGE
					? -SCROLL_STEP
					: y > innerHeight - SCROLL_EDGE
						? SCROLL_STEP
						: 0;
			if (by) {
				scrollBy(0, by);
				this.over(y, from);
			}
			frame = requestAnimationFrame(scroll);
		};

		const move = (ev: PointerEvent) => {
			if (!moving) {
				if (Math.hypot(ev.clientX - x0, ev.clientY - y0) < 5) return;
				moving = true;
				this.dragging = from;
				frame = requestAnimationFrame(scroll);
			}
			ev.preventDefault();
			y = ev.clientY;
			this.over(y, from);
		};
		const end = (ev: Event) => {
			removeEventListener('pointermove', move);
			removeEventListener('pointerup', end);
			removeEventListener('pointercancel', end);
			removeEventListener('keydown', key, true);
			cancelAnimationFrame(frame);
			const to = this.to;
			this.dragging = null;
			this.line = null;
			this.to = null;
			if (!moving) return;
			this.dragged = true;
			setTimeout(() => (this.dragged = false));
			if (to !== null && to !== from && ev.type === 'pointerup')
				this.onmove(from, to);
		};
		// Escape drops nothing
		const key = (ev: KeyboardEvent) => {
			if (ev.key !== 'Escape' || !moving) return;
			ev.preventDefault();
			ev.stopImmediatePropagation();
			this.to = null;
			end(ev);
		};
		addEventListener('pointermove', move);
		addEventListener('pointerup', end);
		addEventListener('pointercancel', end);
		addEventListener('keydown', key, true);
	}

	/** The place of a pointer at `y` over the rows, and its line. */
	private over(y: number, from: number) {
		const box = this.container()?.getBoundingClientRect();
		const others = this.rows()
			.filter((_, i) => i !== from)
			.map(el => el.getBoundingClientRect());
		if (!box || !others.length) return;

		// before the first row whose middle is below the pointer
		let k = others.findIndex(r => y < r.top + r.height / 2);
		if (k < 0) k = others.length;
		this.to = k;

		// in the middle of the gap between the rows
		const edge =
			k === 0
				? others[0].top - 4
				: k === others.length
					? others[k - 1].bottom + 4
					: (others[k - 1].bottom + others[k].top) / 2;
		this.line = edge - box.top;
	}
}

/** How near the viewport's edge a drag scrolls, and how far each frame. */
const SCROLL_EDGE = 64;
const SCROLL_STEP = 10;

/** The list with the item at `from` moved to `to`. */
export function moved<T>(items: T[], from: number, to: number): T[] {
	const next = items.filter((_, i) => i !== from);
	next.splice(to, 0, items[from]);
	return next;
}
