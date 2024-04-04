import { z } from "zod";
export const createTeamFormSchema = z.object({
    title: z.string().min(1).max(20),
    description: z.string().min(1),
    mainBranchTitle: z.string().min(1).default("main"),
});
   
export type FormSchema = typeof createTeamFormSchema;