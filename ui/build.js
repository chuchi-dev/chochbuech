import fs from 'fs/promises';
import command from 'node:child_process';
import util from 'node:util';

const exec = util.promisify(command.exec);

async function main() {
	// const env = await fs.readFile('./.env.production', { encoding: 'utf-8' });
	// if (env.includes('localhost')) return console.warn('change env');

	console.info('build client');
	await exec('npx vite build --outDir dist-client --ssrManifest');

	console.info('build server');
	await exec('npx vite build --outDir dist-server --ssr src/server.ts');

	console.info('create dist');
	try {
		await fs.rm('dist', { force: true, recursive: true });
	} catch (e) {}
	await fs.mkdir('dist');

	await fs.cp('dist-server/', 'dist/', { recursive: true });

	await fs.cp('dist-client/', 'dist/public/', { recursive: true });
	await fs.rename('dist/public/index.html', 'dist/index.html');
	await fs.rename(
		'dist/public/.vite/ssr-manifest.json',
		'dist/ssr-manifest.json',
	);

	try {
		await fs.rm('dist/public/.vite', { force: true, recursive: true });
	} catch (e) {}
	try {
		await fs.rm('dist-client', { force: true, recursive: true });
	} catch (e) {}
	try {
		await fs.rm('dist-server', { force: true, recursive: true });
	} catch (e) {}
}
main();
