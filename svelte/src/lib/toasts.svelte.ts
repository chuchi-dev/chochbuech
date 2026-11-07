// the notifications of the design: blue for done, red for an error
import { getContext, setContext } from 'svelte';

export type Toast = { id: number; text: string; error: boolean };

export class Toasts {
	list: Toast[] = $state([]);
	private next = 1;

	show(text: string, error = false) {
		const id = this.next++;
		this.list = [...this.list, { id, text, error }];
		setTimeout(() => this.close(id), 5000);
	}

	error(text: string) {
		this.show(text, true);
	}

	close(id: number) {
		this.list = this.list.filter(t => t.id !== id);
	}
}

const KEY = Symbol('toasts');

export function setToasts(): Toasts {
	return setContext(KEY, new Toasts());
}

export function getToasts(): Toasts {
	return getContext(KEY);
}
