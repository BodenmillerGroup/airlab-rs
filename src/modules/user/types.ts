export interface UserView {
    readonly id: number;
    readonly email: string;
    readonly name: string;
    readonly isActive: boolean;
    readonly isAdmin: boolean;
    readonly mfaEnabled: boolean;
    readonly createdAt: string;
    readonly updatedAt: string;
}
export interface UserDto {
    readonly id: number;
    readonly email: string;
    readonly name: string;
    readonly isActive: boolean;
    readonly isAdmin: boolean;
    readonly mfaEnabled: boolean;
    readonly createdAt: string;
    readonly updatedAt: string;
}
export interface ProfileDto {
    readonly id: number;
    readonly name: string;
    readonly email: string;
    readonly isAdmin: boolean;
    readonly mfaEnabled: boolean;
}
export interface UpdateProfileDto {
    readonly email: string;
    readonly name: string;
}
export interface UpdatePasswordDto {
    readonly password: string;
}
export interface CreateUserDto {
    readonly email: string;
    readonly name: string;
    readonly password: string;
}
export interface UpdateUserDto {
    readonly email: string;
    readonly name: string;
    readonly isActive: boolean;
    readonly isAdmin: boolean;
    readonly password: string;
}
