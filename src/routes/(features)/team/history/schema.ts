import { z } from "zod";
export const createBranchFromChangeFormSchema = z.object({
    title: z.string().min(1).max(20),
    description: z.string().min(1),
});

export type FormSchema = typeof createBranchFromChangeFormSchema;