import { ConjugateStatus } from "./ConjugateStatus";
import type { ValidationDto } from "@/modules/validation/types";
export interface ConjugateView {
    readonly id: number;
    readonly groupId: number;
    readonly createdBy: number;
    readonly labeledBy: number;
    readonly finishedBy: number;
    readonly lotId: number;
    readonly tagId: number;
    readonly storageId: number | null;
    readonly status: number;
    readonly tubeNumber: number;
    readonly concentration: number;
    readonly description: string | null;
    readonly isArchived: boolean;
    readonly meta: object;
    readonly labeledAt: string;
    readonly createdAt: string;
    readonly updatedAt: string;
    readonly customId: string;
    readonly tagName: string;
    readonly tagMw: number | null;
    readonly proteinName: string;
    readonly proteinId: number;
    readonly cloneName: string;
    readonly cloneId: number;
    readonly lotName: string;
    readonly lotCollectionId: number | null;
    readonly lotCollectionName: string;
    readonly userName: string;
    readonly userId: number;
    readonly validations: ValidationDto[];
}
export interface ConjugateDto {
    readonly id: number;
    readonly groupId: number;
    readonly createdBy: number;
    readonly labeledBy: number;
    readonly finishedBy: number;
    readonly lotId: number;
    readonly tagId: number;
    readonly storageId: number | null;
    readonly status: number;
    readonly tubeNumber: number;
    readonly concentration: number;
    readonly description: string | null;
    readonly isArchived: boolean;
    readonly meta: object;
    readonly labeledAt: string;
    readonly createdAt: string;
    readonly updatedAt: string;
    readonly customId: string;
}
export interface CreateConjugateDto {
    readonly createdBy?: number;
    readonly groupId: number;
    readonly lotId: number;
    readonly tagId: number;
    readonly storageId: number | null;
    readonly labeledBy: number | null;
    readonly concentration: number | null;
    readonly description: string | null;
    readonly customId: string | null;
}
export interface UpdateConjugateDto {
    readonly lotId: number;
    readonly tagId: number;
    readonly storageId: number | null;
    readonly labeledBy: number | null;
    readonly concentration: number | null;
    readonly description: string | null;
    readonly customId: string | null;
}
export interface UpdateConjugateStatusDto {
    readonly status: ConjugateStatus;
}
