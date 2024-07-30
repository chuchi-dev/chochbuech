import type { Router } from 'chuchi';
import NotFound from './NotFound.svelte';

export { NotFound };

export function register(router: Router) {
	router.register('/', () => import('./Home.svelte'));
	router.register('/sub-page', () => import('./SubPage.svelte'));
}
