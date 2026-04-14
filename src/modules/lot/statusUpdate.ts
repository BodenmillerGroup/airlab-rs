import { LotStatus } from "./LotStatus";
import type { UpdateLotStatusDto } from "./types";

export function buildLotStatusUpdatePayload(
  status: LotStatus,
  memberId: number,
  at: string = new Date().toISOString(),
): Partial<UpdateLotStatusDto> {
  if (status === LotStatus.Requested) {
    return {
      requestedBy: memberId,
      requestedAt: at,
    };
  }

  if (status === LotStatus.Approved) {
    return {
      approvedBy: memberId,
      approvedAt: at,
    };
  }

  if (status === LotStatus.Ordered) {
    return {
      orderedBy: memberId,
      orderedAt: at,
    };
  }

  if (status === LotStatus.Stock) {
    return {
      receivedBy: memberId,
      receivedAt: at,
    };
  }

  if (status === LotStatus.Finished) {
    return {
      finishedBy: memberId,
      finishedAt: at,
    };
  }

  return {};
}
