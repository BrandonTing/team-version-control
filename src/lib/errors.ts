import { Data } from "effect"

export class RedirectError extends Data.TaggedError("RedirectError")<{path: string}> {
}

export class InvokeTauriError {
	readonly _tag = "InvokeTauriError"
    constructor(public command: string, public message: string) {}
}
