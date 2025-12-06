export interface IEditingInfo {
    wordCount: number;
    charCount: number;
    cursorLine: number;
    cursorCol: number;
}
export type SidebarType = 'file' | 'toc'