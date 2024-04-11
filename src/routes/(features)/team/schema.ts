import { z } from "zod";
export const createBranchFormSchema = z.object({
    title: z.string().min(1).max(20),
    description: z.string().min(1),
});
   
export type FormSchema = typeof createBranchFormSchema;

export const createChangeFormSchema = z.object({
    message: z.string().min(1).max(50),
    context: z.string().min(1),
});
   
export type CreateChangeFormSchema = typeof createChangeFormSchema;