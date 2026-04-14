import { describe, expect, it } from "vitest"
import fs from "node:fs"
import path from "node:path"

const srcRoot = path.resolve(process.cwd(), "src")

function getVueFiles(dir: string): string[] {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const fullPath = path.join(dir, entry.name)
    if (entry.isDirectory()) {
      if (entry.name === "__tests__") return []
      return getVueFiles(fullPath)
    }
    return entry.isFile() && entry.name.endsWith(".vue") ? [fullPath] : []
  })
}

function extractTemplate(source: string, file: string): string {
  const match = source.match(/<template[^>]*>([\s\S]*?)<\/template>/)
  if (!match) {
    throw new Error(`No <template> block found in ${file}`)
  }
  return match[1]
}

function normalizeTemplate(template: string): string {
  return template
    .replace(/<!--[\s\S]*?-->/g, "")
    .split("\n")
    .map((line) => line.trimEnd())
    .join("\n")
    .replace(/\n{3,}/g, "\n\n")
    .trim()
}

const vueFiles = getVueFiles(srcRoot)

describe("vue templates", () => {
  it("keeps the expected vue file inventory", () => {
    expect(vueFiles.map((file) => path.relative(srcRoot, file))).toMatchSnapshot()
  })

  for (const file of vueFiles) {
    const relativePath = path.relative(srcRoot, file)

    it(`matches template snapshot for ${relativePath}`, () => {
      const source = fs.readFileSync(file, "utf8")
      const template = normalizeTemplate(extractTemplate(source, relativePath))

      expect(template.length).toBeGreaterThan(0)
      expect(template).toMatchSnapshot()
    })
  }
})
