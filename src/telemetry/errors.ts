export async function reportFrontendError(
  error: unknown,
  context?: Record<string, any>
) {
  const payload = {
    message: error instanceof Error ? error.message : String(error),
    stack: error instanceof Error ? error.stack : null,
    context,
    url: location.href,
    ua: navigator.userAgent,
    ts: Date.now()
  }

  try {
    await fetch("/api/telemetry/frontend-error", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload)
    })
  } catch {
    // swallow – telemetry must never crash the app
  }
}
