/**
 * 图片处理工具
 * 处理粘贴的图片，保存到应用数据目录
 */

import { writeFile, mkdir, BaseDirectory } from '@tauri-apps/plugin-fs';
import { appDataDir } from '@tauri-apps/api/path';

/**
 * 图片格式类型
 */
export type ImageFormat = 'png' | 'jpeg' | 'gif' | 'webp' | 'svg';

/**
 * 从 MIME 类型获取图片格式
 */
function getImageFormatFromMime(mime: string): ImageFormat | null {
  const mimeToFormat: Record<string, ImageFormat> = {
    'image/png': 'png',
    'image/jpeg': 'jpeg',
    'image/jpg': 'jpeg',
    'image/gif': 'gif',
    'image/webp': 'webp',
    'image/svg+xml': 'svg',
  };

  return mimeToFormat[mime.toLowerCase()] || null;
}

/**
 * 生成唯一的图片文件名
 */
export function generateImageFilename(format: ImageFormat): string {
  const timestamp = Date.now();
  const random = Math.random().toString(36).substring(2, 8);
  return `image_${timestamp}_${random}.${format}`;
}

/**
 * 从 Blob 获取图片格式
 */
async function getImageFormat(blob: Blob): Promise<ImageFormat | null> {
  return getImageFormatFromMime(blob.type);
}

/**
 * 从 clipboard items 提取图片
 */
export async function extractImageFromClipboard(
  items: DataTransferItemList
): Promise<{ blob: Blob; format: ImageFormat } | null> {
  // 查找图片类型的项
  for (const item of Array.from(items)) {
    if (item.type.startsWith('image/')) {
      // 获取 Blob 对象
      const blob = item.getAsFile();
      const format = getImageFormatFromMime(item.type);

      if (blob && format) {
        return { blob, format };
      }
    }
  }

  return null;
}

/**
 * 保存图片到应用数据目录
 * @param blob 图片 Blob 对象
 * @param format 图片格式
 * @returns 保存的相对路径（相对于项目目录）
 */
export async function saveImageToAppData(
  blob: Blob,
  format: ImageFormat
): Promise<string> {
  try {
    // 获取应用数据目录
    const dataDir = await appDataDir();

    // 创建 images 子目录（确保路径以斜杠结尾）
    const imagesDir = `${dataDir.replace(/\/$/, '')}/images`;

    try {
      // 创建目录（如果不存在）
      await mkdir(imagesDir, { recursive: true, baseDir: BaseDirectory.AppData });
    } catch (error) {
      // 目录可能已存在，忽略错误
      console.warn('创建目录警告:', error);
    }

    // 生成唯一文件名
    const filename = generateImageFilename(format);

    // 将 Blob 转换为 ArrayBuffer
    const arrayBuffer = await blob.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    // 保存文件（路径相对于 AppData 目录）
    await writeFile(`images/${filename}`, uint8Array, { baseDir: BaseDirectory.AppData });

    // 返回相对路径供 Typst 使用
    return `../images/${filename}`;
  } catch (error) {
    console.error('保存图片失败:', error);
    throw new Error(`保存图片失败: ${error}`);
  }
}

/**
 * 从粘贴事件处理图片
 * @param event 粘贴事件
 * @returns 图片相对路径或 null
 */
export async function handlePasteImage(
  event: ClipboardEvent
): Promise<string | null> {
  const items = event.clipboardData?.items;
  if (!items) {
    return null;
  }

  try {
    // 提取图片
    const imageData = await extractImageFromClipboard(items);
    if (!imageData) {
      return null;
    }

    // 保存图片
    const relativePath = await saveImageToAppData(
      imageData.blob,
      imageData.format
    );

    return relativePath;
  } catch (error) {
    console.error('处理粘贴图片失败:', error);
    throw error;
  }
}

/**
 * 获取 Typst 图片引用语法
 * @param imagePath 图片相对路径
 * @returns Typst 图片语法字符串
 */
export function getTypstImageSyntax(imagePath: string): string {
  return `#image("${imagePath}")`;
}
