import ky from "ky";
import { apiUrl } from "@/env";

interface UnauthorizedContext {
  status: number;
  request: Request;
  response: Response;
}

export class ApiManager {
  private static _token: string;
  private static _api: typeof ky;
  private static _onUnauthorized?: (context: UnauthorizedContext) => void | Promise<void>;

  static setUnauthorizedHandler(handler?: (context: UnauthorizedContext) => void | Promise<void>) {
    ApiManager._onUnauthorized = handler;
  }

  static init(token: string) {
    ApiManager._token = token;
    ApiManager._api = ky.extend({
      prefixUrl: apiUrl,
      throwHttpErrors: true,
      hooks: {
        beforeRequest: [
          async (request) => {
            request.headers.set("Authorization", `Bearer ${ApiManager._token}`);
          },
        ],
        afterResponse: [
          async (request, options, response) => {
            if (!response.ok) {
              let status = response.status;
              let statusText = response.statusText;

              try {
                const errorJson = (await response.json()) as { statusCode: number; message?: string; error?: string };
                status = errorJson.statusCode ?? response.status;
                statusText = errorJson.message ? errorJson.message : (errorJson.error ?? response.statusText);
              } catch (e) {
                console.error(e);
              }

              if ((status === 401 || status === 403) && ApiManager._onUnauthorized) {
                await ApiManager._onUnauthorized({
                  status,
                  request,
                  response,
                });
              }

              return new Response(null, {
                status,
                statusText,
              });
            }
          },
        ],
      },
    });
  }

  static get api() {
    return ApiManager._api;
  }
}
