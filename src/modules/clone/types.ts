export interface CloneView {
    id: number;
    name: string;
    groupId: number;
    proteinName?: string;
    proteinId: number;
    speciesName?: string;
    validations: {
        id: number;
        application: string;
        status: string;
    }[];
    isPolyclonal: boolean;
    isPhospho: boolean;
    createdBy: number;
    speciesId: number;
    isotype: string;
    epitope: string;
    reactivity: number[];
    application: object;
    isArchived: boolean;
    createdAt: string;
    updatedAt: string;
}
export interface CloneDto {
    readonly id: number;
    readonly groupId: number;
    readonly createdBy: number;
    readonly proteinId: number;
    readonly speciesId: number;
    readonly name: string;
    readonly isotype: string;
    readonly epitope: string;
    readonly isPhospho: boolean;
    readonly isPolyclonal: boolean;
    readonly reactivity: number[];
    readonly application: object;
    readonly isArchived: boolean;
    readonly meta: object;
    readonly createdAt: string;
    readonly updatedAt: string;
}
export interface CreateCloneDto {
    readonly createdBy?: number;
    readonly groupId: number;
    readonly proteinId: number;
    readonly speciesId: number;
    readonly name: string;
    readonly isotype: string;
    readonly epitope: string;
    readonly isPhospho: boolean;
    readonly isPolyclonal: boolean;
    readonly reactivity: number[];
    readonly application: object;
}
export interface UpdateCloneDto {
    readonly proteinId: number;
    readonly speciesId: number;
    readonly name: string;
    readonly isotype: string;
    readonly epitope: string;
    readonly isPhospho: boolean;
    readonly isPolyclonal: boolean;
    readonly reactivity: number[];
    readonly application: object;
}
