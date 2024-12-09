import type { Router } from 'chuchi';
import NotFound from './NotFound.svelte';

export { NotFound };

export function register(router: Router) {
	router.register('/', () => import('./Index.svelte'));
	router.register('/anmelden', () => import('./SignIn.svelte'));
	router.register('/erstellen', () => import('./New.svelte'));
	router.register('/was-ist-chochbuech', () => import('./About.svelte'));

	router.register('/ich', () => import('./me/Index.svelte'));
}
