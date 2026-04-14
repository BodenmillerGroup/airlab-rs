import type { CreateMemberDto, MemberDto, UpdateMemberDto } from "@/modules/member/types";
import { ApiManager } from "@/utils/api";

export const api = {
  async createMember(data: CreateMemberDto) {
    return ApiManager.api
      .post(`members`, {
        json: data,
      })
      .json<MemberDto>();
  },
  async getMember(id: number) {
    return ApiManager.api.get(`members/${id}`).json<MemberDto>();
  },
  async updateMember(id: number, data: UpdateMemberDto) {
    return ApiManager.api
      .patch(`members/${id}`, {
        json: data,
      })
      .json<MemberDto>();
  },
  async deleteMember(id: number) {
    return ApiManager.api.delete(`members/${id}`).json<number>();
  },
  async getGroupMembers(groupId: number) {
    return ApiManager.api.get(`groups/${groupId}/members`).json<MemberDto[]>();
  },
};
