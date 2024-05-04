import { getTeam } from '@/bindings.js';
import { InvokeTauriError, RedirectError } from '@/errors.js';
import { error, redirect } from '@sveltejs/kit';
import { Effect, Either } from "effect";
import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { createBranchFormSchema, createChangeFormSchema } from './schema.js';
// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;
  
export async function load({url}) {
	const checkTitle = Effect.try({
		try: () => {
			const raw = url.searchParams.get("title");
			if(!raw) {
				throw new Error("title is required")
			}
			return decodeURIComponent(raw)
		},
		catch: () => {
			return new RedirectError("/")
		}
	})
	const eitherTitle = checkTitle.pipe(Effect.either, Effect.runSync)
	if(Either.isLeft(eitherTitle)) {
		throw redirect(302, eitherTitle.left.path)
	}
	const getBranchInfo = Effect.tryPromise({
		try: async () => {
			const title = eitherTitle.right
			const team = await getTeam(title);
			const branchTitle = url.searchParams.get('branch')
			const changeId = url.searchParams.get('change')
			const query = new URLSearchParams(url.searchParams.toString());
			if (!branchTitle) {
				const branch = team.branches.find((val) => val.title === team.current_branch_title);
				const currentChangeId = branch?.current_change_id
				if(!changeId && currentChangeId) {
					query.set('change', currentChangeId);
				}
				query.set('branch', team.current_branch_title);
				throw new RedirectError(`/team?${query.toString()}`)
			}
			
			const branch = team.branches.find((val) => val.title === branchTitle);
			const currentChangeId = branch?.current_change_id
			if(!changeId && currentChangeId) {
				query.set('change', currentChangeId);
				throw new RedirectError(`/team?${query.toString()}`)
			}
			const change = branch?.history.find((val) => val.id === changeId);	
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
	
		},
		catch: (e) => {
			if(e instanceof RedirectError) {
				return e
			}
			return new InvokeTauriError("getTeam", e as string)
		}
	})
	const eitherData = await getBranchInfo.pipe(
		Effect.either,
		Effect.runPromise
	)
	if(Either.isLeft(eitherData)) {
		if(eitherData.left instanceof RedirectError) {
			throw redirect(302, eitherData.left.path)
		}
		throw error(500, "failed to get team data")
	}
	return eitherData.right
}
