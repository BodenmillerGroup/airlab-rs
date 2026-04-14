export interface MemberView {
    readonly id: number;
    readonly groupId: number;
    readonly userId: number;
    readonly userName: string;
    readonly role: number;
    readonly activationKey: string;
    readonly isActive: boolean;
    readonly allPanels: boolean;
    readonly createdAt: string;
    readonly updatedAt: string;
}
export interface MemberDto {
    readonly id: number;
    readonly groupId: number;
    readonly userId: number;
    readonly role: number;
    readonly activationKey: string;
    readonly isActive: boolean;
    readonly allPanels: boolean;
    readonly createdAt: string;
    readonly updatedAt: string;
}
export interface CreateMemberDto {
    readonly groupId: number;
    readonly userId: number;
    readonly role: number;
    readonly isActive: boolean;
    readonly allPanels: boolean;
}
export interface UpdateMemberDto {
    readonly role: number;
    readonly isActive: boolean;
    readonly allPanels: boolean;
}
