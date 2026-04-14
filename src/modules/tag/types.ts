import { TagStatus } from "./TagStatus";
export interface TagView {
    readonly id: number;
    readonly groupId: number;
    readonly name: string;
    readonly isMetal: boolean;
    readonly isFluorophore: boolean;
    readonly isEnzyme: boolean;
    readonly isBiotin: boolean;
    readonly isOther: boolean;
    readonly status: number;
    readonly description: string | null;
    readonly mw: number | null;
    readonly emission: number | null;
    readonly excitation: number | null;
    readonly meta: object;
    readonly createdAt: string;
}
export interface TagDto {
    readonly id: number;
    readonly groupId: number;
    readonly name: string;
    readonly isMetal: boolean;
    readonly isFluorophore: boolean;
    readonly isEnzyme: boolean;
    readonly isBiotin: boolean;
    readonly isOther: boolean;
    readonly status: number;
    readonly description: string | null;
    readonly mw: number | null;
    readonly emission: number | null;
    readonly excitation: number | null;
    readonly meta: object;
    readonly createdAt: string;
}
export interface CreateTagDto {
    readonly groupId: number;
    readonly name: string;
    readonly isMetal: boolean;
    readonly isFluorophore: boolean;
    readonly isEnzyme: boolean;
    readonly isBiotin: boolean;
    readonly isOther: boolean;
    readonly status: TagStatus;
    readonly description: string | null;
    readonly mw: number | null;
    readonly emission: number | null;
    readonly excitation: number | null;
}
export interface UpdateTagDto {
    readonly name: string;
    readonly isMetal: boolean;
    readonly isFluorophore: boolean;
    readonly isEnzyme: boolean;
    readonly isBiotin: boolean;
    readonly isOther: boolean;
    readonly status: TagStatus;
    readonly description: string | null;
    readonly mw: number | null;
    readonly emission: number | null;
    readonly excitation: number | null;
}
