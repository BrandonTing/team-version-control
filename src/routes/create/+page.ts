import { superValidate } from 'sveltekit-superforms';
import { zod } from "sveltekit-superforms/adapters";
import { createTeamFormSchema } from './schema';

// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;
 
export async function load() {
	return {
		form: await superValidate(zod(createTeamFormSchema)),
	}
}
