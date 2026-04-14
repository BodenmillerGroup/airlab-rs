export interface SpeciesView {
    readonly id: number;
    readonly groupId: number;
    readonly name: string;
    readonly acronym: string;
    readonly meta: object;
    readonly createdAt: object;
}
export interface SpeciesDto {
    readonly id: number;
    readonly groupId: number;
    readonly name: string;
    readonly acronym: string;
    readonly meta: object;
    readonly createdAt: object;
}
export interface CreateSpeciesDto {
    readonly groupId: number;
    readonly name: string;
    readonly acronym: string;
}
export interface UpdateSpeciesDto {
    readonly name: string;
    readonly acronym: string;
}
