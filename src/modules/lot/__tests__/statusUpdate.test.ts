import { describe, expect, it } from "vitest";

import { LotStatus } from "../LotStatus";
import { buildLotStatusUpdatePayload } from "../statusUpdate";

describe("buildLotStatusUpdatePayload", () => {
  const memberId = 12;
  const at = "2026-03-19T12:00:00.000Z";

  it("adds actor and timestamp for tracked lifecycle statuses", () => {
    expect(buildLotStatusUpdatePayload(LotStatus.Requested, memberId, at)).toEqual({
      requestedBy: memberId,
      requestedAt: at,
    });

    expect(buildLotStatusUpdatePayload(LotStatus.Approved, memberId, at)).toEqual({
      approvedBy: memberId,
      approvedAt: at,
    });

    expect(buildLotStatusUpdatePayload(LotStatus.Ordered, memberId, at)).toEqual({
      orderedBy: memberId,
      orderedAt: at,
    });

    expect(buildLotStatusUpdatePayload(LotStatus.Stock, memberId, at)).toEqual({
      receivedBy: memberId,
      receivedAt: at,
    });

    expect(buildLotStatusUpdatePayload(LotStatus.Finished, memberId, at)).toEqual({
      finishedBy: memberId,
      finishedAt: at,
    });
  });

  it("returns no actor metadata for statuses without tracked timestamp fields", () => {
    expect(buildLotStatusUpdatePayload(LotStatus.Rejected, memberId, at)).toEqual({});
    expect(buildLotStatusUpdatePayload(LotStatus.Low, memberId, at)).toEqual({});
  });
});
