import { LotStatus } from "./LotStatus";
export interface LotView {
    readonly id: number;
    readonly groupId: number;
    readonly createdBy: number;
    readonly cloneId: number;
    readonly providerId: number;
    readonly storageId: number | null;
    readonly collectionId: number | null;
    readonly name: string;
    readonly reference: string;
    readonly requestedBy: number;
    readonly approvedBy: number;
    readonly orderedBy: number;
    readonly receivedBy: number;
    readonly finishedBy: number;
    readonly number: string;
    readonly status: number;
    readonly purpose: string;
    readonly url: string;
    readonly price: string;
    readonly note: string;
    readonly requestedAt: string;
    readonly approvedAt: string;
    readonly orderedAt: string;
    readonly receivedAt: string;
    readonly finishedAt: string;
    readonly isArchived: boolean;
    readonly meta: object;
    readonly createdAt: string;
    readonly updatedAt: string;
}
export interface LotDto {
    readonly id: number;
    readonly groupId: number;
    readonly createdBy: number;
    readonly cloneId: number;
    readonly providerId: number;
    readonly storageId: number | null;
    readonly collectionId: number | null;
    readonly name: string;
    readonly reference: string;
    readonly requestedBy: number;
    readonly approvedBy: number;
    readonly orderedBy: number;
    readonly receivedBy: number;
    readonly finishedBy: number;
    readonly number: string;
    readonly status: number;
    readonly purpose: string;
    readonly url: string;
    readonly price: string;
    readonly note: string;
    readonly requestedAt: string;
    readonly approvedAt: string;
    readonly orderedAt: string;
    readonly receivedAt: string;
    readonly finishedAt: string;
    readonly isArchived: boolean;
    readonly meta: object;
    readonly createdAt: string;
    readonly updatedAt: string;
}
export interface CreateLotDto {
    readonly createdBy?: number;
    readonly groupId: number;
    readonly cloneId: number;
    readonly providerId: number;
    readonly storageId?: number | null;
    readonly collectionId?: number | null;
    readonly name: string;
    readonly reference: string;
    readonly number: string;
    readonly url: string | null;
    readonly purpose: string | null;
    readonly price?: string | null;
    readonly note?: string | null;
}
export interface UpdateLotDto {
    readonly cloneId: number;
    readonly providerId: number;
    readonly storageId?: number | null;
    readonly collectionId?: number | null;
    readonly name: string;
    readonly reference: string;
    readonly number: string;
    readonly url: string | null;
    readonly purpose: string | null;
    readonly price?: string | null;
    readonly note?: string | null;
}
export interface UpdateLotStatusDto {
    readonly status: LotStatus;
    readonly lotNumber?: string;
    readonly requestedBy?: number;
    readonly requestedAt?: string;
    readonly approvedBy?: number;
    readonly approvedAt?: string;
    readonly orderedBy?: number;
    readonly orderedAt?: string;
    readonly receivedBy?: number;
    readonly receivedAt?: string;
    readonly finishedBy?: number;
    readonly finishedAt?: string;
}
export interface LotQuery {
    status?: LotStatus;
    limit?: number,
}
export interface ReorderLotDto {
    readonly purpose?: string;
    readonly requestedBy?: number;
    readonly requestedAt?: string;
}
