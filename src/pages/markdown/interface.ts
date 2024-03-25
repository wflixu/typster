export interface MdAstNode {
  type: "root" | "heading" | "paragraph" | "image";
  children: MdAstNode[];
  url?: string;
}
