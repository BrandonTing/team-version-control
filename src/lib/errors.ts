export class RedirectHomeError {
	readonly _tag = "RedirectHomeError"
	
	constructor(public path: string) {}
}

export class InvokeTauriError {
	readonly _tag = "InvokeTauriError"
    constructor(public command: string) {}
}
