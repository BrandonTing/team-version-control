import { Store } from 'tauri-plugin-store-api';
// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;

export async function load() {

	const store = new Store('.settings.dat');

	await store.set('some-key', { value: 5 });

	const val = await store.get('some-key');
    console.log(val)
	await store.save();

}