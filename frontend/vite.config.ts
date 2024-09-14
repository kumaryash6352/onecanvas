import { sveltekit } from '@sveltejs/kit/vite';
import basicSsl from '@vitejs/plugin-basic-ssl'
import { defineConfig } from 'vite';
import * as mkcert from 'mkcert'

export default defineConfig({
	plugins: [sveltekit(), basicSsl({
		name: 'onecanvas',
		domains: ['*'],
		certDir: './dev/cert'
	})],
	server: {
		https: true,
		proxy: {} 
	}
});
