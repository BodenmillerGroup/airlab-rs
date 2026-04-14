export interface ProviderView {
    readonly id: number;
    readonly groupId: number;
    readonly name: string;
    readonly description: string;
    readonly url: string;
    readonly meta: object;
    readonly createdAt: string;
}
export interface ProviderDto {
    readonly id: number;
    readonly groupId: number;
    readonly name: string;
    readonly description: string;
    readonly url: string;
    readonly meta: object;
    readonly createdAt: string;
}
export interface CreateProviderDto {
    readonly groupId: number;
    readonly name: string;
    readonly description: string | null;
    readonly url: string | null;
}
export interface UpdateProviderDto {
    readonly name: string;
    readonly description: string | null;
    readonly url: string | null;
}
