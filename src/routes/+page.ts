import { Store } from 'tauri-plugin-store-api';
import { getTeam } from '../bindings';
// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;

export async function load() {
	const teams = await getTeam();
	const store = new Store('team');
	const keys = await store.keys();
	
	return {
		keys,
		teams: teams
	}
}