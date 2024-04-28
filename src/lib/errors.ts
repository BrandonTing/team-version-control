export class RedirectError {
	readonly _tag = "RedirectError"
	
	constructor(public path: string) {}
}

export class InvokeTauriError {
	readonly _tag = "InvokeTauriError"
    constructor(public command: string, public message: string) {}
}
