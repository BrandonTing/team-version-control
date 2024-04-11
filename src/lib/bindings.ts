/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function getTeams() {
    return invoke()<Team[]>("get_teams")
}

export function getTeam(key: string) {
    return invoke()<TeamDetail>("get_team", { key })
}

export function createTeam(title: string, description: string, mainBranchTitle: string) {
    return invoke()<TeamDetail>("create_team", { title,description,mainBranchTitle })
}

export function createBranch(teamTitle: string, title: string, description: string) {
    return invoke()<null>("create_branch", { teamTitle,title,description })
}

export function createChange(teamTitle: string, branchTitle: string, message: string, context: string) {
    return invoke()<string>("create_change", { teamTitle,branchTitle,message,context })
}

export function resetStore() {
    return invoke()<null>("reset_store")
}

export type TeamDetail = { team: Team; branches: Branch[]; current_branch_title: string }
export type Team = { title: string; description: string; created_at: number }
export type Branch = { title: string; description: string; history: Change[]; current_change_id: string }
export type Change = { id: string; message: string; context: string }
