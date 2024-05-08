import { getChangeHistory } from "@/bindings.js";
import { InvokeTauriError, RedirectError } from "@/errors.js";
import { error, redirect } from "@sveltejs/kit";
import { Effect, Either } from "effect";
import { superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { createBranchFromChangeFormSchema } from "./schema";

export async function load({ url }) {
    const program = Effect.gen(function* () {
        const teamTitle = url.searchParams.get('team');
        const branchTitle = url.searchParams.get('branch');
        if (!teamTitle || !branchTitle) {
            return yield* new RedirectError({ path: "/" })
        }
        const history = yield* Effect.tryPromise({
            try: () => getChangeHistory(teamTitle, branchTitle),
            catch: (e) => new InvokeTauriError("getChangeHistory", e as string)
        })

        return {
            history,
            teamTitle,
            branchTitle
        }
    })
    const eitherData = await program.pipe(Effect.either, Effect.runPromise);
    if (Either.isLeft(eitherData)) {
        if (eitherData.left instanceof RedirectError) {
            throw redirect(302, eitherData.left.path)
        }
        throw error(500, "failed to get change history")
    }
    return {
        data: eitherData.right,
        form: await superValidate(zod(createBranchFromChangeFormSchema)),
    }
}