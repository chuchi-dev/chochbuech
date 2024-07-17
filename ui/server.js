import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import express from 'express';
import { createServer as createViteServer } from 'vite';
import fetch from 'node-fetch';

globalThis.fetch = fetch;

async function createServer() {
	const app = express();

	const vite = await createViteServer({
		server: { middlewareMode: true },
		appType: 'custom',
	});

	app.use(vite.middlewares);

	app.use('*', async (req, res, next) => {
		const uri = req.originalUrl;

		try {
			// 1. Read index.html
			let template = fs.readFileSync('./index.html', 'utf-8');

			template = await vite.transformIndexHtml(uri, template);

			const { render } = await vite.ssrLoadModule('./src/server.ts');

			const { status, fields } = await render({
				method: 'GET',
				uri,
				headers: req.headers,
			});

			let html = template;
			for (const field in fields) {
				html = html.replace(`<!--ssr-${field}-->`, fields[field]);
			}

			// 6. Send the rendered HTML back.
			res.status(status).set({ 'Content-Type': 'text/html' }).end(html);
		} catch (e) {
			// If an error is caught, let Vite fix the stack trace so it maps back to
			// your actual source code.
			vite.ssrFixStacktrace(e);
			next(e);
		}
	});

	app.listen(8080);
	console.log('listening on 8080');
}

createServer();
