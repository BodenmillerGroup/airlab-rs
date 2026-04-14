import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import InfoView from "@/components/InfoView.vue";

const globalStubs = {
  "v-list": { template: "<ul><slot /></ul>" },
  "v-list-item": { template: "<li><slot /></li>" },
  "v-list-item-title": { template: "<h4><slot /></h4>" },
  "v-list-item-subtitle": { template: "<p><slot /></p>" },
};

describe("InfoView", () => {
  it("renders nothing when item.meta is missing", () => {
    const wrapper = mount(InfoView, {
      props: { item: {} },
      global: { stubs: globalStubs },
    });

    expect(wrapper.findAll("li").length).toBe(0);
  });

  it("renders meta entries", () => {
    const wrapper = mount(InfoView, {
      props: {
        item: {
          meta: {
            Author: "Lars",
            Version: "1.0",
          },
        },
      },
      global: { stubs: globalStubs },
    });

    const items = wrapper.findAll("li");
    expect(items.length).toBe(2);

    expect(wrapper.text()).toContain("Author");
    expect(wrapper.text()).toContain("Lars");
    expect(wrapper.text()).toContain("Version");
    expect(wrapper.text()).toContain("1.0");
  });

  it("renders links when value starts with http", () => {
    const url = "http://example.com";

    const wrapper = mount(InfoView, {
      props: {
        item: {
          meta: {
            Website: url,
          },
        },
      },
      global: { stubs: globalStubs },
    });

    const link = wrapper.find("a");

    expect(link.exists()).toBe(true);
    expect(link.text()).toBe(url);
    expect(link.attributes("href")).toBe(url);
    expect(link.attributes("target")).toBe("_blank");
  });

  it("renders plain text when value is not a url", () => {
    const wrapper = mount(InfoView, {
      props: {
        item: {
          meta: {
            Description: "Some text",
          },
        },
      },
      global: { stubs: globalStubs },
    });

    expect(wrapper.find("a").exists()).toBe(false);
    expect(wrapper.text()).toContain("Some text");
  });
});
