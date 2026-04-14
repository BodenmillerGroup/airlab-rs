export interface PanelElementDataDto {
    readonly conjugateId: number;
    readonly dilutionType: number;
    readonly concentration?: number;
}
export interface PanelView {
    readonly id: number;
    readonly groupId: number;
    readonly createdBy: number;
    readonly name: string;
    readonly description: string;
    readonly isFluorophore: boolean;
    readonly isLocked: boolean;
    readonly application: number;
    readonly meta: object;
    readonly isArchived: boolean;
    readonly createdAt: string;
    readonly updatedAt: string;
    readonly userName: string;
    readonly userId: number;
    //readonly elements: PanelElementDataDto[];
}
export interface PanelDto {
    readonly id: number;
    readonly groupId: number;
    readonly createdBy: number;
    readonly name: string;
    readonly description: string;
    readonly isFluorophore: boolean;
    readonly isLocked: boolean;
    readonly application: number;
    readonly meta: object;
    readonly isArchived: boolean;
    readonly createdAt: string;
    readonly updatedAt: string;
    //readonly elements: PanelElementDataDto[];
}
export interface CreatePanelDto {
    readonly createdBy?: number;
    readonly groupId: number;
    readonly name: string;
    readonly description: string;
    readonly isFluorophore: boolean;
    readonly isLocked: boolean;
    readonly application: number | null;
    readonly elements?: PanelElementDataDto[];
}
export interface UpdatePanelDto {
    readonly name?: string;
    readonly description?: string;
    readonly isFluorophore?: boolean;
    readonly isLocked?: boolean;
    readonly application?: number | null;
    readonly elements?: PanelElementDataDto[];
}
export interface DuplicatePanelDto {
    readonly createdBy?: number;
    readonly groupId: number;
    readonly name: string;
}
