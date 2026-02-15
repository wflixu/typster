/**
 * 文件路径安全验证工具
 * 防止路径穿越攻击和其他文件系统安全问题
 */

/**
 * 检测路径中是否存在路径穿越攻击模式
 * @param path 要检查的路径
 * @returns 如果路径包含穿越模式返回 true
 */
export function hasPathTraversalPattern(path: string): boolean {
  // 检查常见的路径穿越模式
  const traversalPatterns = [
    '../',       // 向上遍历
    '..\\',      // Windows 风格向上遍历
    '%2e%2e',    // URL 编码的 ..
    '%252e',     // 双重编码的 .
    '..%2f',     // 混合编码
    '..%5c',     // Windows 混合编码
    '%2e%2e%2f', // ../ 的 URL 编码
    '%2e%2e%5c', // ..\ 的 URL 编码
  ];

  const lowerPath = path.toLowerCase();
  return traversalPatterns.some(pattern =>
    lowerPath.includes(pattern.toLowerCase())
  );
}

/**
 * 规范化路径，移除危险模式
 * @param path 要规范的路径
 * @returns 规范化后的安全路径
 * @throws Error 如果路径包含穿越模式
 */
export function sanitizePath(path: string): string {
  if (!path || typeof path !== 'string') {
    throw new Error('Invalid path: path must be a non-empty string');
  }

  // 检查路径穿越
  if (hasPathTraversalPattern(path)) {
    throw new Error(`Path traversal detected: ${path}`);
  }

  // 移除多余的斜杠
  let sanitized = path.replace(/\/+/g, '/').replace(/\\+/g, '/');

  // 移除开头和结尾的斜杠（保留绝对路径的开头斜杠）
  sanitized = sanitized.replace(/^\/+/, '/').replace(/\/+$/, '');

  // 检查是否尝试访问系统目录
  const dangerousPaths = [
    '/etc/',
    '/sys/',
    '/proc/',
    '/dev/',
    '/root/',
    '~/.ssh/',
    '~/.gnupg/',
  ];

  for (const dangerous of dangerousPaths) {
    if (sanitized.includes(dangerous)) {
      throw new Error(`Access to system path is not allowed: ${dangerous}`);
    }
  }

  return sanitized;
}

/**
 * 验证路径是否在允许的根目录范围内
 * @param requestedPath 请求的路径
 * @param allowedRoot 允许的根目录
 * @returns 如果路径在允许范围内返回 true
 */
export function isPathWithinRoot(requestedPath: string, allowedRoot: string): boolean {
  try {
    const sanitized = sanitizePath(requestedPath);

    // 规范化根路径
    const normalizedRoot = allowedRoot.replace(/\/+$/, '');

    // 检查请求的路径是否以根路径开头
    return sanitized.startsWith(normalizedRoot);
  } catch (error) {
    return false;
  }
}

/**
 * 验证文件扩展名是否在允许列表中
 * @param filename 文件名
 * @param allowedExtensions 允许的扩展名列表（不含点号）
 * @returns 如果扩展名允许返回 true
 */
export function isAllowedFileType(filename: string, allowedExtensions: string[]): boolean {
  if (!filename || !filename.includes('.')) {
    return false; // 无扩展名的文件
  }

  const ext = filename.split('.').pop()?.toLowerCase();
  if (!ext) {
    return false;
  }

  return allowedExtensions.includes(ext);
}

/**
 * 验证 Typst 项目文件路径
 * @param filePath 文件路径
 * @param projectPath 项目根路径
 * @returns 如果路径安全返回 true
 */
export function validateTypstFilePath(filePath: string, projectPath: string): boolean {
  // 允许的 Typst 文件类型
  const allowedExtensions = [
    'typ',      // Typst 源文件
    'md',       // Markdown 文件
    'txt',      // 文本文件
    'toml',     // 配置文件
    'json',     // JSON 配置
    'svg',      // SVG 图片
    'png',      // PNG 图片
    'jpg',      // JPEG 图片
    'jpeg',     // JPEG 图片
    'pdf',      // PDF 文件
  ];

  const filename = filePath.split('/').pop() || '';

  try {
    // 1. 检查路径穿越
    if (hasPathTraversalPattern(filePath)) {
      return false;
    }

    // 2. 检查是否在项目范围内
    if (!isPathWithinRoot(filePath, projectPath)) {
      return false;
    }

    // 3. 检查文件类型
    if (!isAllowedFileType(filename, allowedExtensions)) {
      return false;
    }

    return true;
  } catch (error) {
    return false;
  }
}

/**
 * 安全的路径拼接，防止路径穿越
 * @param basePath 基础路径
 * @param relativePath 相对路径
 * @returns 安全的完整路径
 * @throws Error 如果路径不安全
 */
export function safeJoinPath(basePath: string, relativePath: string): string {
  const sanitizedBase = sanitizePath(basePath);
  const sanitizedRelative = sanitizePath(relativePath);

  const joined = `${sanitizedBase}/${sanitizedRelative}`.replace(/\/+/g, '/');

  // 再次检查拼接后的路径
  if (hasPathTraversalPattern(joined)) {
    throw new Error(`Path traversal detected in joined path: ${joined}`);
  }

  return joined;
}
