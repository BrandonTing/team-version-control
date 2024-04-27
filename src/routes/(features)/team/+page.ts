import { goto } from '$app/navigation';
import { getTeam, type Branch, type Change } from '@/bindings.js';
import { error, redirect } from '@sveltejs/kit';
import { Effect, Either } from "effect";
import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { createBranchFormSchema, createChangeFormSchema } from './schema.js';
// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;
  
class RedirectHomeError {
	readonly _tag = "RedirectHomeError"
}

export async function load({url}) {
	const eitherTitle = await Effect.gen(function* (){
		const raw = url.searchParams.get("title");

		if(!raw) {
			return yield* Effect.fail(new RedirectHomeError())
		}

		const title = yield* Effect.try({
			try: () => {
				return decodeURIComponent(raw)	
			},
			catch: () => {
				return new RedirectHomeError()
			}
		})

		return title
	}).pipe(Effect.either, Effect.runPromise)

	if(Either.isLeft(eitherTitle)) {
		throw redirect(302, "/")
	}

	const title = eitherTitle.right
	
	try {
		// Here
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
