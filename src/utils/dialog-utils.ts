import { ask, save, open } from '@tauri-apps/plugin-dialog';

/**
 * 显示保存更改对话框
 * @param filePath 文件路径
 * @param onSaveCallback 保存回调
 * @param onDiscardCallback 丢弃回调
 */
export async function showSaveChanges(
    filePath: string,
    onSaveCallback: () => Promise<void>,
    onDiscardCallback: () => void
) {
    const fileName = filePath.split('/').pop() || 'file';
    const shouldSave = await ask(
        `文件 "${fileName}" 有未保存的更改，是否要保存？`,
        {
            title: '保存更改',
            kind: 'warning',
        }
    );

    if (shouldSave) {
        await onSaveCallback();
    } else {
        onDiscardCallback();
    }
}

/**
 * 显示文件操作错误对话框
 * @param operation 操作名称
 * @param error 错误信息
 */
export async function showFileError(operation: string, error: any) {
    console.error(`${operation}:`, error);
    await ask(
        `${operation}: ${error?.message || '未知错误'}`,
        {
            title: '操作失败',
            kind: 'error',
        }
    );
}

/**
 * 显示成功消息
 * @param message 消息内容
 */
export async function showSuccess(message: string) {
    await ask(
        message,
        {
            title: '成功',
            kind: 'info',
        }
    );
}