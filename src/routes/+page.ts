import { getTeams } from '@/bindings';
import { error } from '@sveltejs/kit';

// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;

export async function load() {
	try {
		return { teamsRequest: getTeams() }
	} catch (e) {
		error(500, {
			message: "Failed to get teams"
		})
	}
}
