import unocss from 'unocss/vite';
import { defineConfig } from 'vite';
import presetUno from '@unocss/preset-uno';
import react from '@vitejs/plugin-react';

const config = defineConfig({
	plugins: [
		react(),
		unocss({
			presets: [presetUno()],
		}),
	],
});

export default config;
