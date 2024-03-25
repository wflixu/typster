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

export type TypstDiagnosticSeverity = "error" | "warning";

export interface TypstSourceDiagnostic {
  range: { start: number; end: number };
  severity: TypstDiagnosticSeverity;
  message: string;
  hints: string[];
}
