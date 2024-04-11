import { goto } from '$app/navigation';
import { getTeam } from '@/bindings';
import { error } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { createBranchFormSchema, createChangeFormSchema } from './schema.js';

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
		const branchTitle = url.searchParams.get('branch')
		const team = await getTeam(title);
		const branch = team.branches.find((val) => val.title === branchTitle);
		const change = branch?.history.find((val) => val.id === branch.current_change_id);
		return {
			team,
			title,
			branchTitle,
			branch,
			change,
			createBranchForm: await superValidate(zod(createBranchFormSchema)),
			createChangeForm: await superValidate(zod(createChangeFormSchema, {
				defaults: {context: change?.context ?? "", message: ""}
			})),
		}
	} catch (e) {
		error(500, {
			message: "Failed to get team info"
		})
	}
}
