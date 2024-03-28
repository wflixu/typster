import { invoke } from "@tauri-apps/api";

export interface TypstRenderResponse {
  image: string;
  width: number;
  height: number;
  nonce: number;
}

export interface TypstCompileEvent {
  document: TypstDocument | null;
  diagnostics: TypstSourceDiagnostic[] | null;
}

export interface TypstDocument {
  pages: number;
  hash: string;
  width: number;
  height: number;
}
export interface TypstPage {
  hash: string;
  width: number;
  height: number;
  num: number;
}

export type TypstDiagnosticSeverity = "error" | "warning";

export interface TypstSourceDiagnostic {
  range: { start: number; end: number };
  severity: TypstDiagnosticSeverity;
  message: string;
  hints: string[];
}

export enum TypstCompletionKind {
  Syntax = 1,
  Function = 2,
  Parameter = 3,
  Constant = 4,
  Symbol = 5,
  Type = 6,
}

export interface TypstCompletion {
  kind: TypstCompletionKind;
  label: string;
  apply: string | null;
  detail: string | null;
}

export type IMode = "all" | "edit" | "preview";
export type IAdjust = "full" | "width" | "height";

export interface TypstCompleteResponse {
  offset: number;
  completions: TypstCompletion[];
}

export const autocomplete = (
  path: string,
  content: string,
  offset: number,
  explicit: boolean
): Promise<TypstCompleteResponse> =>
  invoke<TypstCompleteResponse>("typst_autocomplete", {
    path,
    content,
    offset,
    explicit,
  });
