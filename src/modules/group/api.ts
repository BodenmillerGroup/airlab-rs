import { ApiManager } from "@/utils/api";
import type { CreateGroupDto, GroupDto, UpdateGroupDto } from "@/modules/group/types";
import type { MemberDto } from "@/modules/member/types";

export const api = {
  async getGroups() {
    return ApiManager.api.get(`groups`).json<GroupDto[]>();
  },
  async createGroup(data: CreateGroupDto) {
    return ApiManager.api
      .post(`groups/`, {
        json: data,
      })
      .json<GroupDto>();
  },
  async getGroup(id: number) {
    return ApiManager.api.get(`groups/${id}`).json<GroupDto>();
  },
  async updateGroup(id: number, data: UpdateGroupDto) {
    return ApiManager.api
      .patch(`groups/${id}`, {
        json: data,
      })
      .json<GroupDto>();
  },
  async deleteGroup(id: number) {
    return ApiManager.api.delete(`groups/${id}`).json<number>();
  },
  async joinGroup(id: number) {
    return ApiManager.api.post(`groups/${id}/join`).json<boolean>();
  },
  async getMyMember(groupId: number) {
    return ApiManager.api.get(`groups/${groupId}/members/me`).json<MemberDto>();
  },
  async exportGroupData(id: number, format: "json" | "csv") {
    return ApiManager.api.get(`groups/${id}/export?format=${format}`).blob();
  },
  async exportAllData(format: "json" | "csv") {
    return ApiManager.api.get(`export?format=${format}`).blob();
  },
  async importGroupData(formData: FormData) {
    return ApiManager.api
      .post(`groups/import`, {
        body: formData,
        timeout: false,
      })
      .json<GroupDto>();
  },
  async importAllData(formData: FormData) {
    return ApiManager.api
      .post(`import`, {
        body: formData,
        timeout: false,
      })
      .json<GroupDto[]>();
  },
};
