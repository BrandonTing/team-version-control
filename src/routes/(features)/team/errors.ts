
export class InvalidBranchTitleError {
    readonly _tag = "InvalidBranchTitleError"
}

export class ParsePasteError {
    readonly _tag = "InvalidBranchTitleError"
    constructor(public message: string) {

    }
}
