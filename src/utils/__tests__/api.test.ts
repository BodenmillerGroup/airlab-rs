import { describe, it, expect, vi, beforeEach } from "vitest";
import ky from "ky";
import { ApiManager } from "@/utils/api";

vi.mock("ky", () => {
  return {
    default: {
      extend: vi.fn(),
    },
  };
});

describe("ApiManager", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("initializes ky with prefixUrl and hooks", () => {
    ApiManager.init("TEST_TOKEN");

    expect(ky.extend).toHaveBeenCalledOnce();

    const config = (ky.extend as any).mock.calls[0][0];

    expect(config.prefixUrl).toBeDefined();
    expect(config.throwHttpErrors).toBe(true);
    expect(config.hooks.beforeRequest).toHaveLength(1);
    expect(config.hooks.afterResponse).toHaveLength(1);
  });

  it("injects Authorization header before request", async () => {
    ApiManager.init("MY_SECRET");

    const config = (ky.extend as any).mock.calls[0][0];
    const beforeRequest = config.hooks.beforeRequest[0];

    const headers = new Headers();
    const req = new Request("http://test", { headers });

    await beforeRequest(req);

    expect(req.headers.get("Authorization")).toBe("Bearer MY_SECRET");
  });

  it("rewrites error responses in afterResponse", async () => {
    ApiManager.init("X");

    const config = (ky.extend as any).mock.calls[0][0];
    const afterResponse = config.hooks.afterResponse[0];

    const fakeResponse = {
      ok: false,
      json: vi.fn().mockResolvedValue({
        statusCode: 401,
        message: "Unauthorized",
      }),
    };

    const newResponse = await afterResponse(
      {} as any,
      {} as any,
      fakeResponse as any
    );

    expect(newResponse).toBeInstanceOf(Response);
    expect(newResponse.status).toBe(401);
    expect(newResponse.statusText).toBe("Unauthorized");
  });

  it("exposes the configured api instance", () => {
    (ky.extend as any).mockReturnValue("KY_INSTANCE");

    ApiManager.init("TOKEN");

    expect(ApiManager.api).toBe("KY_INSTANCE");
  });
});
