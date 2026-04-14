import { describe, it, expect } from "vitest";
import { required, email } from "@/utils/validators";

describe("validators", () => {
  describe("required()", () => {
    it("accepts non-empty values", () => {
      expect(required("hello")).toBe(true);
      expect(required("0")).toBe(true);
      expect(required(1)).toBe(true);
      expect(required([])).toBe(true);
      expect(required({})).toBe(true);
    });

    it("rejects empty values", () => {
      expect(required("")).toBe("Required");
      expect(required(null)).toBe("Required");
      expect(required(undefined)).toBe("Required");
      expect(required(false)).toBe("Required");
      expect(required(0)).toBe("Required");
    });
  });

  describe("email()", () => {
    it("accepts valid email addresses", () => {
      const valids = [
        "test@example.com",
        "john.doe@domain.org",
        "john+tag@gmail.com",
        "a@b.co",
        "user_name@sub.domain.net",
        "user-name@domain.io",
      ];

      for (const emailAddr of valids) {
        expect(email(emailAddr)).toBe(true);
      }
    });

    it("rejects invalid email addresses", () => {
      const invalids = [
        "",
        "plainaddress",
        "missingatsign.com",
        "@nouser.com",
        "user@.com",
        "user@com",
        "user@domain.",
        "user@domain..com",
        "user@ domain.com",
        "user@domain,com",
      ];

      for (const emailAddr of invalids) {
        expect(email(emailAddr)).toBe("Invalid e-mail.");
      }
    });
  });
});
