import ky from "ky";
import { apiUrl } from "@/env";
import { ApiManager } from "@/utils/api";
import type {
  CreateUserDto,
  ProfileDto,
  UpdatePasswordDto,
  UpdateProfileDto,
  UpdateUserDto,
  UserDto,
} from "@/modules/user/types";

export const api = {
  async logInGetToken(username: string, password: string) {
    const params = new URLSearchParams();
    params.append("username", username);
    params.append("password", password);

    return ky.post(`${apiUrl}/auth/login`, { body: params }).json();
  },
  async getMe() {
    return ApiManager.api.get(`users/profile`).json<ProfileDto>();
  },
  async updateMe(data: UpdateProfileDto) {
    return ApiManager.api
      .patch(`users/profile`, {
        json: data,
      })
      .json<ProfileDto>();
  },
  async updatePassword(data: UpdatePasswordDto) {
    return ApiManager.api
      .patch(`users/profile/password`, {
        json: data,
      })
      .json<ProfileDto>();
  },
  async getUsers() {
    return ApiManager.api.get(`users`).json<UserDto[]>();
  },
  async getUser(id: number) {
    return ApiManager.api.get(`users/${id}`).json<UserDto>();
  },
  async updateUser(id: number, data: UpdateUserDto) {
    return ApiManager.api
      .patch(`users/${id}`, {
        json: data,
      })
      .json<UserDto>();
  },
  async createUser(data: CreateUserDto) {
    return ApiManager.api
      .post(`users`, {
        json: data,
      })
      .json<UserDto>();
  },
  async verifyOtp(token: string, code: string) {
    const response = await fetch("/api/v1/users/verifymfa", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        code: code,
        secret: "",
      }),
    });
    const result = await response.json();
    return result.success;
  },



  async passwordRecovery(email: string) {
    return ky.post(`${apiUrl}/auth/password-recovery/${email}`);
  },
  async checkUserExists(email: string) {
    return ky.get(`${apiUrl}/auth/check/${email}`).json<{ exists: boolean }>();
  },
  async signUp(data: CreateUserDto) {
    return ky
      .post(`${apiUrl}/auth/signup`, {
        json: data,
      })
      .json<UserDto>();
  },
  async resetPassword(password: string, token: string) {
    return ky
      .post(`${apiUrl}/auth/reset-password/`, {
        json: {
          newPassword: password,
          token,
        },
      })
      .json();
  },
};
