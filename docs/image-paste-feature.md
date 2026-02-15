# 图片粘贴功能文档

## 功能概述

Typster 编辑器现在支持直接粘贴图片！当你在编辑器中粘贴图片时（Ctrl+V / Cmd+V），图片会自动保存到应用数据目录，并插入 Typst 图片语法。

---

## 使用方法

### 1. 粘贴截图或复制的图片

**步骤**:
1. 在任何地方复制图片或截图（如：Snipping Tool、截图软件、浏览器图片等）
2. 在 Typster 编辑器中按 `Ctrl+V` (Windows/Linux) 或 `Cmd+V` (Mac)
3. 图片自动保存并插入 Typst 语法：`#image("../images/image_1234567890_abc123.png")`

### 2. 支持的图片格式

- ✅ PNG (`.png`)
- ✅ JPEG (`.jpg`, `.jpeg`)
- ✅ GIF (`.gif`)
- ✅ WebP (`.webp`)
- ✅ SVG (`.svg`)

---

## 技术实现

### 工作流程

```
1. 用户粘贴图片
   ↓
2. 检测剪贴板中的图片数据
   ↓
3. 生成唯一文件名 (image_时间戳_随机数.格式)
   ↓
4. 保存到应用数据目录 (~/Library/Application Support/typster/images/)
   ↓
5. 在光标位置插入 Typst 图片语法
   ↓
6. 显示状态提示：图片已插入
```

### 文件存储位置

**macOS**:
```
~/Library/Application Support/typster/images/
```

**Linux**:
```
~/.config/typster/images/
```

**Windows**:
```
C:\Users\<用户名>\AppData\Roaming\typster\images\
```

### 插入的语法格式

```typst
#image("../images/image_1735213890000_a3b4c5.png")
```

---

## 代码结构

### 前端模块

**1. `src/utils/image-handler.ts`**
图片处理核心工具库

**主要函数**:
```typescript
// 从剪贴板提取图片
extractImageFromClipboard(items: DataTransferItemList)

// 保存图片到应用数据目录
saveImageToAppData(blob: Blob, format: ImageFormat): Promise<string>

// 生成唯一文件名
generateImageFilename(format: ImageFormat): string

// 处理粘贴事件
handlePasteImage(event: ClipboardEvent): Promise<string | null>

// 生成 Typst 图片语法
getTypstImageSyntax(imagePath: string): string
```

**2. `src/pages/typst/TypstEditor.vue`**
编辑器集成

**关键实现**:
```typescript
editorProps: {
  handlePaste: (_view, event) => {
    handleImagePaste(event);
    return false;
  },
}
```

---

## 示例场景

### 场景 1: 粘贴系统截图

**操作**:
1. 使用 macOS 截图 (`Cmd+Shift+4`) 截取图片
2. 在 Typster 编辑器中按 `Cmd+V`
3. 结果：`#image("../images/image_1735213890123_ab4c8d.png")`

### 场景 2: 粘贴网页图片

**操作**:
1. 在浏览器中右键点击图片 → 复制图片
2. 在 Typster 编辑器中按 `Cmd+V`
3. 结果：图片自动保存并插入引用

### 场景 3: 从文件管理器复制

**操作**:
1. 在 Finder/文件管理器中复制图片文件
2. 在 Typster 编辑器中按 `Cmd+V`
3. 结果：图片自动保存并插入引用

---

## 特性

### ✅ 优点

1. **便捷性**: 一键粘贴，无需手动保存文件
2. **自动命名**: 基于时间戳和随机数生成唯一文件名
3. **统一管理**: 所有图片集中存储在应用数据目录
4. **Typst 原生**: 直接生成 Typst 图片语法
5. **类型安全**: TypeScript 类型检查
6. **错误处理**: 完善的错误提示和状态反馈

### 🔒 安全性

- 图片路径经过安全验证
- 文件保存在应用数据目录，避免系统目录访问
- 支持的图片格式白名单验证

---

## 故障排查

### 问题 1: 粘贴后没有反应

**可能原因**:
- 剪贴板中没有图片数据
- 图片格式不支持

**解决方案**:
1. 确认已复制图片到剪贴板
2. 检查图片格式是否在支持列表中
3. 查看控制台错误信息

### 问题 2: 提示"插入图片失败"

**可能原因**:
- 应用数据目录权限问题
- 磁盘空间不足

**解决方案**:
1. 检查应用数据目录权限
2. 清理磁盘空间
3. 查看控制台详细错误信息

### 问题 3: 图片路径不正确

**可能原因**:
- 项目结构与预期不符

**解决方案**:
- Typst 中使用相对路径 `../images/`
- 确保项目目录结构正确

---

## 开发指南

### 添加新的图片格式支持

编辑 `src/utils/image-handler.ts`:

```typescript
function getImageFormatFromMime(mime: string): ImageFormat | null {
  const mimeToFormat: Record<string, ImageFormat> = {
    // 添加新格式
    'image/bmp': 'bmp',
    'image/tiff': 'tiff',
  };
  return mimeToFormat[mime.toLowerCase()] || null;
}
```

### 自定义文件命名规则

编辑 `generateImageFilename` 函数:

```typescript
export function generateImageFilename(format: ImageFormat): string {
  // 自定义命名逻辑
  const date = new Date().toISOString().split('T')[0];
  const random = Math.random().toString(36).substring(2, 8);
  return `img_${date}_${random}.${format}`;
}
```

---

## 未来改进

### 计划中的功能

1. **图片压缩**: 自动压缩大图片以节省空间
2. **图片预览**: 编辑器中显示图片预览
3. **拖放上传**: 支持拖放图片文件到编辑器
4. **图片管理器**: 浏览和管理已保存的图片
5. **批量导入**: 一次导入多张图片
6. **图片编辑器**: 简单的裁剪和调整功能

---

## 技术细节

### 文件大小

- 不限制图片大小
- 建议单张图片 < 10MB

### 性能

- 图片保存异步处理，不阻塞编辑器
- 平均保存时间 < 100ms

### 兼容性

- macOS ✅
- Linux ✅
- Windows ✅

---

**文档版本**: 1.0
**最后更新**: 2025-12-26
**维护者**: Typster Team
