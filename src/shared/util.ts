

export function relativePath(root: string, cur: string) {
  return cur.replace(root, "");
}
