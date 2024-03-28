import { error } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { zod } from "sveltekit-superforms/adapters";
import { getTeams } from '../bindings';
import { formSchema } from './schema';

// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;
 
export async function load() {
	try {
		const teams = await getTeams();
		return {
			teams,
			form: await superValidate(zod(formSchema)),
		}
	} catch (e) {
		console.log(e)
		error(500, {
			message: "Failed to get teams"
		})
	}
	
}
