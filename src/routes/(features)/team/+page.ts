import { goto } from '$app/navigation';
import { getTeam } from '@/bindings';
import { error } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { createBranchFormSchema } from './schema.js';

// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;

export async function load({url}) {
    const rawTeamTitle = url.searchParams.get("title")
    if(!rawTeamTitle) {
        goto("/")
    }
	try {
		const title = decodeURIComponent(rawTeamTitle ?? '')
		console.log("load")
		return {
			team:await getTeam(title),
			title,
			form: await superValidate(zod(createBranchFormSchema)),
		}
	} catch (e) {
		error(500, {
			message: "Failed to get team info"
		})
	}
}
