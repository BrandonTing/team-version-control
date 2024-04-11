import { goto } from '$app/navigation';
import { getTeam, type Branch, type Change } from '@/bindings';
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
		const changeId = url.searchParams.get('change')
		const team = await getTeam(title);
		let branch: Branch | undefined;
		let change: Change | undefined;
		if (!branchTitle || !changeId) {
			const query = new URLSearchParams(url.searchParams.toString());
			branch = team.branches.find((val) => val.title === team.current_branch_title);
			change = branch?.history.find((val) => val.id === branch?.current_change_id);
			query.set('branch', team.current_branch_title);
			query.set('change', branch?.current_change_id ?? '');
			goto(`/team?${query.toString()}`);
		} else {
			branch = team.branches.find((val) => val.title === branchTitle);
			change = branch?.history.find((val) => val.id === changeId);	
		}
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