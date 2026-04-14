export interface GroupView {
    id: number;
    name: string;
    institution: string;
    url: string;
    isOpen: boolean;
    meta: object;
    createdAt: string;
    members: {
        id: number;
        role: number;
        userId: number;
    }[];
}
export interface GroupDto {
    readonly id: number;
    readonly name: string;
    readonly institution: string;
    readonly url: string;
    readonly isOpen: boolean;
    readonly meta: object;
    readonly createdAt: string;
}
export interface CreateGroupDto {
    readonly name: string;
    readonly institution: string;
    readonly url: string | null;
    readonly isOpen: boolean;
}
export interface UpdateGroupDto {
    readonly name: string;
    readonly institution: string;
    readonly url: string | null;
    readonly isOpen: boolean;
}
export interface InviteDto {
    readonly groupId: number;
    readonly userId: number;
    readonly token: string;
}
